//! Property commands.

use anyhow::{bail, Result};
use clap::Subcommand;
use serde_json::Value;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

fn build_property_body(
    address1: Option<String>,
    address2: Option<String>,
    address3: Option<String>,
    town: Option<String>,
    region: Option<String>,
    postcode: Option<String>,
    data: Option<String>,
    require_core: bool,
) -> Result<Value> {
    let using_structured = address1.is_some()
        || address2.is_some()
        || address3.is_some()
        || town.is_some()
        || region.is_some()
        || postcode.is_some();
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
            if require_core && address1.is_none() {
                bail!("Missing required field: --address1");
            }
            let mut property = serde_json::Map::new();
            if let Some(v) = address1 {
                property.insert("address1".to_string(), serde_json::json!(v));
            }
            if let Some(v) = address2 {
                property.insert("address2".to_string(), serde_json::json!(v));
            }
            if let Some(v) = address3 {
                property.insert("address3".to_string(), serde_json::json!(v));
            }
            if let Some(v) = town {
                property.insert("town".to_string(), serde_json::json!(v));
            }
            if let Some(v) = region {
                property.insert("region".to_string(), serde_json::json!(v));
            }
            if let Some(v) = postcode {
                property.insert("postcode".to_string(), serde_json::json!(v));
            }
            Ok(serde_json::json!({ "property": property }))
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum PropertyCommands {
    /// List properties
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,

        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },

    /// Get a property by ID
    Get {
        /// Property ID
        id: String,
    },

    /// Create a property (JSON payload)
    Create {
        /// Address line 1 (required)
        #[arg(long)]
        address1: Option<String>,

        /// Address line 2
        #[arg(long)]
        address2: Option<String>,

        /// Address line 3
        #[arg(long)]
        address3: Option<String>,

        /// Town
        #[arg(long)]
        town: Option<String>,

        /// Region
        #[arg(long)]
        region: Option<String>,

        /// Postcode
        #[arg(long)]
        postcode: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Update a property (JSON payload)
    Update {
        /// Property ID
        id: String,

        /// Address line 1
        #[arg(long)]
        address1: Option<String>,

        /// Address line 2
        #[arg(long)]
        address2: Option<String>,

        /// Address line 3
        #[arg(long)]
        address3: Option<String>,

        /// Town
        #[arg(long)]
        town: Option<String>,

        /// Region
        #[arg(long)]
        region: Option<String>,

        /// Postcode
        #[arg(long)]
        postcode: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Delete a property
    Delete {
        /// Property ID
        id: String,

        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl PropertyCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("properties", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("properties/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                address1,
                address2,
                address3,
                town,
                region,
                postcode,
                data,
            } => {
                let body = build_property_body(
                    address1.clone(),
                    address2.clone(),
                    address3.clone(),
                    town.clone(),
                    region.clone(),
                    postcode.clone(),
                    data.clone(),
                    true,
                )?;
                let result = client.post("properties", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                id,
                address1,
                address2,
                address3,
                town,
                region,
                postcode,
                data,
            } => {
                let body = build_property_body(
                    address1.clone(),
                    address2.clone(),
                    address3.clone(),
                    town.clone(),
                    region.clone(),
                    postcode.clone(),
                    data.clone(),
                    false,
                )?;
                let result = client.put(&format!("properties/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("properties/{}", id)).await?;
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
    fn property_requires_address1_on_create() {
        let result = build_property_body(None, None, None, None, None, None, None, true);
        assert!(result.is_err());
    }

    #[test]
    fn property_rejects_data_and_structured() {
        let result = build_property_body(
            Some("Address 1".to_string()),
            None,
            None,
            None,
            None,
            None,
            Some("{\"property\":{}}".to_string()),
            true,
        );
        assert!(result.is_err());
    }
}
