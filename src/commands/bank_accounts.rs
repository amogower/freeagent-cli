//! Bank account commands.

use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Clone, ValueEnum)]
pub enum BankAccountView {
    Standard,
    CreditCard,
    PaypalAccount,
    All,
}

impl BankAccountView {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::CreditCard => "credit_card",
            Self::PaypalAccount => "paypal_account",
            Self::All => "all",
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum BankAccountType {
    StandardBankAccount,
    CreditCardAccount,
    PaypalAccount,
    LoanAccount,
    MerchantAccount,
}

impl BankAccountType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::StandardBankAccount => "StandardBankAccount",
            Self::CreditCardAccount => "CreditCardAccount",
            Self::PaypalAccount => "PaypalAccount",
            Self::LoanAccount => "LoanAccount",
            Self::MerchantAccount => "MerchantAccount",
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum BankAccountCommands {
    /// List all bank accounts
    List {
        /// Filter by view
        #[arg(long)]
        view: Option<BankAccountView>,
        
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get a bank account by ID
    Get {
        /// Bank account ID
        id: String,
    },
    
    /// Create a new bank account
    Create {
        /// Account type (required)
        #[arg(long, value_enum)]
        account_type: BankAccountType,
        
        /// Account name (required)
        #[arg(long)]
        name: String,
        
        /// Bank name
        #[arg(long)]
        bank_name: Option<String>,
        
        /// Currency code
        #[arg(long)]
        currency: Option<String>,
        
        /// Opening balance
        #[arg(long)]
        opening_balance: Option<String>,
        
        /// Is primary account
        #[arg(long)]
        is_primary: Option<bool>,
    },
    
    /// Update a bank account
    Update {
        /// Bank account ID
        id: String,
        
        /// Account name
        #[arg(long)]
        name: Option<String>,
        
        /// Bank name
        #[arg(long)]
        bank_name: Option<String>,
        
        /// Is primary account
        #[arg(long)]
        is_primary: Option<bool>,
    },
    
    /// Delete a bank account
    Delete {
        /// Bank account ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl BankAccountCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { view, page, per_page } => {
                let params = QueryBuilder::new()
                    .add("view", view.as_ref().map(|v| v.as_str()))
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("bank_accounts", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("bank_accounts/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                account_type,
                name,
                bank_name,
                currency,
                opening_balance,
                is_primary,
            } => {
                let mut account = serde_json::Map::new();
                account.insert("type".to_string(), json!(account_type.as_str()));
                account.insert("name".to_string(), json!(name));
                
                if let Some(v) = bank_name {
                    account.insert("bank_name".to_string(), json!(v));
                }
                if let Some(v) = currency {
                    account.insert("currency".to_string(), json!(v));
                }
                if let Some(v) = opening_balance {
                    account.insert("opening_balance".to_string(), json!(v));
                }
                if let Some(v) = is_primary {
                    account.insert("is_primary".to_string(), json!(v));
                }
                
                let body = json!({ "bank_account": account });
                let result = client.post("bank_accounts", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, name, bank_name, is_primary } => {
                let mut account = serde_json::Map::new();
                
                if let Some(v) = name {
                    account.insert("name".to_string(), json!(v));
                }
                if let Some(v) = bank_name {
                    account.insert("bank_name".to_string(), json!(v));
                }
                if let Some(v) = is_primary {
                    account.insert("is_primary".to_string(), json!(v));
                }
                
                let body = json!({ "bank_account": account });
                let result = client.put(&format!("bank_accounts/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("bank_accounts/{}", id)).await?;
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
    fn bank_account_view_as_str_matches_api() {
        assert_eq!(BankAccountView::Standard.as_str(), "standard");
        assert_eq!(BankAccountView::CreditCard.as_str(), "credit_card");
        assert_eq!(BankAccountView::All.as_str(), "all");
    }

    #[test]
    fn bank_account_type_as_str_matches_api() {
        assert_eq!(BankAccountType::StandardBankAccount.as_str(), "StandardBankAccount");
        assert_eq!(BankAccountType::CreditCardAccount.as_str(), "CreditCardAccount");
        assert_eq!(BankAccountType::MerchantAccount.as_str(), "MerchantAccount");
    }
}
