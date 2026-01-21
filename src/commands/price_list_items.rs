//! Price list item commands.

use anyhow::{bail, Result};
use clap::Subcommand;
use serde_json::Value;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

fn build_price_list_item_body(
    code: Option<String>,
    item_type: Option<String>,
    description: Option<String>,
    price: Option<String>,
    quantity: Option<String>,
    vat_status: Option<String>,
    sales_tax_rate: Option<String>,
    second_sales_tax_rate: Option<String>,
    category: Option<String>,
    stock_item: Option<String>,
    data: Option<String>,
    require_core: bool,
) -> Result<Value> {
    let using_structured = code.is_some()
        || item_type.is_some()
        || description.is_some()
        || price.is_some()
        || quantity.is_some()
        || vat_status.is_some()
        || sales_tax_rate.is_some()
        || second_sales_tax_rate.is_some()
        || category.is_some()
        || stock_item.is_some();
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
            if let Some(ref status) = vat_status {
                let allowed = ["VAT", "EXEMPT", "OUT_OF_SCOPE"];
                if !allowed.contains(&status.as_str()) {
                    bail!("--vat-status must be one of: VAT, EXEMPT, OUT_OF_SCOPE");
                }
            }
            if require_core
                && (code.is_none()
                    || item_type.is_none()
                    || description.is_none()
                    || price.is_none()
                    || quantity.is_none())
            {
                bail!("Missing required fields: --code, --item-type, --description, --price, --quantity");
            }
            let mut item = serde_json::Map::new();
            if let Some(v) = code {
                item.insert("code".to_string(), serde_json::json!(v));
            }
            if let Some(v) = item_type {
                item.insert("item_type".to_string(), serde_json::json!(v));
            }
            if let Some(v) = description {
                item.insert("description".to_string(), serde_json::json!(v));
            }
            if let Some(v) = price {
                item.insert("price".to_string(), serde_json::json!(v));
            }
            if let Some(v) = quantity {
                item.insert("quantity".to_string(), serde_json::json!(v));
            }
            if let Some(v) = vat_status {
                item.insert("vat_status".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_rate {
                item.insert("sales_tax_rate".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_rate {
                item.insert("second_sales_tax_rate".to_string(), serde_json::json!(v));
            }
            if let Some(v) = category {
                item.insert("category".to_string(), serde_json::json!(v));
            }
            if let Some(v) = stock_item {
                item.insert("stock_item".to_string(), serde_json::json!(v));
            }
            Ok(serde_json::json!({ "price_list_item": item }))
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum PriceListItemCommands {
    /// List price list items
    List {
        /// Sort by (code, description, price, quantity, created_at, updated_at)
        #[arg(long)]
        sort: Option<String>,

        /// Page number
        #[arg(long)]
        page: Option<i32>,

        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },

    /// Get a price list item by ID
    Get {
        /// Price list item ID
        id: String,
    },

    /// Create a price list item (JSON payload)
    Create {
        /// Code
        #[arg(long)]
        code: Option<String>,

        /// Item type (service, product, expense, time)
        #[arg(long)]
        item_type: Option<String>,

        /// Description
        #[arg(long)]
        description: Option<String>,

        /// Price
        #[arg(long)]
        price: Option<String>,

        /// Quantity
        #[arg(long)]
        quantity: Option<String>,

        /// VAT status (VAT, EXEMPT, OUT_OF_SCOPE)
        #[arg(long)]
        vat_status: Option<String>,

        /// Sales tax rate
        #[arg(long)]
        sales_tax_rate: Option<String>,

        /// Second sales tax rate
        #[arg(long)]
        second_sales_tax_rate: Option<String>,

        /// Category URL
        #[arg(long)]
        category: Option<String>,

        /// Stock item URL
        #[arg(long)]
        stock_item: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Update a price list item (JSON payload)
    Update {
        /// Price list item ID
        id: String,

        /// Code
        #[arg(long)]
        code: Option<String>,

        /// Item type (service, product, expense, time)
        #[arg(long)]
        item_type: Option<String>,

        /// Description
        #[arg(long)]
        description: Option<String>,

        /// Price
        #[arg(long)]
        price: Option<String>,

        /// Quantity
        #[arg(long)]
        quantity: Option<String>,

        /// VAT status (VAT, EXEMPT, OUT_OF_SCOPE)
        #[arg(long)]
        vat_status: Option<String>,

        /// Sales tax rate
        #[arg(long)]
        sales_tax_rate: Option<String>,

        /// Second sales tax rate
        #[arg(long)]
        second_sales_tax_rate: Option<String>,

        /// Category URL
        #[arg(long)]
        category: Option<String>,

        /// Stock item URL
        #[arg(long)]
        stock_item: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Delete a price list item
    Delete {
        /// Price list item ID
        id: String,

        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl PriceListItemCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { sort, page, per_page } => {
                let params = QueryBuilder::new()
                    .add("sort", sort.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("price_list_items", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("price_list_items/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                code,
                item_type,
                description,
                price,
                quantity,
                vat_status,
                sales_tax_rate,
                second_sales_tax_rate,
                category,
                stock_item,
                data,
            } => {
                let body = build_price_list_item_body(
                    code.clone(),
                    item_type.clone(),
                    description.clone(),
                    price.clone(),
                    quantity.clone(),
                    vat_status.clone(),
                    sales_tax_rate.clone(),
                    second_sales_tax_rate.clone(),
                    category.clone(),
                    stock_item.clone(),
                    data.clone(),
                    true,
                )?;
                let result = client.post("price_list_items", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                id,
                code,
                item_type,
                description,
                price,
                quantity,
                vat_status,
                sales_tax_rate,
                second_sales_tax_rate,
                category,
                stock_item,
                data,
            } => {
                let body = build_price_list_item_body(
                    code.clone(),
                    item_type.clone(),
                    description.clone(),
                    price.clone(),
                    quantity.clone(),
                    vat_status.clone(),
                    sales_tax_rate.clone(),
                    second_sales_tax_rate.clone(),
                    category.clone(),
                    stock_item.clone(),
                    data.clone(),
                    false,
                )?;
                let result = client.put(&format!("price_list_items/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("price_list_item/{}", id)).await?;
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
    fn price_list_item_rejects_invalid_item_type() {
        let result = build_price_list_item_body(
            Some("CODE".to_string()),
            Some("bad".to_string()),
            Some("Desc".to_string()),
            Some("10.00".to_string()),
            Some("1".to_string()),
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
    fn price_list_item_requires_core_fields() {
        let result = build_price_list_item_body(
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
    fn price_list_item_rejects_data_and_structured() {
        let result = build_price_list_item_body(
            Some("CODE".to_string()),
            Some("service".to_string()),
            Some("Desc".to_string()),
            Some("10.00".to_string()),
            Some("1".to_string()),
            None,
            None,
            None,
            None,
            None,
            Some("{\"price_list_item\":{}}".to_string()),
            true,
        );
        assert!(result.is_err());
    }
}
