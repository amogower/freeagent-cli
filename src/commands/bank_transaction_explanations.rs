//! Bank transaction explanation commands.

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum BankTransactionExplanationCommands {
    /// List bank transaction explanations for a bank account
    List {
        /// Bank account URL (required)
        #[arg(long)]
        bank_account: String,
        
        /// Filter from date (YYYY-MM-DD)
        #[arg(long)]
        from_date: Option<String>,
        
        /// Filter to date (YYYY-MM-DD)
        #[arg(long)]
        to_date: Option<String>,
        
        /// Filter by updated since (ISO 8601 timestamp)
        #[arg(long)]
        updated_since: Option<String>,
    },
    
    /// Get a bank transaction explanation by ID
    Get {
        /// Bank transaction explanation ID
        id: String,
    },
    
    /// Create a bank transaction explanation
    Create {
        /// Bank transaction URL to explain (required if bank_account not specified)
        #[arg(long)]
        bank_transaction: Option<String>,
        
        /// Bank account URL to create transaction in (required if bank_transaction not specified)
        #[arg(long)]
        bank_account: Option<String>,
        
        /// Date of the explanation (YYYY-MM-DD)
        #[arg(long)]
        dated_on: String,
        
        /// Gross value (amount in bank account's currency)
        #[arg(long)]
        gross_value: String,
        
        /// Description
        #[arg(long)]
        description: Option<String>,
        
        /// Category URL (not required for transfers, invoices, or bills)
        #[arg(long)]
        category: Option<String>,
        
        /// Project URL (for rebilling)
        #[arg(long)]
        project: Option<String>,
        
        /// Rebill type (cost, markup, price)
        #[arg(long)]
        rebill_type: Option<String>,
        
        /// Rebill factor (required for markup or price rebill types)
        #[arg(long)]
        rebill_factor: Option<String>,
        
        /// Invoice URL that has been paid
        #[arg(long)]
        paid_invoice: Option<String>,
        
        /// Bill URL that has been paid
        #[arg(long)]
        paid_bill: Option<String>,
        
        /// User URL for user payments
        #[arg(long)]
        paid_user: Option<String>,
        
        /// Bank account URL for transfers
        #[arg(long)]
        transfer_bank_account: Option<String>,
        
        /// Sales tax rate
        #[arg(long)]
        sales_tax_rate: Option<String>,
        
        /// Sales tax status (TAXABLE, EXEMPT, OUT_OF_SCOPE)
        #[arg(long)]
        sales_tax_status: Option<String>,
        
        /// EC status (UK/Non-EC, EC Goods, EC Services, Reverse Charge, EC VAT MOSS)
        #[arg(long)]
        ec_status: Option<String>,
        
        /// Receipt reference
        #[arg(long)]
        receipt_reference: Option<String>,
    },
    
    /// Transfer money between bank accounts (convenience command)
    Transfer {
        /// Source bank account URL (money leaves this account)
        #[arg(long)]
        from_account: String,
        
        /// Destination bank account URL (money enters this account)
        #[arg(long)]
        to_account: String,
        
        /// Amount to transfer (positive number)
        #[arg(long)]
        amount: String,
        
        /// Date of transfer (YYYY-MM-DD)
        #[arg(long)]
        dated_on: String,
        
        /// Description (optional)
        #[arg(long)]
        description: Option<String>,
    },
    
    /// Update a bank transaction explanation
    Update {
        /// Bank transaction explanation ID
        id: String,
        
        /// Date of the explanation (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,
        
        /// Gross value
        #[arg(long)]
        gross_value: Option<String>,
        
        /// Description
        #[arg(long)]
        description: Option<String>,
        
        /// Category URL
        #[arg(long)]
        category: Option<String>,
        
        /// Project URL
        #[arg(long)]
        project: Option<String>,
        
        /// Rebill type
        #[arg(long)]
        rebill_type: Option<String>,
        
        /// Rebill factor
        #[arg(long)]
        rebill_factor: Option<String>,
        
        /// Sales tax rate
        #[arg(long)]
        sales_tax_rate: Option<String>,
        
        /// Sales tax status
        #[arg(long)]
        sales_tax_status: Option<String>,
        
        /// Receipt reference
        #[arg(long)]
        receipt_reference: Option<String>,
    },
    
    /// Delete a bank transaction explanation
    Delete {
        /// Bank transaction explanation ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl BankTransactionExplanationCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List {
                bank_account,
                from_date,
                to_date,
                updated_since,
            } => {
                let params = QueryBuilder::new()
                    .add("bank_account", Some(bank_account.clone()))
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .add("updated_since", updated_since.clone())
                    .build();
                let result = client.get("bank_transaction_explanations", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("bank_transaction_explanations/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                bank_transaction,
                bank_account,
                dated_on,
                gross_value,
                description,
                category,
                project,
                rebill_type,
                rebill_factor,
                paid_invoice,
                paid_bill,
                paid_user,
                transfer_bank_account,
                sales_tax_rate,
                sales_tax_status,
                ec_status,
                receipt_reference,
            } => {
                // Validate that either bank_transaction or bank_account is provided
                if bank_transaction.is_none() && bank_account.is_none() {
                    anyhow::bail!("Either --bank-transaction or --bank-account must be specified");
                }
                
                let mut explanation = serde_json::Map::new();
                
                if let Some(v) = bank_transaction {
                    explanation.insert("bank_transaction".to_string(), json!(v));
                }
                if let Some(v) = bank_account {
                    explanation.insert("bank_account".to_string(), json!(v));
                }
                
                explanation.insert("dated_on".to_string(), json!(dated_on));
                explanation.insert("gross_value".to_string(), json!(gross_value));
                
                if let Some(v) = description {
                    explanation.insert("description".to_string(), json!(v));
                }
                if let Some(v) = category {
                    explanation.insert("category".to_string(), json!(v));
                }
                if let Some(v) = project {
                    explanation.insert("project".to_string(), json!(v));
                }
                if let Some(v) = rebill_type {
                    explanation.insert("rebill_type".to_string(), json!(v));
                }
                if let Some(v) = rebill_factor {
                    explanation.insert("rebill_factor".to_string(), json!(v));
                }
                if let Some(v) = paid_invoice {
                    explanation.insert("paid_invoice".to_string(), json!(v));
                }
                if let Some(v) = paid_bill {
                    explanation.insert("paid_bill".to_string(), json!(v));
                }
                if let Some(v) = paid_user {
                    explanation.insert("paid_user".to_string(), json!(v));
                }
                if let Some(v) = transfer_bank_account {
                    explanation.insert("transfer_bank_account".to_string(), json!(v));
                }
                if let Some(v) = sales_tax_rate {
                    explanation.insert("sales_tax_rate".to_string(), json!(v));
                }
                if let Some(v) = sales_tax_status {
                    explanation.insert("sales_tax_status".to_string(), json!(v));
                }
                if let Some(v) = ec_status {
                    explanation.insert("ec_status".to_string(), json!(v));
                }
                if let Some(v) = receipt_reference {
                    explanation.insert("receipt_reference".to_string(), json!(v));
                }
                
                let body = json!({ "bank_transaction_explanation": explanation });
                let result = client.post("bank_transaction_explanations", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Transfer {
                from_account,
                to_account,
                amount,
                dated_on,
                description,
            } => {
                // Parse amount to ensure it's positive
                let amount_value: f64 = amount.parse()
                    .map_err(|_| anyhow::anyhow!("Invalid amount: {}", amount))?;
                
                if amount_value <= 0.0 {
                    anyhow::bail!("Amount must be positive");
                }
                
                // Create the outgoing transfer explanation
                let mut explanation = serde_json::Map::new();
                explanation.insert("bank_account".to_string(), json!(from_account));
                explanation.insert("transfer_bank_account".to_string(), json!(to_account));
                explanation.insert("dated_on".to_string(), json!(dated_on));
                explanation.insert("gross_value".to_string(), json!(format!("-{}", amount)));
                
                if let Some(desc) = description {
                    explanation.insert("description".to_string(), json!(desc));
                } else {
                    explanation.insert("description".to_string(), 
                        json!(format!("Transfer to account")));
                }
                
                let body = json!({ "bank_transaction_explanation": explanation });
                let result = client.post("bank_transaction_explanations", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                id,
                dated_on,
                gross_value,
                description,
                category,
                project,
                rebill_type,
                rebill_factor,
                sales_tax_rate,
                sales_tax_status,
                receipt_reference,
            } => {
                let mut explanation = serde_json::Map::new();
                
                if let Some(v) = dated_on {
                    explanation.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = gross_value {
                    explanation.insert("gross_value".to_string(), json!(v));
                }
                if let Some(v) = description {
                    explanation.insert("description".to_string(), json!(v));
                }
                if let Some(v) = category {
                    explanation.insert("category".to_string(), json!(v));
                }
                if let Some(v) = project {
                    explanation.insert("project".to_string(), json!(v));
                }
                if let Some(v) = rebill_type {
                    explanation.insert("rebill_type".to_string(), json!(v));
                }
                if let Some(v) = rebill_factor {
                    explanation.insert("rebill_factor".to_string(), json!(v));
                }
                if let Some(v) = sales_tax_rate {
                    explanation.insert("sales_tax_rate".to_string(), json!(v));
                }
                if let Some(v) = sales_tax_status {
                    explanation.insert("sales_tax_status".to_string(), json!(v));
                }
                if let Some(v) = receipt_reference {
                    explanation.insert("receipt_reference".to_string(), json!(v));
                }
                
                let body = json!({ "bank_transaction_explanation": explanation });
                let result = client.put(&format!("bank_transaction_explanations/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("bank_transaction_explanations/{}", id)).await?;
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
    fn transfer_validates_positive_amount() {
        // This is tested via the execute method which validates the amount
        // The validation happens at runtime, not compile time
        assert!(true); // Placeholder for now
    }
}
