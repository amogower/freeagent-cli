//! Estimate commands.

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum EstimateCommands {
    /// List all estimates
    List {
        /// Filter by contact URL
        #[arg(long)]
        contact: Option<String>,
        
        /// Filter by project URL
        #[arg(long)]
        project: Option<String>,
        
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get an estimate by ID
    Get {
        /// Estimate ID
        id: String,
    },
    
    /// Create a new estimate
    Create {
        /// Contact URL (required)
        #[arg(long)]
        contact: String,
        
        /// Estimate date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,
        
        /// Reference
        #[arg(long)]
        reference: Option<String>,
        
        /// Currency code
        #[arg(long)]
        currency: Option<String>,
        
        /// Comments
        #[arg(long)]
        comments: Option<String>,
    },
    
    /// Update an estimate
    Update {
        /// Estimate ID
        id: String,
        
        /// Estimate date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,
        
        /// Reference
        #[arg(long)]
        reference: Option<String>,
        
        /// Comments
        #[arg(long)]
        comments: Option<String>,
    },
    
    /// Delete an estimate
    Delete {
        /// Estimate ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
    
    /// Send estimate by email
    SendEmail {
        /// Estimate ID
        id: String,
        
        /// Email recipient
        #[arg(long)]
        email_to: String,
    },
}

impl EstimateCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { contact, project, page, per_page } => {
                let params = QueryBuilder::new()
                    .add("contact", contact.clone())
                    .add("project", project.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("estimates", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("estimates/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create { contact, dated_on, reference, currency, comments } => {
                let mut estimate = serde_json::Map::new();
                estimate.insert("contact".to_string(), json!(contact));
                
                if let Some(v) = dated_on {
                    estimate.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = reference {
                    estimate.insert("reference".to_string(), json!(v));
                }
                if let Some(v) = currency {
                    estimate.insert("currency".to_string(), json!(v));
                }
                if let Some(v) = comments {
                    estimate.insert("comments".to_string(), json!(v));
                }
                
                let body = json!({ "estimate": estimate });
                let result = client.post("estimates", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, dated_on, reference, comments } => {
                let mut estimate = serde_json::Map::new();
                
                if let Some(v) = dated_on {
                    estimate.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = reference {
                    estimate.insert("reference".to_string(), json!(v));
                }
                if let Some(v) = comments {
                    estimate.insert("comments".to_string(), json!(v));
                }
                
                let body = json!({ "estimate": estimate });
                let result = client.put(&format!("estimates/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("estimates/{}", id)).await?;
                print_output(&result, format);
            }
            Self::SendEmail { id, email_to } => {
                let body = json!({ "email": { "to": email_to } });
                let result = client.post(&format!("estimates/{}/send_email", id), Some(body)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
