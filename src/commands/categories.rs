//! Category commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::FreeAgentClient;
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum CategoryCommands {
    /// List all categories
    List,
    
    /// Get a category by nominal code
    Get {
        /// Category nominal code
        nominal_code: String,
    },
}

impl CategoryCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List => {
                let result = client.get("categories", None).await?;
                print_output(&result, format);
            }
            Self::Get { nominal_code } => {
                let result = client.get(&format!("categories/{}", nominal_code), None).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
