//! Category commands.

use anyhow::{bail, Result};
use clap::Subcommand;
use serde_json::Value;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

fn validate_category_group(group: &str) -> Result<()> {
    let allowed = ["income", "cost_of_sales", "admin_expenses"];
    if !allowed.contains(&group) {
        bail!("--category-group must be one of: income, cost_of_sales, admin_expenses");
    }
    Ok(())
}

#[derive(Debug, Subcommand)]
pub enum CategoryCommands {
    /// List all categories
    List {
        /// Include sub accounts in the list
        #[arg(long)]
        sub_accounts: bool,
    },
    
    /// Get a category by nominal code
    Get {
        /// Category nominal code
        nominal_code: String,
    },

    /// Create a new category (JSON payload)
    Create {
        /// Category description
        #[arg(long)]
        description: Option<String>,

        /// Category nominal code
        #[arg(long)]
        nominal_code: Option<String>,

        /// Category group (income, cost_of_sales, admin_expenses)
        #[arg(long)]
        category_group: Option<String>,

        /// Tax reporting name
        #[arg(long)]
        tax_reporting_name: Option<String>,

        /// Allowable for tax
        #[arg(long)]
        allowable_for_tax: Option<bool>,

        /// Automatic sales tax rate
        #[arg(long)]
        auto_sales_tax_rate: Option<String>,

        /// JSON body for the category request
        #[arg(long)]
        data: Option<String>,
    },

    /// Update a category (JSON payload)
    Update {
        /// Category nominal code
        nominal_code: String,

        /// Category description
        #[arg(long)]
        description: Option<String>,

        /// Category group (income, cost_of_sales, admin_expenses)
        #[arg(long)]
        category_group: Option<String>,

        /// Tax reporting name
        #[arg(long)]
        tax_reporting_name: Option<String>,

        /// Allowable for tax
        #[arg(long)]
        allowable_for_tax: Option<bool>,

        /// Automatic sales tax rate
        #[arg(long)]
        auto_sales_tax_rate: Option<String>,

        /// JSON body for the category request
        #[arg(long)]
        data: Option<String>,
    },

    /// Delete a category
    Delete {
        /// Category nominal code
        nominal_code: String,

        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl CategoryCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { sub_accounts } => {
                let params = QueryBuilder::new()
                    .add_bool("sub_accounts", if *sub_accounts { Some(true) } else { None })
                    .build();
                let result = client.get("categories", params).await?;
                print_output(&result, format);
            }
            Self::Get { nominal_code } => {
                let result = client.get(&format!("categories/{}", nominal_code), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                description,
                nominal_code,
                category_group,
                tax_reporting_name,
                allowable_for_tax,
                auto_sales_tax_rate,
                data,
            } => {
                let using_structured = description.is_some()
                    || nominal_code.is_some()
                    || category_group.is_some()
                    || tax_reporting_name.is_some()
                    || allowable_for_tax.is_some()
                    || auto_sales_tax_rate.is_some();

                let body: Value = match data {
                    Some(raw) => {
                        if using_structured {
                            bail!("Use either structured flags or --data, not both");
                        }
                        serde_json::from_str(raw)?
                    }
                    None => {
                        if description.is_none() || nominal_code.is_none() || category_group.is_none() {
                            bail!("Missing required fields: --description, --nominal-code, --category-group");
                        }
                        if let Some(ref group) = category_group {
                            validate_category_group(group)?;
                        }
                        let mut category = serde_json::Map::new();
                        category.insert("description".to_string(), serde_json::json!(description));
                        category.insert("nominal_code".to_string(), serde_json::json!(nominal_code));
                        category.insert("category_group".to_string(), serde_json::json!(category_group));
                        if let Some(v) = tax_reporting_name {
                            category.insert("tax_reporting_name".to_string(), serde_json::json!(v));
                        }
                        if let Some(v) = allowable_for_tax {
                            category.insert("allowable_for_tax".to_string(), serde_json::json!(v));
                        }
                        if let Some(v) = auto_sales_tax_rate {
                            category.insert("auto_sales_tax_rate".to_string(), serde_json::json!(v));
                        }
                        serde_json::json!({ "category": category })
                    }
                };
                let result = client.post("categories", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                nominal_code,
                description,
                category_group,
                tax_reporting_name,
                allowable_for_tax,
                auto_sales_tax_rate,
                data,
            } => {
                let using_structured = description.is_some()
                    || category_group.is_some()
                    || tax_reporting_name.is_some()
                    || allowable_for_tax.is_some()
                    || auto_sales_tax_rate.is_some();

                let body: Value = match data {
                    Some(raw) => {
                        if using_structured {
                            bail!("Use either structured flags or --data, not both");
                        }
                        serde_json::from_str(raw)?
                    }
                    None => {
                        if !using_structured {
                            bail!("Provide at least one field to update or use --data");
                        }
                        if let Some(ref group) = category_group {
                            validate_category_group(group)?;
                        }
                        let mut category = serde_json::Map::new();
                        if let Some(v) = description {
                            category.insert("description".to_string(), serde_json::json!(v));
                        }
                        if let Some(v) = category_group {
                            category.insert("category_group".to_string(), serde_json::json!(v));
                        }
                        if let Some(v) = tax_reporting_name {
                            category.insert("tax_reporting_name".to_string(), serde_json::json!(v));
                        }
                        if let Some(v) = allowable_for_tax {
                            category.insert("allowable_for_tax".to_string(), serde_json::json!(v));
                        }
                        if let Some(v) = auto_sales_tax_rate {
                            category.insert("auto_sales_tax_rate".to_string(), serde_json::json!(v));
                        }
                        serde_json::json!({ "category": category })
                    }
                };
                let result = client.put(&format!("categories/{}", nominal_code), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { nominal_code, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("categories/{}", nominal_code)).await?;
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
    fn category_group_validation_rejects_unknown() {
        let result = validate_category_group("other");
        assert!(result.is_err());
    }
}
