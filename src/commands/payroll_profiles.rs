//! Payroll profile commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::FreeAgentClient;
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum PayrollProfileCommands {
    /// List payroll profiles for a year
    ListYear {
        /// Payroll year
        year: String,
    },
}

impl PayrollProfileCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::ListYear { year } => {
                let result = client.get(&format!("payroll_profiles/{}", year), None).await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
