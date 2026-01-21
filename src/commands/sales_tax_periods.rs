//! Sales tax period commands.

use anyhow::{bail, Result};
use clap::Subcommand;
use serde_json::Value;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

fn build_sales_tax_period_body(
    sales_tax_name: Option<String>,
    sales_tax_registration_status: Option<String>,
    sales_tax_rate_1: Option<String>,
    sales_tax_rate_2: Option<String>,
    sales_tax_rate_3: Option<String>,
    sales_tax_is_value_added: Option<bool>,
    sales_tax_registration_number: Option<String>,
    effective_date: Option<String>,
    second_sales_tax_name: Option<String>,
    second_sales_tax_rate_1: Option<String>,
    second_sales_tax_rate_2: Option<String>,
    second_sales_tax_rate_3: Option<String>,
    second_sales_tax_is_compound: Option<bool>,
    data: Option<String>,
    require_core: bool,
) -> Result<Value> {
    let using_structured = sales_tax_name.is_some()
        || sales_tax_registration_status.is_some()
        || sales_tax_rate_1.is_some()
        || sales_tax_rate_2.is_some()
        || sales_tax_rate_3.is_some()
        || sales_tax_is_value_added.is_some()
        || sales_tax_registration_number.is_some()
        || effective_date.is_some()
        || second_sales_tax_name.is_some()
        || second_sales_tax_rate_1.is_some()
        || second_sales_tax_rate_2.is_some()
        || second_sales_tax_rate_3.is_some()
        || second_sales_tax_is_compound.is_some();
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
            if let Some(ref status) = sales_tax_registration_status {
                let allowed = ["Registered", "Not Registered"];
                if !allowed.contains(&status.as_str()) {
                    bail!("--sales-tax-registration-status must be one of: Registered, Not Registered");
                }
            }
            if require_core
                && (sales_tax_name.is_none()
                    || sales_tax_registration_status.is_none()
                    || sales_tax_rate_1.is_none()
                    || sales_tax_is_value_added.is_none()
                    || effective_date.is_none())
            {
                bail!("Missing required fields: --sales-tax-name, --sales-tax-registration-status, --sales-tax-rate-1, --sales-tax-is-value-added, --effective-date");
            }
            let mut period = serde_json::Map::new();
            if let Some(v) = sales_tax_name {
                period.insert("sales_tax_name".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_registration_status {
                period.insert(
                    "sales_tax_registration_status".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(v) = sales_tax_rate_1 {
                period.insert("sales_tax_rate_1".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_rate_2 {
                period.insert("sales_tax_rate_2".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_rate_3 {
                period.insert("sales_tax_rate_3".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_is_value_added {
                period.insert("sales_tax_is_value_added".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_registration_number {
                period.insert("sales_tax_registration_number".to_string(), serde_json::json!(v));
            }
            if let Some(v) = effective_date {
                period.insert("effective_date".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_name {
                period.insert("second_sales_tax_name".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_rate_1 {
                period.insert("second_sales_tax_rate_1".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_rate_2 {
                period.insert("second_sales_tax_rate_2".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_rate_3 {
                period.insert("second_sales_tax_rate_3".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_is_compound {
                period.insert("second_sales_tax_is_compound".to_string(), serde_json::json!(v));
            }
            Ok(serde_json::json!({ "sales_tax_period": period }))
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum SalesTaxPeriodCommands {
    /// List sales tax periods
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,

        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },

    /// Get a sales tax period by ID
    Get {
        /// Sales tax period ID
        id: String,
    },

    /// Create a sales tax period (JSON payload)
    Create {
        /// Sales tax name (e.g., VAT, GST, Sales Tax)
        #[arg(long)]
        sales_tax_name: Option<String>,

        /// Sales tax registration status (Registered, Not Registered)
        #[arg(long)]
        sales_tax_registration_status: Option<String>,

        /// First sales tax rate
        #[arg(long)]
        sales_tax_rate_1: Option<String>,

        /// Second sales tax rate
        #[arg(long)]
        sales_tax_rate_2: Option<String>,

        /// Third sales tax rate
        #[arg(long)]
        sales_tax_rate_3: Option<String>,

        /// True if tax is value-added
        #[arg(long)]
        sales_tax_is_value_added: Option<bool>,

        /// Sales tax registration number
        #[arg(long)]
        sales_tax_registration_number: Option<String>,

        /// Effective date (YYYY-MM-DD)
        #[arg(long)]
        effective_date: Option<String>,

        /// Second sales tax name (Universal accounts)
        #[arg(long)]
        second_sales_tax_name: Option<String>,

        /// Second sales tax rate 1 (Universal accounts)
        #[arg(long)]
        second_sales_tax_rate_1: Option<String>,

        /// Second sales tax rate 2 (Universal accounts)
        #[arg(long)]
        second_sales_tax_rate_2: Option<String>,

        /// Second sales tax rate 3 (Universal accounts)
        #[arg(long)]
        second_sales_tax_rate_3: Option<String>,

        /// Second sales tax is compound (Universal accounts)
        #[arg(long)]
        second_sales_tax_is_compound: Option<bool>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Update a sales tax period (JSON payload)
    Update {
        /// Sales tax period ID
        id: String,

        /// Sales tax name (e.g., VAT, GST, Sales Tax)
        #[arg(long)]
        sales_tax_name: Option<String>,

        /// Sales tax registration status (Registered, Not Registered)
        #[arg(long)]
        sales_tax_registration_status: Option<String>,

        /// First sales tax rate
        #[arg(long)]
        sales_tax_rate_1: Option<String>,

        /// Second sales tax rate
        #[arg(long)]
        sales_tax_rate_2: Option<String>,

        /// Third sales tax rate
        #[arg(long)]
        sales_tax_rate_3: Option<String>,

        /// True if tax is value-added
        #[arg(long)]
        sales_tax_is_value_added: Option<bool>,

        /// Sales tax registration number
        #[arg(long)]
        sales_tax_registration_number: Option<String>,

        /// Effective date (YYYY-MM-DD)
        #[arg(long)]
        effective_date: Option<String>,

        /// Second sales tax name (Universal accounts)
        #[arg(long)]
        second_sales_tax_name: Option<String>,

        /// Second sales tax rate 1 (Universal accounts)
        #[arg(long)]
        second_sales_tax_rate_1: Option<String>,

        /// Second sales tax rate 2 (Universal accounts)
        #[arg(long)]
        second_sales_tax_rate_2: Option<String>,

        /// Second sales tax rate 3 (Universal accounts)
        #[arg(long)]
        second_sales_tax_rate_3: Option<String>,

        /// Second sales tax is compound (Universal accounts)
        #[arg(long)]
        second_sales_tax_is_compound: Option<bool>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Delete a sales tax period
    Delete {
        /// Sales tax period ID
        id: String,

        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl SalesTaxPeriodCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("sales_tax_periods", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("sales_tax_periods/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                sales_tax_name,
                sales_tax_registration_status,
                sales_tax_rate_1,
                sales_tax_rate_2,
                sales_tax_rate_3,
                sales_tax_is_value_added,
                sales_tax_registration_number,
                effective_date,
                second_sales_tax_name,
                second_sales_tax_rate_1,
                second_sales_tax_rate_2,
                second_sales_tax_rate_3,
                second_sales_tax_is_compound,
                data,
            } => {
                let body = build_sales_tax_period_body(
                    sales_tax_name.clone(),
                    sales_tax_registration_status.clone(),
                    sales_tax_rate_1.clone(),
                    sales_tax_rate_2.clone(),
                    sales_tax_rate_3.clone(),
                    *sales_tax_is_value_added,
                    sales_tax_registration_number.clone(),
                    effective_date.clone(),
                    second_sales_tax_name.clone(),
                    second_sales_tax_rate_1.clone(),
                    second_sales_tax_rate_2.clone(),
                    second_sales_tax_rate_3.clone(),
                    *second_sales_tax_is_compound,
                    data.clone(),
                    true,
                )?;
                let result = client.post("sales_tax_periods", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                id,
                sales_tax_name,
                sales_tax_registration_status,
                sales_tax_rate_1,
                sales_tax_rate_2,
                sales_tax_rate_3,
                sales_tax_is_value_added,
                sales_tax_registration_number,
                effective_date,
                second_sales_tax_name,
                second_sales_tax_rate_1,
                second_sales_tax_rate_2,
                second_sales_tax_rate_3,
                second_sales_tax_is_compound,
                data,
            } => {
                let body = build_sales_tax_period_body(
                    sales_tax_name.clone(),
                    sales_tax_registration_status.clone(),
                    sales_tax_rate_1.clone(),
                    sales_tax_rate_2.clone(),
                    sales_tax_rate_3.clone(),
                    *sales_tax_is_value_added,
                    sales_tax_registration_number.clone(),
                    effective_date.clone(),
                    second_sales_tax_name.clone(),
                    second_sales_tax_rate_1.clone(),
                    second_sales_tax_rate_2.clone(),
                    second_sales_tax_rate_3.clone(),
                    *second_sales_tax_is_compound,
                    data.clone(),
                    false,
                )?;
                let result = client.put(&format!("sales_tax_periods/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("sales_tax_periods/{}", id)).await?;
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
    fn sales_tax_period_requires_core_fields() {
        let result = build_sales_tax_period_body(
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
            None,
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn sales_tax_period_rejects_invalid_status() {
        let result = build_sales_tax_period_body(
            Some("VAT".to_string()),
            Some("Bad".to_string()),
            Some("20.0".to_string()),
            None,
            None,
            Some(true),
            None,
            Some("2024-01-01".to_string()),
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
    fn sales_tax_period_rejects_data_and_structured() {
        let result = build_sales_tax_period_body(
            Some("VAT".to_string()),
            Some("Registered".to_string()),
            Some("20.0".to_string()),
            None,
            None,
            Some(true),
            None,
            Some("2024-01-01".to_string()),
            None,
            None,
            None,
            None,
            None,
            Some("{\"sales_tax_period\":{}}".to_string()),
            true,
        );
        assert!(result.is_err());
    }
}
