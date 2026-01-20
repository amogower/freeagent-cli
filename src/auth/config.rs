//! OAuth2 configuration with embedded credentials.
//!
//! The client ID and secret can be embedded at compile time via environment variables,
//! or configured at runtime via the login command.

/// FreeAgent OAuth2 Client ID
/// Set FREEAGENT_CLIENT_ID at compile time to embed, or configure at runtime
pub const CLIENT_ID: &str = match option_env!("FREEAGENT_CLIENT_ID") {
    Some(id) => id,
    None => "YOUR_CLIENT_ID",
};

/// FreeAgent OAuth2 Client Secret
/// Set FREEAGENT_CLIENT_SECRET at compile time to embed, or configure at runtime
pub const CLIENT_SECRET: &str = match option_env!("FREEAGENT_CLIENT_SECRET") {
    Some(secret) => secret,
    None => "YOUR_CLIENT_SECRET",
};

/// FreeAgent Production API base URL
pub const PRODUCTION_API_URL: &str = "https://api.freeagent.com/v2";

/// FreeAgent Sandbox API base URL
pub const SANDBOX_API_URL: &str = "https://api.sandbox.freeagent.com/v2";

/// FreeAgent Production OAuth authorization URL
pub const PRODUCTION_AUTH_URL: &str = "https://api.freeagent.com/v2/approve_app";

/// FreeAgent Sandbox OAuth authorization URL
pub const SANDBOX_AUTH_URL: &str = "https://api.sandbox.freeagent.com/v2/approve_app";

/// FreeAgent Production OAuth token URL
pub const PRODUCTION_TOKEN_URL: &str = "https://api.freeagent.com/v2/token_endpoint";

/// FreeAgent Sandbox OAuth token URL
pub const SANDBOX_TOKEN_URL: &str = "https://api.sandbox.freeagent.com/v2/token_endpoint";

/// Local callback port for OAuth redirect
pub const CALLBACK_PORT: u16 = 8484;

/// OAuth redirect URI
pub fn redirect_uri() -> String {
    format!("http://localhost:{}/callback", CALLBACK_PORT)
}

/// Get API URL based on sandbox mode
pub fn api_url(sandbox: bool) -> &'static str {
    if sandbox {
        SANDBOX_API_URL
    } else {
        PRODUCTION_API_URL
    }
}

/// Get auth URL based on sandbox mode
pub fn auth_url(sandbox: bool) -> &'static str {
    if sandbox {
        SANDBOX_AUTH_URL
    } else {
        PRODUCTION_AUTH_URL
    }
}

/// Get token URL based on sandbox mode
pub fn token_url(sandbox: bool) -> &'static str {
    if sandbox {
        SANDBOX_TOKEN_URL
    } else {
        PRODUCTION_TOKEN_URL
    }
}
