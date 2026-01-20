//! Recurring invoice commands.

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum RecurringInvoiceCommands {
    /// List all recurring invoices
    List {
        /// Filter by contact URL
        #[arg(long)]
        contact: Option<String>,
        
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get a recurring invoice by ID
    Get {
        /// Recurring invoice ID
        id: String,
    },
    
    /// Create a new recurring invoice
    Create {
        /// Contact URL (required)
        #[arg(long)]
        contact: String,
        
        /// Recurring profile name
        #[arg(long)]
        recurring_profile_name: Option<String>,
        
        /// Frequency period (week, month, year)
        #[arg(long)]
        frequency_period: Option<String>,
        
        /// Frequency (e.g., 1 for monthly, 2 for bi-monthly)
        #[arg(long)]
        frequency: Option<i32>,
        
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        
        /// Currency code
        #[arg(long)]
        currency: Option<String>,
    },
    
    /// Update a recurring invoice
    Update {
        /// Recurring invoice ID
        id: String,
        
        /// Recurring profile name
        #[arg(long)]
        recurring_profile_name: Option<String>,
        
        /// Frequency period (week, month, year)
        #[arg(long)]
        frequency_period: Option<String>,
        
        /// Frequency
        #[arg(long)]
        frequency: Option<i32>,
    },
    
    /// Delete a recurring invoice
    Delete {
        /// Recurring invoice ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl RecurringInvoiceCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { contact, page, per_page } => {
                let params = QueryBuilder::new()
                    .add("contact", contact.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("recurring_invoices", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("recurring_invoices/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                contact,
                recurring_profile_name,
                frequency_period,
                frequency,
                start_date,
                currency,
            } => {
                let mut recurring = serde_json::Map::new();
                recurring.insert("contact".to_string(), json!(contact));
                
                if let Some(v) = recurring_profile_name {
                    recurring.insert("recurring_profile_name".to_string(), json!(v));
                }
                if let Some(v) = frequency_period {
                    recurring.insert("frequency_period".to_string(), json!(v));
                }
                if let Some(v) = frequency {
                    recurring.insert("frequency".to_string(), json!(v));
                }
                if let Some(v) = start_date {
                    recurring.insert("start_date".to_string(), json!(v));
                }
                if let Some(v) = currency {
                    recurring.insert("currency".to_string(), json!(v));
                }
                
                let body = json!({ "recurring_invoice": recurring });
                let result = client.post("recurring_invoices", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, recurring_profile_name, frequency_period, frequency } => {
                let mut recurring = serde_json::Map::new();
                
                if let Some(v) = recurring_profile_name {
                    recurring.insert("recurring_profile_name".to_string(), json!(v));
                }
                if let Some(v) = frequency_period {
                    recurring.insert("frequency_period".to_string(), json!(v));
                }
                if let Some(v) = frequency {
                    recurring.insert("frequency".to_string(), json!(v));
                }
                
                let body = json!({ "recurring_invoice": recurring });
                let result = client.put(&format!("recurring_invoices/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("recurring_invoices/{}", id)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
