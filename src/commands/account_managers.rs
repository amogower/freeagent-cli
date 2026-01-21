//! Account manager commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum AccountManagerCommands {
    /// List all account managers
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,

        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },

    /// Get an account manager by ID
    Get {
        /// Account manager ID
        id: String,
    },
}

impl AccountManagerCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("account_managers", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("account_managers/{}", id), None).await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
