//! Income tax return commands (self-assessment returns).

use anyhow::Result;
use clap::Subcommand;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum IncomeTaxReturnCommands {
    /// List self-assessment returns for a user
    List {
        /// User ID
        user_id: String,

        /// Page number
        #[arg(long)]
        page: Option<i32>,

        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },

    /// Get a self-assessment return for a user
    Get {
        /// User ID
        user_id: String,

        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,
    },

    /// Mark a self-assessment return as filed
    MarkAsFiled {
        /// User ID
        user_id: String,

        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,
    },

    /// Mark a self-assessment return as unfiled
    MarkAsUnfiled {
        /// User ID
        user_id: String,

        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,
    },

    /// Mark a self-assessment payment as paid
    MarkPaymentAsPaid {
        /// User ID
        user_id: String,

        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,

        /// Payment date (YYYY-MM-DD)
        payment_date: String,
    },

    /// Mark a self-assessment payment as unpaid
    MarkPaymentAsUnpaid {
        /// User ID
        user_id: String,

        /// Period end date (YYYY-MM-DD)
        period_ends_on: String,

        /// Payment date (YYYY-MM-DD)
        payment_date: String,
    },
}

impl IncomeTaxReturnCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { user_id, page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client
                    .get(&format!("users/{}/self_assessment_returns", user_id), params)
                    .await?;
                print_output(&result, format);
            }
            Self::Get { user_id, period_ends_on } => {
                let result = client
                    .get(
                        &format!(
                            "users/{}/self_assessment_returns/{}",
                            user_id, period_ends_on
                        ),
                        None,
                    )
                    .await?;
                print_output(&result, format);
            }
            Self::MarkAsFiled { user_id, period_ends_on } => {
                let result = client
                    .put(
                        &format!(
                            "users/{}/self_assessment_returns/{}/mark_as_filed",
                            user_id, period_ends_on
                        ),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
            Self::MarkAsUnfiled { user_id, period_ends_on } => {
                let result = client
                    .put(
                        &format!(
                            "users/{}/self_assessment_returns/{}/mark_as_unfiled",
                            user_id, period_ends_on
                        ),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
            Self::MarkPaymentAsPaid { user_id, period_ends_on, payment_date } => {
                let result = client
                    .put(
                        &format!(
                            "users/{}/self_assessment_returns/{}/payments/{}/mark_as_paid",
                            user_id, period_ends_on, payment_date
                        ),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
            Self::MarkPaymentAsUnpaid { user_id, period_ends_on, payment_date } => {
                let result = client
                    .put(
                        &format!(
                            "users/{}/self_assessment_returns/{}/payments/{}/mark_as_unpaid",
                            user_id, period_ends_on, payment_date
                        ),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
