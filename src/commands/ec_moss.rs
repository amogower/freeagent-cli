//! EC MOSS commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::FreeAgentClient;
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum EcMossCommands {
    /// List EC MOSS sales tax rates
    SalesTaxRates,
}

impl EcMossCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::SalesTaxRates => {
                let result = client.get("ec_moss/sales_tax_rates", None).await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
