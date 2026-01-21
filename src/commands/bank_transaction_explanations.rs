//! Bank transaction explanation commands.

use anyhow::{bail, Result};
use clap::Subcommand;
use serde_json::Value;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

fn build_explanation_body(
    bank_account: Option<String>,
    bank_transaction: Option<String>,
    dated_on: Option<String>,
    gross_value: Option<String>,
    description: Option<String>,
    category: Option<String>,
    sales_tax_rate: Option<String>,
    sales_tax_value: Option<String>,
    sales_tax_status: Option<String>,
    second_sales_tax_rate: Option<String>,
    second_sales_tax_value: Option<String>,
    second_sales_tax_status: Option<String>,
    ec_status: Option<String>,
    place_of_supply: Option<String>,
    cheque_number: Option<String>,
    project: Option<String>,
    rebill_type: Option<String>,
    rebill_factor: Option<String>,
    receipt_reference: Option<String>,
    paid_invoice: Option<String>,
    paid_bill: Option<String>,
    paid_user: Option<String>,
    transfer_bank_account: Option<String>,
    stock_item: Option<String>,
    stock_altering_quantity: Option<i32>,
    disposed_asset: Option<String>,
    property: Option<String>,
    direct_contact: Option<String>,
    attachment_json: Option<String>,
    capital_asset_json: Option<String>,
    data: Option<String>,
    require_core: bool,
) -> Result<Value> {
    let using_structured = bank_account.is_some()
        || bank_transaction.is_some()
        || dated_on.is_some()
        || gross_value.is_some()
        || description.is_some()
        || category.is_some()
        || sales_tax_rate.is_some()
        || sales_tax_value.is_some()
        || sales_tax_status.is_some()
        || second_sales_tax_rate.is_some()
        || second_sales_tax_value.is_some()
        || second_sales_tax_status.is_some()
        || ec_status.is_some()
        || place_of_supply.is_some()
        || cheque_number.is_some()
        || project.is_some()
        || rebill_type.is_some()
        || rebill_factor.is_some()
        || receipt_reference.is_some()
        || paid_invoice.is_some()
        || paid_bill.is_some()
        || paid_user.is_some()
        || transfer_bank_account.is_some()
        || stock_item.is_some()
        || stock_altering_quantity.is_some()
        || disposed_asset.is_some()
        || property.is_some()
        || direct_contact.is_some()
        || attachment_json.is_some()
        || capital_asset_json.is_some();

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
            if let Some(ref rebill) = rebill_type {
                let allowed = ["cost", "markup", "price"];
                if !allowed.contains(&rebill.as_str()) {
                    bail!("--rebill-type must be one of: cost, markup, price");
                }
            }
            if require_core {
                if bank_account.is_none() && bank_transaction.is_none() {
                    bail!("Provide either --bank-account or --bank-transaction");
                }
                if dated_on.is_none() || gross_value.is_none() {
                    bail!("Missing required fields: --dated-on and --gross-value");
                }
            }

            let mut explanation = serde_json::Map::new();
            if let Some(v) = bank_account {
                explanation.insert("bank_account".to_string(), serde_json::json!(v));
            }
            if let Some(v) = bank_transaction {
                explanation.insert("bank_transaction".to_string(), serde_json::json!(v));
            }
            if let Some(v) = dated_on {
                explanation.insert("dated_on".to_string(), serde_json::json!(v));
            }
            if let Some(v) = gross_value {
                explanation.insert("gross_value".to_string(), serde_json::json!(v));
            }
            if let Some(v) = description {
                explanation.insert("description".to_string(), serde_json::json!(v));
            }
            if let Some(v) = category {
                explanation.insert("category".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_rate {
                explanation.insert("sales_tax_rate".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_value {
                explanation.insert("sales_tax_value".to_string(), serde_json::json!(v));
            }
            if let Some(v) = sales_tax_status {
                explanation.insert("sales_tax_status".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_rate {
                explanation.insert("second_sales_tax_rate".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_value {
                explanation.insert("second_sales_tax_value".to_string(), serde_json::json!(v));
            }
            if let Some(v) = second_sales_tax_status {
                explanation.insert("second_sales_tax_status".to_string(), serde_json::json!(v));
            }
            if let Some(v) = ec_status {
                explanation.insert("ec_status".to_string(), serde_json::json!(v));
            }
            if let Some(v) = place_of_supply {
                explanation.insert("place_of_supply".to_string(), serde_json::json!(v));
            }
            if let Some(v) = cheque_number {
                explanation.insert("cheque_number".to_string(), serde_json::json!(v));
            }
            if let Some(v) = project {
                explanation.insert("project".to_string(), serde_json::json!(v));
            }
            if let Some(v) = rebill_type {
                explanation.insert("rebill_type".to_string(), serde_json::json!(v));
            }
            if let Some(v) = rebill_factor {
                explanation.insert("rebill_factor".to_string(), serde_json::json!(v));
            }
            if let Some(v) = receipt_reference {
                explanation.insert("receipt_reference".to_string(), serde_json::json!(v));
            }
            if let Some(v) = paid_invoice {
                explanation.insert("paid_invoice".to_string(), serde_json::json!(v));
            }
            if let Some(v) = paid_bill {
                explanation.insert("paid_bill".to_string(), serde_json::json!(v));
            }
            if let Some(v) = paid_user {
                explanation.insert("paid_user".to_string(), serde_json::json!(v));
            }
            if let Some(v) = transfer_bank_account {
                explanation.insert("transfer_bank_account".to_string(), serde_json::json!(v));
            }
            if let Some(v) = stock_item {
                explanation.insert("stock_item".to_string(), serde_json::json!(v));
            }
            if let Some(v) = stock_altering_quantity {
                explanation.insert("stock_altering_quantity".to_string(), serde_json::json!(v));
            }
            if let Some(v) = disposed_asset {
                explanation.insert("disposed_asset".to_string(), serde_json::json!(v));
            }
            if let Some(v) = property {
                explanation.insert("property".to_string(), serde_json::json!(v));
            }
            if let Some(v) = direct_contact {
                explanation.insert("direct_contact".to_string(), serde_json::json!(v));
            }
            if let Some(raw) = attachment_json {
                let attachment: Value = serde_json::from_str(&raw)?;
                explanation.insert("attachment".to_string(), attachment);
            }
            if let Some(raw) = capital_asset_json {
                let capital_asset: Value = serde_json::from_str(&raw)?;
                explanation.insert("capital_asset".to_string(), capital_asset);
            }

            Ok(serde_json::json!({ "bank_transaction_explanation": explanation }))
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum BankTransactionExplanationCommands {
    /// List bank transaction explanations
    List {
        /// Bank account URL (required)
        #[arg(long)]
        bank_account: String,

        /// Filter from date (YYYY-MM-DD)
        #[arg(long)]
        from_date: Option<String>,

        /// Filter to date (YYYY-MM-DD)
        #[arg(long)]
        to_date: Option<String>,

        /// Filter by update date (ISO 8601)
        #[arg(long)]
        updated_since: Option<String>,

        /// Page number
        #[arg(long)]
        page: Option<i32>,

        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },

    /// Get a bank transaction explanation by ID
    Get {
        /// Explanation ID
        id: String,
    },

    /// Create a bank transaction explanation (JSON payload)
    Create {
        /// Bank account URL
        #[arg(long)]
        bank_account: Option<String>,

        /// Bank transaction URL
        #[arg(long)]
        bank_transaction: Option<String>,

        /// Explanation date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,

        /// Gross value
        #[arg(long)]
        gross_value: Option<String>,

        /// Description
        #[arg(long)]
        description: Option<String>,

        /// Category URL
        #[arg(long)]
        category: Option<String>,

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

        /// EC status
        #[arg(long)]
        ec_status: Option<String>,

        /// Place of supply (for EC VAT MOSS)
        #[arg(long)]
        place_of_supply: Option<String>,

        /// Cheque number
        #[arg(long)]
        cheque_number: Option<String>,

        /// Project URL
        #[arg(long)]
        project: Option<String>,

        /// Rebill type (cost, markup, price)
        #[arg(long)]
        rebill_type: Option<String>,

        /// Rebill factor
        #[arg(long)]
        rebill_factor: Option<String>,

        /// Receipt reference
        #[arg(long)]
        receipt_reference: Option<String>,

        /// Paid invoice URL
        #[arg(long)]
        paid_invoice: Option<String>,

        /// Paid bill URL
        #[arg(long)]
        paid_bill: Option<String>,

        /// Paid user URL
        #[arg(long)]
        paid_user: Option<String>,

        /// Transfer bank account URL
        #[arg(long)]
        transfer_bank_account: Option<String>,

        /// Stock item URL
        #[arg(long)]
        stock_item: Option<String>,

        /// Stock altering quantity
        #[arg(long)]
        stock_altering_quantity: Option<i32>,

        /// Disposed asset URL
        #[arg(long)]
        disposed_asset: Option<String>,

        /// Property URL
        #[arg(long)]
        property: Option<String>,

        /// Direct contact URL
        #[arg(long)]
        direct_contact: Option<String>,

        /// Attachment as JSON
        #[arg(long)]
        attachment_json: Option<String>,

        /// Capital asset as JSON (for depreciation profile)
        #[arg(long)]
        capital_asset_json: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Update a bank transaction explanation (JSON payload)
    Update {
        /// Explanation ID
        id: String,

        /// Bank account URL
        #[arg(long)]
        bank_account: Option<String>,

        /// Bank transaction URL
        #[arg(long)]
        bank_transaction: Option<String>,

        /// Explanation date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,

        /// Gross value
        #[arg(long)]
        gross_value: Option<String>,

        /// Description
        #[arg(long)]
        description: Option<String>,

        /// Category URL
        #[arg(long)]
        category: Option<String>,

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

        /// EC status
        #[arg(long)]
        ec_status: Option<String>,

        /// Place of supply (for EC VAT MOSS)
        #[arg(long)]
        place_of_supply: Option<String>,

        /// Cheque number
        #[arg(long)]
        cheque_number: Option<String>,

        /// Project URL
        #[arg(long)]
        project: Option<String>,

        /// Rebill type (cost, markup, price)
        #[arg(long)]
        rebill_type: Option<String>,

        /// Rebill factor
        #[arg(long)]
        rebill_factor: Option<String>,

        /// Receipt reference
        #[arg(long)]
        receipt_reference: Option<String>,

        /// Paid invoice URL
        #[arg(long)]
        paid_invoice: Option<String>,

        /// Paid bill URL
        #[arg(long)]
        paid_bill: Option<String>,

        /// Paid user URL
        #[arg(long)]
        paid_user: Option<String>,

        /// Transfer bank account URL
        #[arg(long)]
        transfer_bank_account: Option<String>,

        /// Stock item URL
        #[arg(long)]
        stock_item: Option<String>,

        /// Stock altering quantity
        #[arg(long)]
        stock_altering_quantity: Option<i32>,

        /// Disposed asset URL
        #[arg(long)]
        disposed_asset: Option<String>,

        /// Property URL
        #[arg(long)]
        property: Option<String>,

        /// Direct contact URL
        #[arg(long)]
        direct_contact: Option<String>,

        /// Attachment as JSON
        #[arg(long)]
        attachment_json: Option<String>,

        /// Capital asset as JSON (for depreciation profile)
        #[arg(long)]
        capital_asset_json: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Delete a bank transaction explanation
    Delete {
        /// Explanation ID
        id: String,

        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl BankTransactionExplanationCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List {
                bank_account,
                from_date,
                to_date,
                updated_since,
                page,
                per_page,
            } => {
                let params = QueryBuilder::new()
                    .add("bank_account", Some(bank_account.clone()))
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .add("updated_since", updated_since.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("bank_transaction_explanations", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client
                    .get(&format!("bank_transaction_explanations/{}", id), None)
                    .await?;
                print_output(&result, format);
            }
            Self::Create {
                bank_account,
                bank_transaction,
                dated_on,
                gross_value,
                description,
                category,
                sales_tax_rate,
                sales_tax_value,
                sales_tax_status,
                second_sales_tax_rate,
                second_sales_tax_value,
                second_sales_tax_status,
                ec_status,
                place_of_supply,
                cheque_number,
                project,
                rebill_type,
                rebill_factor,
                receipt_reference,
                paid_invoice,
                paid_bill,
                paid_user,
                transfer_bank_account,
                stock_item,
                stock_altering_quantity,
                disposed_asset,
                property,
                direct_contact,
                attachment_json,
                capital_asset_json,
                data,
            } => {
                let body = build_explanation_body(
                    bank_account.clone(),
                    bank_transaction.clone(),
                    dated_on.clone(),
                    gross_value.clone(),
                    description.clone(),
                    category.clone(),
                    sales_tax_rate.clone(),
                    sales_tax_value.clone(),
                    sales_tax_status.clone(),
                    second_sales_tax_rate.clone(),
                    second_sales_tax_value.clone(),
                    second_sales_tax_status.clone(),
                    ec_status.clone(),
                    place_of_supply.clone(),
                    cheque_number.clone(),
                    project.clone(),
                    rebill_type.clone(),
                    rebill_factor.clone(),
                    receipt_reference.clone(),
                    paid_invoice.clone(),
                    paid_bill.clone(),
                    paid_user.clone(),
                    transfer_bank_account.clone(),
                    stock_item.clone(),
                    stock_altering_quantity.clone(),
                    disposed_asset.clone(),
                    property.clone(),
                    direct_contact.clone(),
                    attachment_json.clone(),
                    capital_asset_json.clone(),
                    data.clone(),
                    true,
                )?;
                let result = client.post("bank_transaction_explanations", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                id,
                bank_account,
                bank_transaction,
                dated_on,
                gross_value,
                description,
                category,
                sales_tax_rate,
                sales_tax_value,
                sales_tax_status,
                second_sales_tax_rate,
                second_sales_tax_value,
                second_sales_tax_status,
                ec_status,
                place_of_supply,
                cheque_number,
                project,
                rebill_type,
                rebill_factor,
                receipt_reference,
                paid_invoice,
                paid_bill,
                paid_user,
                transfer_bank_account,
                stock_item,
                stock_altering_quantity,
                disposed_asset,
                property,
                direct_contact,
                attachment_json,
                capital_asset_json,
                data,
            } => {
                let body = build_explanation_body(
                    bank_account.clone(),
                    bank_transaction.clone(),
                    dated_on.clone(),
                    gross_value.clone(),
                    description.clone(),
                    category.clone(),
                    sales_tax_rate.clone(),
                    sales_tax_value.clone(),
                    sales_tax_status.clone(),
                    second_sales_tax_rate.clone(),
                    second_sales_tax_value.clone(),
                    second_sales_tax_status.clone(),
                    ec_status.clone(),
                    place_of_supply.clone(),
                    cheque_number.clone(),
                    project.clone(),
                    rebill_type.clone(),
                    rebill_factor.clone(),
                    receipt_reference.clone(),
                    paid_invoice.clone(),
                    paid_bill.clone(),
                    paid_user.clone(),
                    transfer_bank_account.clone(),
                    stock_item.clone(),
                    stock_altering_quantity.clone(),
                    disposed_asset.clone(),
                    property.clone(),
                    direct_contact.clone(),
                    attachment_json.clone(),
                    capital_asset_json.clone(),
                    data.clone(),
                    false,
                )?;
                let result = client
                    .put(&format!("bank_transaction_explanations/{}", id), Some(body))
                    .await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client
                    .delete(&format!("bank_transaction_explanations/{}", id))
                    .await?;
                print_output(&result, format);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_minimal(
        bank_account: Option<&str>,
        bank_transaction: Option<&str>,
    ) -> Result<Value> {
        build_explanation_body(
            bank_account.map(|s| s.to_string()),
            bank_transaction.map(|s| s.to_string()),
            Some("2024-01-01".to_string()),
            Some("10.00".to_string()),
            None, // description
            None, // category
            None, // sales_tax_rate
            None, // sales_tax_value
            None, // sales_tax_status
            None, // second_sales_tax_rate
            None, // second_sales_tax_value
            None, // second_sales_tax_status
            None, // ec_status
            None, // place_of_supply
            None, // cheque_number
            None, // project
            None, // rebill_type
            None, // rebill_factor
            None, // receipt_reference
            None, // paid_invoice
            None, // paid_bill
            None, // paid_user
            None, // transfer_bank_account
            None, // stock_item
            None, // stock_altering_quantity
            None, // disposed_asset
            None, // property
            None, // direct_contact
            None, // attachment_json
            None, // capital_asset_json
            None, // data
            true,
        )
    }

    #[test]
    fn explanation_requires_account_or_transaction() {
        let result = build_minimal(None, None);
        assert!(result.is_err());
    }

    #[test]
    fn explanation_rejects_invalid_sales_tax_status() {
        let result = build_explanation_body(
            Some("bank_account".to_string()),
            None,
            Some("2024-01-01".to_string()),
            Some("10.00".to_string()),
            None, // description
            None, // category
            None, // sales_tax_rate
            None, // sales_tax_value
            Some("BAD".to_string()), // sales_tax_status
            None, // second_sales_tax_rate
            None, // second_sales_tax_value
            None, // second_sales_tax_status
            None, // ec_status
            None, // place_of_supply
            None, // cheque_number
            None, // project
            None, // rebill_type
            None, // rebill_factor
            None, // receipt_reference
            None, // paid_invoice
            None, // paid_bill
            None, // paid_user
            None, // transfer_bank_account
            None, // stock_item
            None, // stock_altering_quantity
            None, // disposed_asset
            None, // property
            None, // direct_contact
            None, // attachment_json
            None, // capital_asset_json
            None, // data
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn explanation_rejects_data_and_structured() {
        let result = build_explanation_body(
            Some("bank_account".to_string()),
            None,
            Some("2024-01-01".to_string()),
            Some("10.00".to_string()),
            Some("Desc".to_string()),
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
            Some("{\"bank_transaction_explanation\":{}}".to_string()),
            true,
        );
        assert!(result.is_err());
    }
}
