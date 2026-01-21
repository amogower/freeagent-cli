//! Hire purchase commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum HirePurchaseCommands {
    /// List hire purchases
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,

        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },

    /// Get a hire purchase by ID
    Get {
        /// Hire purchase ID
        id: String,
    },
}

impl HirePurchaseCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("hire_purchases", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("hire_purchases/{}", id), None).await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
