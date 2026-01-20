//! Stock item commands.

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum StockItemCommands {
    /// List all stock items
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get a stock item by ID
    Get {
        /// Stock item ID
        id: String,
    },
    
    /// Create a new stock item
    Create {
        /// Description (required)
        #[arg(long)]
        description: String,
        
        /// Opening quantity
        #[arg(long)]
        opening_quantity: Option<String>,
        
        /// Opening balance
        #[arg(long)]
        opening_balance: Option<String>,
        
        /// Stock item code
        #[arg(long)]
        code: Option<String>,
    },
    
    /// Update a stock item
    Update {
        /// Stock item ID
        id: String,
        
        /// Description
        #[arg(long)]
        description: Option<String>,
        
        /// Stock item code
        #[arg(long)]
        code: Option<String>,
    },
    
    /// Delete a stock item
    Delete {
        /// Stock item ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl StockItemCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("stock_items", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("stock_items/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create { description, opening_quantity, opening_balance, code } => {
                let mut item = serde_json::Map::new();
                item.insert("description".to_string(), json!(description));
                
                if let Some(v) = opening_quantity {
                    item.insert("opening_quantity".to_string(), json!(v));
                }
                if let Some(v) = opening_balance {
                    item.insert("opening_balance".to_string(), json!(v));
                }
                if let Some(v) = code {
                    item.insert("code".to_string(), json!(v));
                }
                
                let body = json!({ "stock_item": item });
                let result = client.post("stock_items", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, description, code } => {
                let mut item = serde_json::Map::new();
                
                if let Some(v) = description {
                    item.insert("description".to_string(), json!(v));
                }
                if let Some(v) = code {
                    item.insert("code".to_string(), json!(v));
                }
                
                let body = json!({ "stock_item": item });
                let result = client.put(&format!("stock_items/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("stock_items/{}", id)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
