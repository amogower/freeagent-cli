//! Currency commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::FreeAgentClient;
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum CurrencyCommands {
    /// List all currencies
    List,
}

impl CurrencyCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List => {
                let result = client.get("currencies", None).await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
