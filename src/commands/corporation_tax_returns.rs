//! Corporation tax return commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum CorporationTaxReturnCommands {
    /// List corporation tax returns
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,

        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },

    /// Get a corporation tax return by period end date
    Get {
        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,
    },

    /// Mark a corporation tax return as filed
    MarkAsFiled {
        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,
    },

    /// Mark a corporation tax return as unfiled
    MarkAsUnfiled {
        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,
    },

    /// Mark a corporation tax return as paid
    MarkAsPaid {
        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,
    },

    /// Mark a corporation tax return as unpaid
    MarkAsUnpaid {
        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,
    },
}

impl CorporationTaxReturnCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("corporation_tax_returns", params).await?;
                print_output(&result, format);
            }
            Self::Get { period_ends_on } => {
                let result = client
                    .get(&format!("corporation_tax_returns/{}", period_ends_on), None)
                    .await?;
                print_output(&result, format);
            }
            Self::MarkAsFiled { period_ends_on } => {
                let result = client
                    .put(
                        &format!(
                            "corporation_tax_returns/{}/mark_as_filed",
                            period_ends_on
                        ),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
            Self::MarkAsUnfiled { period_ends_on } => {
                let result = client
                    .put(
                        &format!(
                            "corporation_tax_returns/{}/mark_as_unfiled",
                            period_ends_on
                        ),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
            Self::MarkAsPaid { period_ends_on } => {
                let result = client
                    .put(
                        &format!("corporation_tax_returns/{}/mark_as_paid", period_ends_on),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
            Self::MarkAsUnpaid { period_ends_on } => {
                let result = client
                    .put(
                        &format!("corporation_tax_returns/{}/mark_as_unpaid", period_ends_on),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
