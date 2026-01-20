//! Authentication commands.

use anyhow::Result;
use clap::Subcommand;

use crate::auth::OAuthManager;
use crate::output::{print_info, print_success};

#[derive(Debug, Subcommand)]
pub enum AuthCommands {
    /// Login to FreeAgent using OAuth2
    Login,
    
    /// Logout and delete stored tokens
    Logout,
    
    /// Show current authentication status
    Status,
}

impl AuthCommands {
    pub async fn execute(&self, sandbox: bool) -> Result<()> {
        let oauth = OAuthManager::new(sandbox)?;
        
        match self {
            Self::Login => {
                oauth.login().await?;
            }
            Self::Logout => {
                oauth.logout()?;
            }
            Self::Status => {
                match oauth.storage().load()? {
                    Some(tokens) => {
                        print_success("Logged in");
                        print_info(&format!("Environment: {}", if tokens.sandbox { "Sandbox" } else { "Production" }));
                        print_info(&format!("Token expires: {}", tokens.expires_at));
                        print_info(&format!("Last refreshed: {}", tokens.last_refreshed));
                        if tokens.is_expired() {
                            print_info("Token is expired (will be refreshed on next request)");
                        }
                    }
                    None => {
                        print_info("Not logged in. Run 'freeagent login' to authenticate.");
                    }
                }
            }
        }
        
        Ok(())
    }
}
