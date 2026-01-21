//! Practice commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::FreeAgentClient;
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum PracticeCommands {
    /// Get practice details
    Get,
}

impl PracticeCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::Get => {
                let result = client.get("practice", None).await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
