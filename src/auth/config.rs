//! OAuth2 configuration with embedded credentials.
//!
//! The client ID and secret can be embedded at compile time via environment variables,
//! or provided at runtime via environment variables.

use anyhow::{anyhow, Result};

const CLIENT_ID_PLACEHOLDER: &str = "YOUR_CLIENT_ID";
const CLIENT_SECRET_PLACEHOLDER: &str = "YOUR_CLIENT_SECRET";

/// FreeAgent OAuth2 Client ID
/// Set FREEAGENT_CLIENT_ID at compile time to embed, or configure at runtime
pub const CLIENT_ID: &str = match option_env!("FREEAGENT_CLIENT_ID") {
    Some(id) => id,
    None => CLIENT_ID_PLACEHOLDER,
};

/// FreeAgent OAuth2 Client Secret
/// Set FREEAGENT_CLIENT_SECRET at compile time to embed, or configure at runtime
pub const CLIENT_SECRET: &str = match option_env!("FREEAGENT_CLIENT_SECRET") {
    Some(secret) => secret,
    None => CLIENT_SECRET_PLACEHOLDER,
};

/// Resolve the OAuth2 Client ID with runtime env override.
pub fn client_id() -> Result<String> {
    if let Ok(value) = std::env::var("FREEAGENT_CLIENT_ID") {
        let trimmed = value.trim();
        if !trimmed.is_empty() && trimmed != CLIENT_ID_PLACEHOLDER {
            return Ok(trimmed.to_string());
        }
    }

    if CLIENT_ID != CLIENT_ID_PLACEHOLDER {
        return Ok(CLIENT_ID.to_string());
    }

    Err(anyhow!(
        "FreeAgent client ID is not configured. Set FREEAGENT_CLIENT_ID or build with FREEAGENT_CLIENT_ID embedded."
    ))
}

/// Resolve the OAuth2 Client Secret with runtime env override.
pub fn client_secret() -> Result<String> {
    if let Ok(value) = std::env::var("FREEAGENT_CLIENT_SECRET") {
        let trimmed = value.trim();
        if !trimmed.is_empty() && trimmed != CLIENT_SECRET_PLACEHOLDER {
            return Ok(trimmed.to_string());
        }
    }

    if CLIENT_SECRET != CLIENT_SECRET_PLACEHOLDER {
        return Ok(CLIENT_SECRET.to_string());
    }

    Err(anyhow!(
        "FreeAgent client secret is not configured. Set FREEAGENT_CLIENT_SECRET or build with FREEAGENT_CLIENT_SECRET embedded."
    ))
}

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

/// OAuth redirect URI
pub fn redirect_uri(port: u16) -> String {
    format!("http://localhost:{}/callback", port)
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
