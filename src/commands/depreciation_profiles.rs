//! Depreciation profile commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::FreeAgentClient;
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum DepreciationProfileCommands {
    /// List all depreciation profiles
    List,
}

impl DepreciationProfileCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List => {
                let result = client.get("depreciation_profiles", None).await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
