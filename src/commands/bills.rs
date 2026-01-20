//! Bill commands.

use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Clone, ValueEnum)]
pub enum BillView {
    RecentOpenOrOverdue,
    OpenOrOverdue,
    Open,
    Overdue,
    Draft,
    All,
}

impl BillView {
    fn as_str(&self) -> &'static str {
        match self {
            Self::RecentOpenOrOverdue => "recent_open_or_overdue",
            Self::OpenOrOverdue => "open_or_overdue",
            Self::Open => "open",
            Self::Overdue => "overdue",
            Self::Draft => "draft",
            Self::All => "all",
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum BillCommands {
    /// List all bills
    List {
        /// Filter by view
        #[arg(long)]
        view: Option<BillView>,
        
        /// Filter by contact URL
        #[arg(long)]
        contact: Option<String>,
        
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
    
    /// Get a bill by ID
    Get {
        /// Bill ID
        id: String,
    },
    
    /// Create a new bill
    Create {
        /// Contact URL (required)
        #[arg(long)]
        contact: String,
        
        /// Bill date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,
        
        /// Due date (YYYY-MM-DD)
        #[arg(long)]
        due_on: Option<String>,
        
        /// Reference
        #[arg(long)]
        reference: Option<String>,
        
        /// Currency code
        #[arg(long)]
        currency: Option<String>,
        
        /// Total value
        #[arg(long)]
        total_value: Option<String>,
        
        /// Category URL
        #[arg(long)]
        category: Option<String>,
        
        /// Comments
        #[arg(long)]
        comments: Option<String>,
    },
    
    /// Update a bill
    Update {
        /// Bill ID
        id: String,
        
        /// Bill date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,
        
        /// Due date (YYYY-MM-DD)
        #[arg(long)]
        due_on: Option<String>,
        
        /// Reference
        #[arg(long)]
        reference: Option<String>,
        
        /// Comments
        #[arg(long)]
        comments: Option<String>,
    },
    
    /// Delete a bill
    Delete {
        /// Bill ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl BillCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List {
                view,
                contact,
                from_date,
                to_date,
                page,
                per_page,
            } => {
                let params = QueryBuilder::new()
                    .add("view", view.as_ref().map(|v| v.as_str()))
                    .add("contact", contact.clone())
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("bills", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("bills/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                contact,
                dated_on,
                due_on,
                reference,
                currency,
                total_value,
                category,
                comments,
            } => {
                let mut bill = serde_json::Map::new();
                bill.insert("contact".to_string(), json!(contact));
                
                if let Some(v) = dated_on {
                    bill.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = due_on {
                    bill.insert("due_on".to_string(), json!(v));
                }
                if let Some(v) = reference {
                    bill.insert("reference".to_string(), json!(v));
                }
                if let Some(v) = currency {
                    bill.insert("currency".to_string(), json!(v));
                }
                if let Some(v) = total_value {
                    bill.insert("total_value".to_string(), json!(v));
                }
                if let Some(v) = category {
                    bill.insert("category".to_string(), json!(v));
                }
                if let Some(v) = comments {
                    bill.insert("comments".to_string(), json!(v));
                }
                
                let body = json!({ "bill": bill });
                let result = client.post("bills", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, dated_on, due_on, reference, comments } => {
                let mut bill = serde_json::Map::new();
                
                if let Some(v) = dated_on {
                    bill.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = due_on {
                    bill.insert("due_on".to_string(), json!(v));
                }
                if let Some(v) = reference {
                    bill.insert("reference".to_string(), json!(v));
                }
                if let Some(v) = comments {
                    bill.insert("comments".to_string(), json!(v));
                }
                
                let body = json!({ "bill": bill });
                let result = client.put(&format!("bills/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("bills/{}", id)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
