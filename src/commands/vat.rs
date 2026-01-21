//! VAT return commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum VatCommands {
    /// List all VAT returns
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get a VAT return by ID
    Get {
        /// VAT return ID
        id: String,
    },
    
    /// Mark VAT return as filed
    MarkAsFiled {
        /// VAT return ID
        id: String,
    },
    
    /// Mark VAT return as unfiled
    MarkAsUnfiled {
        /// VAT return ID
        id: String,
    },

    /// Mark a VAT return payment as paid
    MarkPaymentAsPaid {
        /// VAT return period end date (YYYY-MM-DD)
        period_ends_on: String,

        /// Payment date (YYYY-MM-DD)
        payment_date: String,
    },

    /// Mark a VAT return payment as unpaid
    MarkPaymentAsUnpaid {
        /// VAT return period end date (YYYY-MM-DD)
        period_ends_on: String,

        /// Payment date (YYYY-MM-DD)
        payment_date: String,
    },
}

impl VatCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("vat_returns", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("vat_returns/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::MarkAsFiled { id } => {
                let result = client.put(&format!("vat_returns/{}/transitions/mark_as_filed", id), None::<()>).await?;
                print_output(&result, format);
            }
            Self::MarkAsUnfiled { id } => {
                let result = client.put(&format!("vat_returns/{}/transitions/mark_as_unfiled", id), None::<()>).await?;
                print_output(&result, format);
            }
            Self::MarkPaymentAsPaid { period_ends_on, payment_date } => {
                let result = client
                    .put(
                        &format!(
                            "vat_returns/{}/payments/{}/mark_as_paid",
                            period_ends_on, payment_date
                        ),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
            Self::MarkPaymentAsUnpaid { period_ends_on, payment_date } => {
                let result = client
                    .put(
                        &format!(
                            "vat_returns/{}/payments/{}/mark_as_unpaid",
                            period_ends_on, payment_date
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
