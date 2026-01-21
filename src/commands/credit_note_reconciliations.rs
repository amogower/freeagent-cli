//! Credit note reconciliation commands.

use anyhow::{bail, Result};
use clap::Subcommand;
use serde_json::Value;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

fn build_reconciliation_body(
    credit_note: Option<String>,
    invoice: Option<String>,
    gross_value: Option<String>,
    dated_on: Option<String>,
    exchange_rate: Option<String>,
    data: Option<String>,
    require_core: bool,
) -> Result<Value> {
    let using_structured = credit_note.is_some()
        || invoice.is_some()
        || gross_value.is_some()
        || dated_on.is_some()
        || exchange_rate.is_some();

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
            if require_core {
                if credit_note.is_none() || invoice.is_none() || gross_value.is_none() {
                    bail!("Missing required fields: --credit-note, --invoice, --gross-value");
                }
            }
            let mut reconciliation = serde_json::Map::new();
            if let Some(v) = credit_note {
                reconciliation.insert("credit_note".to_string(), serde_json::json!(v));
            }
            if let Some(v) = invoice {
                reconciliation.insert("invoice".to_string(), serde_json::json!(v));
            }
            if let Some(v) = gross_value {
                reconciliation.insert("gross_value".to_string(), serde_json::json!(v));
            }
            if let Some(v) = dated_on {
                reconciliation.insert("dated_on".to_string(), serde_json::json!(v));
            }
            if let Some(v) = exchange_rate {
                reconciliation.insert("exchange_rate".to_string(), serde_json::json!(v));
            }
            Ok(serde_json::json!({ "credit_note_reconciliation": reconciliation }))
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum CreditNoteReconciliationCommands {
    /// List credit note reconciliations
    List {
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

    /// Get a credit note reconciliation by ID
    Get {
        /// Reconciliation ID
        id: String,
    },

    /// Create a credit note reconciliation (JSON payload)
    Create {
        /// Credit note URL
        #[arg(long)]
        credit_note: Option<String>,

        /// Invoice URL
        #[arg(long)]
        invoice: Option<String>,

        /// Gross value
        #[arg(long)]
        gross_value: Option<String>,

        /// Reconciliation date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,

        /// Exchange rate
        #[arg(long)]
        exchange_rate: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Update a credit note reconciliation (JSON payload)
    Update {
        /// Reconciliation ID
        id: String,

        /// Credit note URL
        #[arg(long)]
        credit_note: Option<String>,

        /// Invoice URL
        #[arg(long)]
        invoice: Option<String>,

        /// Gross value
        #[arg(long)]
        gross_value: Option<String>,

        /// Reconciliation date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,

        /// Exchange rate
        #[arg(long)]
        exchange_rate: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Delete a credit note reconciliation
    Delete {
        /// Reconciliation ID
        id: String,

        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl CreditNoteReconciliationCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List {
                from_date,
                to_date,
                updated_since,
                page,
                per_page,
            } => {
                let params = QueryBuilder::new()
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .add("updated_since", updated_since.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("credit_note_reconciliations", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client
                    .get(&format!("credit_note_reconciliations/{}", id), None)
                    .await?;
                print_output(&result, format);
            }
            Self::Create {
                credit_note,
                invoice,
                gross_value,
                dated_on,
                exchange_rate,
                data,
            } => {
                let body = build_reconciliation_body(
                    credit_note.clone(),
                    invoice.clone(),
                    gross_value.clone(),
                    dated_on.clone(),
                    exchange_rate.clone(),
                    data.clone(),
                    true,
                )?;
                let result = client.post("credit_note_reconciliations", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                id,
                credit_note,
                invoice,
                gross_value,
                dated_on,
                exchange_rate,
                data,
            } => {
                let body = build_reconciliation_body(
                    credit_note.clone(),
                    invoice.clone(),
                    gross_value.clone(),
                    dated_on.clone(),
                    exchange_rate.clone(),
                    data.clone(),
                    false,
                )?;
                let result = client
                    .put(&format!("credit_note_reconciliations/{}", id), Some(body))
                    .await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client
                    .delete(&format!("credit_note_reconciliations/{}", id))
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

    #[test]
    fn reconciliation_requires_core_fields() {
        let result = build_reconciliation_body(
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
    fn reconciliation_rejects_data_and_structured() {
        let result = build_reconciliation_body(
            Some("credit_note".to_string()),
            Some("invoice".to_string()),
            Some("10.00".to_string()),
            None,
            None,
            Some("{\"credit_note_reconciliation\":{}}".to_string()),
            true,
        );
        assert!(result.is_err());
    }
}
