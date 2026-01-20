//! Note commands.

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum NoteCommands {
    /// List all notes
    List {
        /// Filter by parent URL (contact, project, etc.)
        #[arg(long)]
        parent: Option<String>,
        
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get a note by ID
    Get {
        /// Note ID
        id: String,
    },
    
    /// Create a new note
    Create {
        /// Parent URL (contact, project, etc.) (required)
        #[arg(long)]
        parent: String,
        
        /// Note text (required)
        #[arg(long)]
        text: String,
    },
    
    /// Update a note
    Update {
        /// Note ID
        id: String,
        
        /// Note text
        #[arg(long)]
        text: String,
    },
    
    /// Delete a note
    Delete {
        /// Note ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl NoteCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { parent, page, per_page } => {
                let params = QueryBuilder::new()
                    .add("parent", parent.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("notes", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("notes/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create { parent, text } => {
                let body = json!({
                    "note": {
                        "parent": parent,
                        "text": text
                    }
                });
                let result = client.post("notes", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, text } => {
                let body = json!({
                    "note": {
                        "text": text
                    }
                });
                let result = client.put(&format!("notes/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("notes/{}", id)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
