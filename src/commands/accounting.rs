//! Accounting report commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum AccountingCommands {
    /// Get balance sheet
    BalanceSheet {
        /// Date for the balance sheet (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
    },
    
    /// Get profit and loss summary
    ProfitAndLoss {
        /// From date (YYYY-MM-DD)
        #[arg(long)]
        from_date: Option<String>,
        
        /// To date (YYYY-MM-DD)
        #[arg(long)]
        to_date: Option<String>,
    },
    
    /// Get trial balance
    TrialBalance {
        /// Date for the trial balance (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
    },
    
    /// Get cashflow report
    Cashflow {
        /// From date (YYYY-MM-DD)
        #[arg(long)]
        from_date: Option<String>,
        
        /// To date (YYYY-MM-DD)
        #[arg(long)]
        to_date: Option<String>,
    },
    
    /// Get general ledger
    GeneralLedger {
        /// From date (YYYY-MM-DD)
        #[arg(long)]
        from_date: Option<String>,
        
        /// To date (YYYY-MM-DD)
        #[arg(long)]
        to_date: Option<String>,
    },
    
    /// Get account transactions
    AccountTransactions {
        /// Category nominal code
        #[arg(long)]
        category: String,
        
        /// From date (YYYY-MM-DD)
        #[arg(long)]
        from_date: Option<String>,
        
        /// To date (YYYY-MM-DD)
        #[arg(long)]
        to_date: Option<String>,
    },

    /// List all accounting transactions
    Transactions,

    /// Get an accounting transaction by ID
    Transaction {
        /// Transaction ID
        id: String,
    },
}

impl AccountingCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::BalanceSheet { date } => {
                let params = QueryBuilder::new()
                    .add("date", date.clone())
                    .build();
                let result = client.get("accounting/balance_sheet", params).await?;
                print_output(&result, format);
            }
            Self::ProfitAndLoss { from_date, to_date } => {
                let params = QueryBuilder::new()
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .build();
                let result = client.get("accounting/profit_and_loss_summary", params).await?;
                print_output(&result, format);
            }
            Self::TrialBalance { date } => {
                let params = QueryBuilder::new()
                    .add("date", date.clone())
                    .build();
                let result = client.get("accounting/trial_balance", params).await?;
                print_output(&result, format);
            }
            Self::Cashflow { from_date, to_date } => {
                let params = QueryBuilder::new()
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .build();
                let result = client.get("accounting/cashflow", params).await?;
                print_output(&result, format);
            }
            Self::GeneralLedger { from_date, to_date } => {
                let params = QueryBuilder::new()
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .build();
                let result = client.get("accounting/general_ledger", params).await?;
                print_output(&result, format);
            }
            Self::AccountTransactions { category, from_date, to_date } => {
                let params = QueryBuilder::new()
                    .add("category", Some(category.clone()))
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .build();
                let result = client.get("accounting/account_transactions", params).await?;
                print_output(&result, format);
            }
            Self::Transactions => {
                let result = client.get("accounting/transactions", None).await?;
                print_output(&result, format);
            }
            Self::Transaction { id } => {
                let result = client.get(&format!("accounting/transactions/{}", id), None).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
