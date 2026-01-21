//! Bank transaction commands.

use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Clone, ValueEnum)]
pub enum BankTransactionView {
    All,
    Unexplained,
    Explained,
    ManuallyAdded,
    Imported,
}

impl BankTransactionView {
    fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Unexplained => "unexplained",
            Self::Explained => "explained",
            Self::ManuallyAdded => "manually_added",
            Self::Imported => "imported",
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum BankTransactionCommands {
    /// List bank transactions
    List {
        /// Bank account URL (required)
        #[arg(long)]
        bank_account: String,
        
        /// Filter by view
        #[arg(long)]
        view: Option<BankTransactionView>,
        
        /// Filter from date (YYYY-MM-DD)
        #[arg(long)]
        from_date: Option<String>,
        
        /// Filter to date (YYYY-MM-DD)
        #[arg(long)]
        to_date: Option<String>,
        
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get a bank transaction by ID
    Get {
        /// Bank transaction ID
        id: String,
    },
    
    /// Create a new bank transaction
    Create {
        /// Bank account URL (required)
        #[arg(long)]
        bank_account: String,
        
        /// Transaction date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: String,
        
        /// Amount (positive for credit, negative for debit)
        #[arg(long)]
        amount: String,
        
        /// Description
        #[arg(long)]
        description: Option<String>,
    },
    
    /// Update a bank transaction
    Update {
        /// Bank transaction ID
        id: String,
        
        /// Transaction date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,
        
        /// Amount
        #[arg(long)]
        amount: Option<String>,
        
        /// Description
        #[arg(long)]
        description: Option<String>,
    },
    
    /// Delete a bank transaction
    Delete {
        /// Bank transaction ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl BankTransactionCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List {
                bank_account,
                view,
                from_date,
                to_date,
                page,
                per_page,
            } => {
                let params = QueryBuilder::new()
                    .add("bank_account", Some(bank_account.clone()))
                    .add("view", view.as_ref().map(|v| v.as_str()))
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("bank_transactions", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("bank_transactions/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create { bank_account, dated_on, amount, description } => {
                let mut transaction = serde_json::Map::new();
                transaction.insert("bank_account".to_string(), json!(bank_account));
                transaction.insert("dated_on".to_string(), json!(dated_on));
                transaction.insert("amount".to_string(), json!(amount));
                
                if let Some(v) = description {
                    transaction.insert("description".to_string(), json!(v));
                }
                
                let body = json!({ "bank_transaction": transaction });
                let result = client.post("bank_transactions", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, dated_on, amount, description } => {
                let mut transaction = serde_json::Map::new();
                
                if let Some(v) = dated_on {
                    transaction.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = amount {
                    transaction.insert("amount".to_string(), json!(v));
                }
                if let Some(v) = description {
                    transaction.insert("description".to_string(), json!(v));
                }
                
                let body = json!({ "bank_transaction": transaction });
                let result = client.put(&format!("bank_transactions/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("bank_transactions/{}", id)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bank_transaction_view_as_str_matches_api() {
        assert_eq!(BankTransactionView::All.as_str(), "all");
        assert_eq!(BankTransactionView::ManuallyAdded.as_str(), "manually_added");
        assert_eq!(BankTransactionView::Imported.as_str(), "imported");
    }
}
