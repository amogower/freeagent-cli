//! OAuth2 authentication flow implementation.
//!
//! Implements the Authorization Code flow with PKCE for enhanced security.

use anyhow::{anyhow, Context, Result};

use oauth2::{
    basic::BasicClient, AuthType, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, ErrorResponse, PkceCodeChallenge, RedirectUrl, RefreshToken,
    RequestTokenError, TokenResponse, TokenUrl,
};



use std::net::TcpListener;
use tiny_http::{Response, Server};
use url::Url;

use super::config;
use super::token::{StoredTokens, TokenStorage};

/// OAuth2 authentication manager
pub struct OAuthManager {
    sandbox: bool,
    storage: TokenStorage,
}

impl OAuthManager {
    /// Create a new OAuth manager
    pub fn new(sandbox: bool) -> Result<Self> {
        Ok(Self {
            sandbox,
            storage: TokenStorage::new()?,
        })
    }

    /// Create the OAuth2 client
    fn create_client(&self, redirect_uri: Option<&str>) -> Result<BasicClient> {
        let client_id = config::client_id()?;
        let client_secret = config::client_secret()?;
        let mut client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(config::auth_url(self.sandbox).to_string())
                .context("Invalid auth URL")?,
            Some(TokenUrl::new(config::token_url(self.sandbox).to_string())
                .context("Invalid token URL")?),
        )
        .set_auth_type(AuthType::RequestBody);

        if let Some(uri) = redirect_uri {
            client = client.set_redirect_uri(
                RedirectUrl::new(uri.to_string()).context("Invalid redirect URL")?,
            );
        }

