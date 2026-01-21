//! Email address commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::FreeAgentClient;
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum EmailAddressCommands {
    /// List all email addresses
    List,
}

impl EmailAddressCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List => {
                let result = client.get("email_addresses", None).await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
