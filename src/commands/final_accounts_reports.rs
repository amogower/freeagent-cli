//! Final accounts report commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum FinalAccountsReportCommands {
    /// List final accounts reports
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,

        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },

    /// Get a final accounts report by period end date
    Get {
        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,
    },

    /// Mark a final accounts report as filed
    MarkAsFiled {
        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,
    },

    /// Mark a final accounts report as unfiled
    MarkAsUnfiled {
        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,
    },
}

impl FinalAccountsReportCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("final_accounts_reports", params).await?;
                print_output(&result, format);
            }
            Self::Get { period_ends_on } => {
                let result = client
                    .get(&format!("final_accounts_reports/{}", period_ends_on), None)
                    .await?;
                print_output(&result, format);
            }
            Self::MarkAsFiled { period_ends_on } => {
                let result = client
                    .put(
                        &format!("final_accounts_reports/{}/mark_as_filed", period_ends_on),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
            Self::MarkAsUnfiled { period_ends_on } => {
                let result = client
                    .put(
                        &format!("final_accounts_reports/{}/mark_as_unfiled", period_ends_on),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
