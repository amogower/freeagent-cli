//! Capital asset commands.

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum CapitalAssetCommands {
    /// List all capital assets
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get a capital asset by ID
    Get {
        /// Capital asset ID
        id: String,
    },
    
    /// Create a new capital asset
    Create {
        /// Description (required)
        #[arg(long)]
        description: String,
        
        /// Asset life in years (required)
        #[arg(long)]
        asset_life_years: i32,
        
        /// Purchase date (YYYY-MM-DD)
        #[arg(long)]
        purchased_on: Option<String>,
        
        /// Purchase price
        #[arg(long)]
        purchase_price: Option<String>,
        
        /// Category URL
        #[arg(long)]
        category: Option<String>,
    },
    
    /// Update a capital asset
    Update {
        /// Capital asset ID
        id: String,
        
        /// Description
        #[arg(long)]
        description: Option<String>,
        
        /// Asset life in years
        #[arg(long)]
        asset_life_years: Option<i32>,
    },
    
    /// Delete a capital asset
    Delete {
        /// Capital asset ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl CapitalAssetCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("capital_assets", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("capital_assets/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                description,
                asset_life_years,
                purchased_on,
                purchase_price,
                category,
            } => {
                let mut asset = serde_json::Map::new();
                asset.insert("description".to_string(), json!(description));
                asset.insert("asset_life_years".to_string(), json!(asset_life_years));
                
                if let Some(v) = purchased_on {
                    asset.insert("purchased_on".to_string(), json!(v));
                }
                if let Some(v) = purchase_price {
                    asset.insert("purchase_price".to_string(), json!(v));
                }
                if let Some(v) = category {
                    asset.insert("category".to_string(), json!(v));
                }
                
                let body = json!({ "capital_asset": asset });
                let result = client.post("capital_assets", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, description, asset_life_years } => {
                let mut asset = serde_json::Map::new();
                
                if let Some(v) = description {
                    asset.insert("description".to_string(), json!(v));
                }
                if let Some(v) = asset_life_years {
                    asset.insert("asset_life_years".to_string(), json!(v));
                }
                
                let body = json!({ "capital_asset": asset });
                let result = client.put(&format!("capital_assets/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("capital_assets/{}", id)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