        Ok(client)
    }

    /// Start local server to receive callback on an available port
    fn start_callback_server(&self) -> Result<(Server, u16)> {
        let mut last_err = None;

        for _ in 0..10 {
            let listener = TcpListener::bind("127.0.0.1:0")
                .map_err(|e| anyhow!("Failed to bind callback listener: {}", e))?;
            let port = listener
                .local_addr()
                .map_err(|e| anyhow!("Failed to determine callback port: {}", e))?
                .port();
            drop(listener);

            match Server::http(format!("127.0.0.1:{}", port)) {
                Ok(server) => return Ok((server, port)),
                Err(err) => last_err = Some(err),
            }
        }

        if let Some(err) = last_err {
            Err(anyhow!(
                "Failed to start callback server on an available port: {}",
                err
            ))
        } else {
            Err(anyhow!(
                "Failed to start callback server on an available port"
            ))
        }
    }

    /// Start the OAuth login flow
    pub async fn login(&self) -> Result<StoredTokens> {
        let (server, callback_port) = self.start_callback_server()?;
        let redirect_uri = config::redirect_uri(callback_port);
        let client = self.create_client(Some(&redirect_uri))?;

        let use_pkce = std::env::var("FREEAGENT_OAUTH_PKCE")
            .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
            .unwrap_or(false);

        let (auth_url, csrf_token, pkce_verifier) = if use_pkce {
            let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
            let (auth_url, csrf_token) = client
                .authorize_url(CsrfToken::new_random)
                .set_pkce_challenge(pkce_challenge)
                .url();
            (auth_url, csrf_token, Some(pkce_verifier))
        } else {
            let (auth_url, csrf_token) = client.authorize_url(CsrfToken::new_random).url();
            (auth_url, csrf_token, None)
        };

        println!("Opening browser for authentication...");
        println!();
        println!("If the browser doesn't open automatically, visit:");
        println!("{}", auth_url);
        println!();

        // Try to open browser
        if webbrowser::open(auth_url.as_str()).is_err() {
            println!("Could not open browser automatically.");
        }

        // Start local server to receive callback
        let auth_code = self.wait_for_callback(server, callback_port, &csrf_token)?;

        println!("Authorization code received. Exchanging for tokens...");

        // Exchange code for tokens
        let token_request = client.exchange_code(AuthorizationCode::new(auth_code));
        let token_request = if let Some(pkce_verifier) = pkce_verifier {
            token_request.set_pkce_verifier(pkce_verifier)
        } else {
            token_request
        };

        let token_result = token_request
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|err| format_token_error("authorization code exchange", err))?;

        let access_token = token_result.access_token().secret().clone();
        let refresh_token = token_result
            .refresh_token()
            .map(|t| t.secret().clone())
            .ok_or_else(|| anyhow!("No refresh token received"))?;
        let expires_in = token_result
            .expires_in()
            .map(|d| d.as_secs() as i64)
            .unwrap_or(3600);

        let tokens = StoredTokens::new(access_token, refresh_token, expires_in, self.sandbox);

        // Save tokens
        self.storage.save(&tokens)?;

        println!("Successfully authenticated!");
        println!("Tokens saved to: {:?}", self.storage.token_file_path());

        Ok(tokens)
    }

    /// Wait for OAuth callback on local server
    fn wait_for_callback(
        &self,
        server: Server,
        callback_port: u16,
        expected_state: &CsrfToken,
    ) -> Result<String> {
        println!(
            "Waiting for authorization callback on port {}...",
            callback_port
        );

        // Wait for the callback request
        let request = server
            .recv()
            .map_err(|e| anyhow!("Failed to receive callback: {}", e))?;

        let url = format!("http://localhost{}", request.url());
        let parsed_url = Url::parse(&url).context("Failed to parse callback URL")?;

        let mut code = None;
        let mut state = None;
        let mut error = None;

        for (key, value) in parsed_url.query_pairs() {
            match key.as_ref() {
                "code" => code = Some(value.to_string()),
                "state" => state = Some(value.to_string()),
                "error" => error = Some(value.to_string()),
                _ => {}
            }
        }

        // Send response to browser
        let response_html = if error.is_some() {
            r#"<!DOCTYPE html>
<html>
<head><title>Authentication Failed</title></head>
<body style="font-family: sans-serif; text-align: center; padding: 50px;">
<h1 style="color: #e74c3c;">Authentication Failed</h1>
<p>You can close this window.</p>
</body>
</html>"#
        } else {
            r#"<!DOCTYPE html>
<html>
<head><title>Authentication Successful</title></head>
<body style="font-family: sans-serif; text-align: center; padding: 50px;">
<h1 style="color: #27ae60;">Authentication Successful!</h1>
<p>You can close this window and return to the terminal.</p>
</body>
</html>"#
        };

        let response = Response::from_string(response_html)
            .with_header(tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap());
        let _ = request.respond(response);

        // Check for errors
        if let Some(err) = error {
            return Err(anyhow!("OAuth error: {}", err));
        }

        // Verify state
        let received_state = state.ok_or_else(|| anyhow!("No state parameter in callback"))?;
        if &received_state != expected_state.secret() {
            return Err(anyhow!("State mismatch - possible CSRF attack"));
        }

        // Return authorization code
        code.ok_or_else(|| anyhow!("No authorization code in callback"))
    }

    /// Refresh the access token
    pub async fn refresh(&self, tokens: &mut StoredTokens) -> Result<()> {
        let client = self.create_client(None)?;

        let token_result = client
            .exchange_refresh_token(&RefreshToken::new(tokens.refresh_token.clone()))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|err| format_token_error("refresh token exchange", err))?;

        let access_token = token_result.access_token().secret().clone();
        let refresh_token = token_result.refresh_token().map(|t| t.secret().clone());
        let expires_in = token_result
            .expires_in()
            .map(|d| d.as_secs() as i64)
            .unwrap_or(3600);

        tokens.update(access_token, refresh_token, expires_in);
        self.storage.save(tokens)?;

        Ok(())
    }

    /// Get valid tokens, refreshing if necessary
    pub async fn get_valid_tokens(&self) -> Result<StoredTokens> {
        let mut tokens = self
            .storage
            .load()?
            .ok_or_else(|| anyhow!("Not logged in. Run 'freeagent login' first."))?;

        // Check if we need to use sandbox mode consistently
        if tokens.sandbox != self.sandbox {
            return Err(anyhow!(
                "Token was created for {} mode, but {} mode was requested. Run 'freeagent login{}' to authenticate.",
                if tokens.sandbox { "sandbox" } else { "production" },
                if self.sandbox { "sandbox" } else { "production" },
                if self.sandbox { " --sandbox" } else { "" }
            ));
        }

        if tokens.is_expired() {
            println!("Access token expired, refreshing...");
            self.refresh(&mut tokens).await?;
        }

        Ok(tokens)
    }

    /// Logout - delete stored tokens
    pub fn logout(&self) -> Result<()> {
        self.storage.delete()?;
        println!("Successfully logged out.");
        Ok(())
    }

    /// Get token storage reference
    pub fn storage(&self) -> &TokenStorage {
        &self.storage
    }
}

fn format_token_error<E, T>(context: &str, error: RequestTokenError<E, T>) -> anyhow::Error
where
    E: std::error::Error + Send + Sync + 'static,
    T: ErrorResponse + 'static,
{
    match error {
        RequestTokenError::ServerResponse(err) => {
            anyhow!("{} failed: server returned error response: {:?}", context, err)
        }
        RequestTokenError::Request(err) => {
            anyhow!("{} failed: request error: {}", context, err)
        }
        RequestTokenError::Parse(parse_err, body) => {
            let body_text = format_response_body(&body);
            anyhow!(
                "{} failed: could not parse token response: {}. Raw response: {}",
                context,
                parse_err,
                body_text
            )
        }
        RequestTokenError::Other(message) => {
            anyhow!("{} failed: {}", context, message)
        }
    }
}

fn format_response_body(body: &[u8]) -> String {
    if body.is_empty() {
        return "[empty response body]".to_string();
    }

    const MAX_LEN: usize = 2000;
    let text = String::from_utf8_lossy(body);
    if text.len() <= MAX_LEN {
        text.to_string()
    } else {
        let mut truncated: String = text.chars().take(MAX_LEN).collect();
        truncated.push_str("...(truncated)");
        truncated
    }
}
