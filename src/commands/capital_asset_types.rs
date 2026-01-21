//! Capital asset type commands.

use anyhow::{bail, Result};
use clap::Subcommand;
use serde_json::Value;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

fn build_capital_asset_type_body(
    name: Option<String>,
    data: Option<String>,
    require_core: bool,
) -> Result<Value> {
    let using_structured = name.is_some();
    match data {
        Some(raw) => {
            if using_structured {
                bail!("Use either structured fields or --data, not both");
            }
            Ok(serde_json::from_str(&raw)?)
        }
        None => {
            if !using_structured {
                bail!("Provide structured fields or --data");
            }
            if require_core && name.is_none() {
                bail!("Missing required field: --name");
            }
            let mut asset_type = serde_json::Map::new();
            if let Some(v) = name {
                asset_type.insert("name".to_string(), serde_json::json!(v));
            }
            Ok(serde_json::json!({ "capital_asset_type": asset_type }))
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum CapitalAssetTypeCommands {
    /// List capital asset types
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,

        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },

    /// Get a capital asset type by ID
    Get {
        /// Capital asset type ID
        id: String,
    },

    /// Create a capital asset type (JSON payload)
    Create {
        /// Name (required)
        #[arg(long)]
        name: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Update a capital asset type (JSON payload)
    Update {
        /// Capital asset type ID
        id: String,

        /// Name
        #[arg(long)]
        name: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Delete a capital asset type
    Delete {
        /// Capital asset type ID
        id: String,

        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl CapitalAssetTypeCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("capital_asset_types", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("capital_asset_types/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create { name, data } => {
                let body = build_capital_asset_type_body(name.clone(), data.clone(), true)?;
                let result = client.post("capital_asset_types", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, name, data } => {
                let body = build_capital_asset_type_body(name.clone(), data.clone(), false)?;
                let result = client.put(&format!("capital_asset_types/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("capital_asset_types/{}", id)).await?;
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
    fn capital_asset_type_requires_name() {
        let result = build_capital_asset_type_body(None, None, true);
        assert!(result.is_err());
    }

    #[test]
    fn capital_asset_type_rejects_data_and_structured() {
        let result = build_capital_asset_type_body(
            Some("Equipment".to_string()),
            Some("{\"capital_asset_type\":{}}".to_string()),
            true,
        );
        assert!(result.is_err());
    }
}
