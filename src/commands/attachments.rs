//! Attachment commands.

use anyhow::Result;
use clap::Subcommand;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum AttachmentCommands {
    /// List all attachments
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get an attachment by ID
    Get {
        /// Attachment ID
        id: String,
    },
    
    /// Delete an attachment
    Delete {
        /// Attachment ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl AttachmentCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("attachments", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("attachments/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("attachments/{}", id)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
