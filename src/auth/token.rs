//! Token storage and management.
//!
//! Tokens are stored in a JSON file in the user's config directory.
//! The file permissions are set to be readable only by the owner.

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Stored OAuth tokens with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredTokens {
    /// OAuth2 access token
    pub access_token: String,
    /// OAuth2 refresh token
    pub refresh_token: String,
    /// Token expiration time
    pub expires_at: DateTime<Utc>,
    /// Whether this is for sandbox environment
    pub sandbox: bool,
    /// When the tokens were last refreshed
    pub last_refreshed: DateTime<Utc>,
}

impl StoredTokens {
    /// Create new stored tokens
    pub fn new(
        access_token: String,
        refresh_token: String,
        expires_in_seconds: i64,
        sandbox: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            access_token,
            refresh_token,
            expires_at: now + Duration::seconds(expires_in_seconds),
            sandbox,
            last_refreshed: now,
        }
    }

    /// Check if the access token is expired or about to expire
    pub fn is_expired(&self) -> bool {
        // Consider expired if less than 5 minutes remaining
        Utc::now() + Duration::minutes(5) >= self.expires_at
    }

    /// Update tokens after refresh
    pub fn update(&mut self, access_token: String, refresh_token: Option<String>, expires_in_seconds: i64) {
        self.access_token = access_token;
        if let Some(rt) = refresh_token {
            self.refresh_token = rt;
        }
        self.expires_at = Utc::now() + Duration::seconds(expires_in_seconds);
        self.last_refreshed = Utc::now();
    }
}

/// Token storage manager
pub struct TokenStorage {
    config_dir: PathBuf,
    token_file: PathBuf,
}

impl TokenStorage {
    /// Create a new token storage instance
    pub fn new() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "freeagent", "freeagent-cli")
            .context("Failed to determine config directory")?;
        
        let config_dir = project_dirs.config_dir().to_path_buf();
        let token_file = config_dir.join("tokens.json");
        
        Ok(Self {
            config_dir,
            token_file,
        })
    }

    /// Ensure the config directory exists with proper permissions
    fn ensure_config_dir(&self) -> Result<()> {
        if !self.config_dir.exists() {
            fs::create_dir_all(&self.config_dir)
                .context("Failed to create config directory")?;
            
            #[cfg(unix)]
            {
                let mut perms = fs::metadata(&self.config_dir)?.permissions();
                perms.set_mode(0o700);
                fs::set_permissions(&self.config_dir, perms)?;
            }
        }
        Ok(())
    }

    /// Save tokens to storage
    pub fn save(&self, tokens: &StoredTokens) -> Result<()> {
        self.ensure_config_dir()?;
        
        let json = serde_json::to_string_pretty(tokens)
            .context("Failed to serialize tokens")?;
        
        fs::write(&self.token_file, &json)
            .context("Failed to write token file")?;
        
        #[cfg(unix)]
        {
            let mut perms = fs::metadata(&self.token_file)?.permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&self.token_file, perms)?;
        }
        
        Ok(())
    }

    /// Load tokens from storage
    pub fn load(&self) -> Result<Option<StoredTokens>> {
        if !self.token_file.exists() {
            return Ok(None);
        }
        
        let json = fs::read_to_string(&self.token_file)
            .context("Failed to read token file")?;
        
        let tokens: StoredTokens = serde_json::from_str(&json)
            .context("Failed to parse token file")?;
        
        Ok(Some(tokens))
    }

    /// Delete stored tokens (logout)
    pub fn delete(&self) -> Result<()> {
        if self.token_file.exists() {
            fs::remove_file(&self.token_file)
                .context("Failed to delete token file")?;
        }
        Ok(())
    }

    /// Get the path to the token file (for display purposes)
    pub fn token_file_path(&self) -> &PathBuf {
        &self.token_file
    }
}

impl Default for TokenStorage {
    fn default() -> Self {
        Self::new().expect("Failed to initialize token storage")
    }
}
