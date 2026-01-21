//! Payroll commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::FreeAgentClient;
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum PayrollCommands {
    /// List payroll for a year
    ListYear {
        /// Payroll year
        year: String,
    },

    /// Get payroll for a period
    GetPeriod {
        /// Payroll year
        year: String,

        /// Payroll period
        period: String,
    },

    /// Mark payroll payment as paid
    MarkPaymentAsPaid {
        /// Payroll year
        year: String,

        /// Payment date (YYYY-MM-DD)
        payment_date: String,
    },

    /// Mark payroll payment as unpaid
    MarkPaymentAsUnpaid {
        /// Payroll year
        year: String,

        /// Payment date (YYYY-MM-DD)
        payment_date: String,
    },
}

impl PayrollCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::ListYear { year } => {
                let result = client.get(&format!("payroll/{}", year), None).await?;
                print_output(&result, format);
            }
            Self::GetPeriod { year, period } => {
                let result = client.get(&format!("payroll/{}/{}", year, period), None).await?;
                print_output(&result, format);
            }
            Self::MarkPaymentAsPaid { year, payment_date } => {
                let result = client
                    .put(
                        &format!("payroll/{}/payments/{}/mark_as_paid", year, payment_date),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
            Self::MarkPaymentAsUnpaid { year, payment_date } => {
                let result = client
                    .put(
                        &format!("payroll/{}/payments/{}/mark_as_unpaid", year, payment_date),
                        None::<()>,
                    )
                    .await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}
