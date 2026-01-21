//! Expense commands.

use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Clone, ValueEnum)]
pub enum ExpenseView {
    All,
    Unbilled,
    Billed,
}

impl ExpenseView {
    fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Unbilled => "unbilled",
            Self::Billed => "billed",
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum ExpenseCommands {
    /// List all expenses
    List {
        /// Filter by view
        #[arg(long)]
        view: Option<ExpenseView>,
        
        /// Filter by user URL
        #[arg(long)]
        user: Option<String>,
        
        /// Filter by project URL
        #[arg(long)]
        project: Option<String>,
        
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
    
    /// Get an expense by ID
    Get {
        /// Expense ID
        id: String,
    },
    
    /// Create a new expense
    Create {
        /// User URL (required)
        #[arg(long)]
        user: String,
        
        /// Category URL (required)
        #[arg(long)]
        category: String,
        
        /// Expense date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: String,
        
        /// Gross value
        #[arg(long)]
        gross_value: String,
        
        /// Description
        #[arg(long)]
        description: Option<String>,
        
        /// Project URL
        #[arg(long)]
        project: Option<String>,
        
        /// Currency code
        #[arg(long)]
        currency: Option<String>,
        
        /// Rebill to project
        #[arg(long)]
        rebill_to_project: Option<bool>,
        
        /// Manual sales tax amount
        #[arg(long)]
        manual_sales_tax_amount: Option<String>,
    },
    
    /// Update an expense
    Update {
        /// Expense ID
        id: String,
        
        /// Expense date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,
        
        /// Gross value
        #[arg(long)]
        gross_value: Option<String>,
        
        /// Description
        #[arg(long)]
        description: Option<String>,
    },
    
    /// Delete an expense
    Delete {
        /// Expense ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl ExpenseCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List {
                view,
                user,
                project,
                from_date,
                to_date,
                page,
                per_page,
            } => {
                let params = QueryBuilder::new()
                    .add("view", view.as_ref().map(|v| v.as_str()))
                    .add("user", user.clone())
                    .add("project", project.clone())
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("expenses", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("expenses/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                user,
                category,
                dated_on,
                gross_value,
                description,
                project,
                currency,
                rebill_to_project,
                manual_sales_tax_amount,
            } => {
                let mut expense = serde_json::Map::new();
                expense.insert("user".to_string(), json!(user));
                expense.insert("category".to_string(), json!(category));
                expense.insert("dated_on".to_string(), json!(dated_on));
                expense.insert("gross_value".to_string(), json!(gross_value));
                
                if let Some(v) = description {
                    expense.insert("description".to_string(), json!(v));
                }
                if let Some(v) = project {
                    expense.insert("project".to_string(), json!(v));
                }
                if let Some(v) = currency {
                    expense.insert("currency".to_string(), json!(v));
                }
                if let Some(v) = rebill_to_project {
                    expense.insert("rebill_to_project".to_string(), json!(v));
                }
                if let Some(v) = manual_sales_tax_amount {
                    expense.insert("manual_sales_tax_amount".to_string(), json!(v));
                }
                
                let body = json!({ "expense": expense });
                let result = client.post("expenses", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, dated_on, gross_value, description } => {
                let mut expense = serde_json::Map::new();
                
                if let Some(v) = dated_on {
                    expense.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = gross_value {
                    expense.insert("gross_value".to_string(), json!(v));
                }
                if let Some(v) = description {
                    expense.insert("description".to_string(), json!(v));
                }
                
                let body = json!({ "expense": expense });
                let result = client.put(&format!("expenses/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("expenses/{}", id)).await?;
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
    fn expense_view_as_str_matches_api() {
        assert_eq!(ExpenseView::All.as_str(), "all");
        assert_eq!(ExpenseView::Unbilled.as_str(), "unbilled");
        assert_eq!(ExpenseView::Billed.as_str(), "billed");
    }
}
