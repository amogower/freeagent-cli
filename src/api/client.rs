//! FreeAgent API client with automatic token refresh.

use anyhow::{Context, Result};
use reqwest::{Client, Method, Response};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::auth::{api_url, OAuthManager, StoredTokens};

/// FreeAgent API client
pub struct FreeAgentClient {
    http_client: Client,
    oauth_manager: OAuthManager,
    tokens: Arc<RwLock<StoredTokens>>,
    base_url: String,
}

impl FreeAgentClient {
    /// Create a new API client
    pub async fn new(sandbox: bool) -> Result<Self> {
        let oauth_manager = OAuthManager::new(sandbox)?;
        let tokens = oauth_manager.get_valid_tokens().await?;
        let base_url = api_url(sandbox).to_string();

        let http_client = Client::builder()
            .user_agent("FreeAgent-CLI-Rust/1.0")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            http_client,
            oauth_manager,
            tokens: Arc::new(RwLock::new(tokens)),
            base_url,
        })
    }

    #[cfg(test)]
    pub(crate) fn new_for_test(base_url: String, tokens: StoredTokens) -> Result<Self> {
        let oauth_manager = OAuthManager::new(false)?;

        let http_client = Client::builder()
            .user_agent("FreeAgent-CLI-Rust/1.0")
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            http_client,
            oauth_manager,
            tokens: Arc::new(RwLock::new(tokens)),
            base_url,
        })
    }

    /// Get the current access token, refreshing if necessary
    async fn get_access_token(&self) -> Result<String> {
        let mut tokens = self.tokens.write().await;
        
        if tokens.is_expired() {
            self.oauth_manager.refresh(&mut tokens).await?;
        }
        
        Ok(tokens.access_token.clone())
    }

    /// Build the full URL for an endpoint
    fn build_url(&self, endpoint: &str) -> String {
        format!("{}/{}", self.base_url, endpoint.trim_start_matches('/'))
    }

    /// Make a GET request
    pub async fn get(&self, endpoint: &str, params: Option<HashMap<String, String>>) -> Result<Value> {
        self.request(Method::GET, endpoint, params, None::<()>).await
    }

    /// Make a POST request
    pub async fn post<T: Serialize>(&self, endpoint: &str, body: Option<T>) -> Result<Value> {
        self.request(Method::POST, endpoint, None, body).await
    }

    /// Make a PUT request
    pub async fn put<T: Serialize>(&self, endpoint: &str, body: Option<T>) -> Result<Value> {
        self.request(Method::PUT, endpoint, None, body).await
    }

    /// Make a DELETE request
    pub async fn delete(&self, endpoint: &str) -> Result<Value> {
        self.request(Method::DELETE, endpoint, None, None::<()>).await
    }

    /// Make an HTTP request
    async fn request<T: Serialize>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
        body: Option<T>,
    ) -> Result<Value> {
        let url = self.build_url(endpoint);
        let token = self.get_access_token().await?;

        let mut request = self
            .http_client
            .request(method.clone(), &url)
            .header("Authorization", format!("Bearer {}", token))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");

        if let Some(p) = params {
            request = request.query(&p);
        }

        if let Some(b) = body {
            request = request.json(&b);
        }

        let response = request.send().await.context("Request failed")?;
        
        self.handle_response(response).await
    }

    /// Handle API response
    async fn handle_response(&self, response: Response) -> Result<Value> {
        let status = response.status();
        
        if status.is_success() {
            if status == reqwest::StatusCode::NO_CONTENT {
                return Ok(serde_json::json!({"success": true}));
            }
            
            let text = response.text().await.context("Failed to read response body")?;
            if text.is_empty() {
                return Ok(serde_json::json!({"success": true}));
            }
            
            serde_json::from_str(&text).context("Failed to parse JSON response")
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            
            // Try to parse as JSON for better error messages
            if let Ok(error_json) = serde_json::from_str::<Value>(&error_text) {
                if let Some(errors) = error_json.get("errors") {
                    anyhow::bail!("API Error ({}): {}", status, errors);
                }
            }
            
            anyhow::bail!("API Error ({}): {}", status, error_text)
        }
    }
}

/// Helper to build query parameters
pub struct QueryBuilder {
    params: HashMap<String, String>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    pub fn add<S: Into<String>>(mut self, key: &str, value: Option<S>) -> Self {
        if let Some(v) = value {
            self.params.insert(key.to_string(), v.into());
        }
        self
    }

    pub fn add_bool(mut self, key: &str, value: Option<bool>) -> Self {
        if let Some(v) = value {
            self.params.insert(key.to_string(), v.to_string());
        }
        self
    }

    pub fn add_i32(mut self, key: &str, value: Option<i32>) -> Self {
        if let Some(v) = value {
            self.params.insert(key.to_string(), v.to_string());
        }
        self
    }

    pub fn build(self) -> Option<HashMap<String, String>> {
        if self.params.is_empty() {
            None
        } else {
            Some(self.params)
        }
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::Method::{DELETE, GET};
    use httpmock::MockServer;
    use serde_json::json;
    use std::collections::HashMap;

    fn test_tokens() -> StoredTokens {
        StoredTokens::new(
            "test-access".to_string(),
            "test-refresh".to_string(),
            3600,
            false,
        )
    }

    #[tokio::test]
    async fn query_builder_builds_when_params_present() {
        let params = QueryBuilder::new()
            .add("q", Some("search"))
            .add_bool("active", Some(true))
            .add_i32("page", Some(2))
            .build()
            .expect("params should exist");
        assert_eq!(params.get("q"), Some(&"search".to_string()));
        assert_eq!(params.get("active"), Some(&"true".to_string()));
        assert_eq!(params.get("page"), Some(&"2".to_string()));
    }

    #[tokio::test]
    async fn query_builder_returns_none_when_empty() {
        let params = QueryBuilder::new().build();
        assert!(params.is_none());
    }

    #[tokio::test]
    async fn client_get_includes_query_and_auth_header() -> Result<()> {
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path("/v2/widgets")
                    .query_param("q", "test")
                    .header("Authorization", "Bearer test-access");
                then.status(200).json_body(json!({"ok": true}));
            })
            .await;

        let client = FreeAgentClient::new_for_test(server.url("/v2"), test_tokens())?;
        let params = HashMap::from([("q".to_string(), "test".to_string())]);
        let result = client.get("widgets", Some(params)).await?;
        assert_eq!(result["ok"], true);
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn client_handles_no_content_as_success() -> Result<()> {
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(DELETE).path("/v2/widgets/1");
                then.status(204);
            })
            .await;

        let client = FreeAgentClient::new_for_test(server.url("/v2"), test_tokens())?;
        let result = client.delete("widgets/1").await?;
        assert_eq!(result["success"], true);
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn client_handles_empty_body_as_success() -> Result<()> {
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(GET).path("/v2/empty");
                then.status(200).body("");
            })
            .await;

        let client = FreeAgentClient::new_for_test(server.url("/v2"), test_tokens())?;
        let result = client.get("empty", None).await?;
        assert_eq!(result["success"], true);
        mock.assert_async().await;
        Ok(())
    }

    #[tokio::test]
    async fn client_surfaces_api_errors() -> Result<()> {
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(GET).path("/v2/bad");
                then.status(400)
                    .json_body(json!({"errors": {"message": "bad request"}}));
            })
            .await;

        let client = FreeAgentClient::new_for_test(server.url("/v2"), test_tokens())?;
        let err = client.get("bad", None).await.unwrap_err();
        assert!(err.to_string().contains("API Error"));
        mock.assert_async().await;
        Ok(())
    }
}
