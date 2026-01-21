//! Estimate item commands.

use anyhow::{bail, Result};
use clap::Subcommand;
use serde_json::Value;

use crate::api::FreeAgentClient;
use crate::output::{print_output, OutputFormat};

fn build_estimate_item_body(
    estimate: Option<String>,
    item_type: Option<String>,
    description: Option<String>,
    quantity: Option<String>,
    price: Option<String>,
    sales_tax_rate: Option<String>,
    sales_tax_value: Option<String>,
    sales_tax_status: Option<String>,
    second_sales_tax_rate: Option<String>,
    second_sales_tax_value: Option<String>,
    second_sales_tax_status: Option<String>,
    category: Option<String>,
    data: Option<String>,
    require_core: bool,
) -> Result<Value> {
    let using_structured = estimate.is_some()
        || item_type.is_some()
        || description.is_some()
        || quantity.is_some()
        || price.is_some()
        || sales_tax_rate.is_some()
        || sales_tax_value.is_some()
        || sales_tax_status.is_some()
        || second_sales_tax_rate.is_some()
        || second_sales_tax_value.is_some()
        || second_sales_tax_status.is_some()
        || category.is_some();

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
            if let Some(ref kind) = item_type {
                let allowed = ["service", "product", "expense", "time"];
                if !allowed.contains(&kind.as_str()) {
                    bail!("--item-type must be one of: service, product, expense, time");
                }
            }
            if let Some(ref status) = sales_tax_status {
                let allowed = ["TAXABLE", "EXEMPT", "OUT_OF_SCOPE"];
                if !allowed.contains(&status.as_str()) {
                    bail!("--sales-tax-status must be one of: TAXABLE, EXEMPT, OUT_OF_SCOPE");
                }
            }
            if let Some(ref status) = second_sales_tax_status {
                let allowed = ["TAXABLE", "EXEMPT", "OUT_OF_SCOPE"];
                if !allowed.contains(&status.as_str()) {
                    bail!("--second-sales-tax-status must be one of: TAXABLE, EXEMPT, OUT_OF_SCOPE");
                }
            }
            if require_core {
                if estimate.is_none() || item_type.is_none() || description.is_none() || price.is_none() {
                    bail!("Missing required fields: --estimate, --item-type, --description, --price");
                }
            }
            let mut item = serde_json::Map::new();
            if let Some(v) = estimate {
                item.insert("estimate".to_string(), serde_json::json!(v));
            }
            if let Some(v) = item_type {
                item.insert("item_type".to_string(), serde_json::json!(v));
            }
            if let Some(v) = description {
                item.insert("description".to_string(), serde_json::json!(v));
            }
            if let Some(v) = quantity {
                item.insert("quantity".to_string(), serde_json::json!(v));
            }
            if let Some(v) = price {
                item.insert("price".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_rate {
                item.insert("sales_tax_rate".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_value {
                item.insert("sales_tax_value".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_status {
                item.insert("sales_tax_status".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_rate {
                item.insert("second_sales_tax_rate".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_value {
                item.insert("second_sales_tax_value".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_status {
                item.insert("second_sales_tax_status".to_string(), serde_json::json!(v));
            }
            if let Some(v) = category {
                item.insert("category".to_string(), serde_json::json!(v));
            }
            Ok(serde_json::json!({ "estimate_item": item }))
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum EstimateItemCommands {
    /// Create an estimate item (JSON payload)
    Create {
        /// Estimate URL
        #[arg(long)]
        estimate: Option<String>,

        /// Item type (service, product, expense, time)
        #[arg(long)]
        item_type: Option<String>,

        /// Description
        #[arg(long)]
        description: Option<String>,

        /// Quantity
        #[arg(long)]
        quantity: Option<String>,

        /// Price
        #[arg(long)]
        price: Option<String>,

        /// Sales tax rate
        #[arg(long)]
        sales_tax_rate: Option<String>,

        /// Sales tax value
        #[arg(long)]
        sales_tax_value: Option<String>,

        /// Sales tax status (TAXABLE, EXEMPT, OUT_OF_SCOPE)
        #[arg(long)]
        sales_tax_status: Option<String>,

        /// Second sales tax rate
        #[arg(long)]
        second_sales_tax_rate: Option<String>,

        /// Second sales tax value
        #[arg(long)]
        second_sales_tax_value: Option<String>,

        /// Second sales tax status
        #[arg(long)]
        second_sales_tax_status: Option<String>,

        /// Category URL
        #[arg(long)]
        category: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Update an estimate item (JSON payload)
    Update {
        /// Estimate item ID
        id: String,

        /// Estimate URL
        #[arg(long)]
        estimate: Option<String>,

        /// Item type (service, product, expense, time)
        #[arg(long)]
        item_type: Option<String>,

        /// Description
        #[arg(long)]
        description: Option<String>,

        /// Quantity
        #[arg(long)]
        quantity: Option<String>,

        /// Price
        #[arg(long)]
        price: Option<String>,

        /// Sales tax rate
        #[arg(long)]
        sales_tax_rate: Option<String>,

        /// Sales tax value
        #[arg(long)]
        sales_tax_value: Option<String>,

        /// Sales tax status (TAXABLE, EXEMPT, OUT_OF_SCOPE)
        #[arg(long)]
        sales_tax_status: Option<String>,

        /// Second sales tax rate
        #[arg(long)]
        second_sales_tax_rate: Option<String>,

        /// Second sales tax value
        #[arg(long)]
        second_sales_tax_value: Option<String>,

        /// Second sales tax status
        #[arg(long)]
        second_sales_tax_status: Option<String>,

        /// Category URL
        #[arg(long)]
        category: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Delete an estimate item
    Delete {
        /// Estimate item ID
        id: String,

        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl EstimateItemCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::Create {
                estimate,
                item_type,
                description,
                quantity,
                price,
                sales_tax_rate,
                sales_tax_value,
                sales_tax_status,
                second_sales_tax_rate,
                second_sales_tax_value,
                second_sales_tax_status,
                category,
                data,
            } => {
                let body = build_estimate_item_body(
                    estimate.clone(),
                    item_type.clone(),
                    description.clone(),
                    quantity.clone(),
                    price.clone(),
                    sales_tax_rate.clone(),
                    sales_tax_value.clone(),
                    sales_tax_status.clone(),
                    second_sales_tax_rate.clone(),
                    second_sales_tax_value.clone(),
                    second_sales_tax_status.clone(),
                    category.clone(),
                    data.clone(),
                    true,
                )?;
                let result = client.post("estimate_items", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                id,
                estimate,
                item_type,
                description,
                quantity,
                price,
                sales_tax_rate,
                sales_tax_value,
                sales_tax_status,
                second_sales_tax_rate,
                second_sales_tax_value,
                second_sales_tax_status,
                category,
                data,
            } => {
                let body = build_estimate_item_body(
                    estimate.clone(),
                    item_type.clone(),
                    description.clone(),
                    quantity.clone(),
                    price.clone(),
                    sales_tax_rate.clone(),
                    sales_tax_value.clone(),
                    sales_tax_status.clone(),
                    second_sales_tax_rate.clone(),
                    second_sales_tax_value.clone(),
                    second_sales_tax_status.clone(),
                    category.clone(),
                    data.clone(),
                    false,
                )?;
                let result = client.put(&format!("estimate_items/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("estimate_items/{}", id)).await?;
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
    fn estimate_item_rejects_invalid_item_type() {
        let result = build_estimate_item_body(
            Some("estimate".to_string()),
            Some("bad".to_string()),
            Some("Desc".to_string()),
            None,
            Some("10.00".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn estimate_item_requires_core_fields() {
        let result = build_estimate_item_body(
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn estimate_item_rejects_data_and_structured() {
        let result = build_estimate_item_body(
            Some("estimate".to_string()),
            Some("service".to_string()),
            Some("Desc".to_string()),
            None,
            Some("10.00".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some("{\"estimate_item\":{}}".to_string()),
            true,
        );
        assert!(result.is_err());
    }
}
