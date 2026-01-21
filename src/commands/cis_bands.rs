//! CIS band commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::FreeAgentClient;
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum CisBandCommands {
    /// List CIS bands
    List,
}

impl CisBandCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List => {
                let result = client.get("cis_bands", None).await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
