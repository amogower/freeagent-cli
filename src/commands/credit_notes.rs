//! Credit note commands.

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum CreditNoteCommands {
    /// List all credit notes
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
    
    /// Get a credit note by ID
    Get {
        /// Credit note ID
        id: String,
    },
    
    /// Create a new credit note
    Create {
        /// Contact URL (required)
        #[arg(long)]
        contact: String,
        
        /// Credit note date (YYYY-MM-DD)
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
    
    /// Update a credit note
    Update {
        /// Credit note ID
        id: String,
        
        /// Credit note date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,
        
        /// Reference
        #[arg(long)]
        reference: Option<String>,
        
        /// Comments
        #[arg(long)]
        comments: Option<String>,
    },
    
    /// Delete a credit note
    Delete {
        /// Credit note ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl CreditNoteCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { contact, project, page, per_page } => {
                let params = QueryBuilder::new()
                    .add("contact", contact.clone())
                    .add("project", project.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("credit_notes", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("credit_notes/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create { contact, dated_on, reference, currency, comments } => {
                let mut credit_note = serde_json::Map::new();
                credit_note.insert("contact".to_string(), json!(contact));
                
                if let Some(v) = dated_on {
                    credit_note.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = reference {
                    credit_note.insert("reference".to_string(), json!(v));
                }
                if let Some(v) = currency {
                    credit_note.insert("currency".to_string(), json!(v));
                }
                if let Some(v) = comments {
                    credit_note.insert("comments".to_string(), json!(v));
                }
                
                let body = json!({ "credit_note": credit_note });
                let result = client.post("credit_notes", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, dated_on, reference, comments } => {
                let mut credit_note = serde_json::Map::new();
                
                if let Some(v) = dated_on {
                    credit_note.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = reference {
                    credit_note.insert("reference".to_string(), json!(v));
                }
                if let Some(v) = comments {
                    credit_note.insert("comments".to_string(), json!(v));
                }
                
                let body = json!({ "credit_note": credit_note });
                let result = client.put(&format!("credit_notes/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("credit_notes/{}", id)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
