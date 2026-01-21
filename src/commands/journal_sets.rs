//! Journal set commands.

use anyhow::{bail, Result};
use clap::Subcommand;
use serde_json::Value;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

fn build_journal_set_body(
    description: Option<String>,
    dated_on: Option<String>,
    tag: Option<String>,
    journal_entries_json: Option<String>,
    data: Option<String>,
    require_core: bool,
) -> Result<Value> {
    let using_structured =
        description.is_some() || dated_on.is_some() || tag.is_some() || journal_entries_json.is_some();
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
            if require_core && (description.is_none() || journal_entries_json.is_none()) {
                bail!("Missing required fields: --description and --journal-entries-json");
            }
            let mut journal_set = serde_json::Map::new();
            if let Some(v) = description {
                journal_set.insert("description".to_string(), serde_json::json!(v));
            }
            if let Some(v) = dated_on {
                journal_set.insert("dated_on".to_string(), serde_json::json!(v));
            }
            if let Some(v) = tag {
                journal_set.insert("tag".to_string(), serde_json::json!(v));
            }
            if let Some(raw) = journal_entries_json {
                let entries: Value = serde_json::from_str(&raw)?;
                if !entries.is_array() {
                    bail!("--journal-entries-json must be a JSON array");
                }
                journal_set.insert("journal_entries".to_string(), entries);
            }
            Ok(serde_json::json!({ "journal_set": journal_set }))
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum JournalSetCommands {
    /// List journal sets
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

        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,

        /// Page number
        #[arg(long)]
        page: Option<i32>,

        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },

    /// Get a journal set by ID
    Get {
        /// Journal set ID
        id: String,
    },

    /// Get opening balances
    OpeningBalances,

    /// Create a journal set (JSON payload)
    Create {
        /// Description (required)
        #[arg(long)]
        description: Option<String>,

        /// Dated on (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,

        /// Tag
        #[arg(long)]
        tag: Option<String>,

        /// Journal entries as JSON array
        #[arg(long)]
        journal_entries_json: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Update a journal set (JSON payload)
    Update {
        /// Journal set ID
        id: String,

        /// Description
        #[arg(long)]
        description: Option<String>,

        /// Dated on (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,

        /// Tag
        #[arg(long)]
        tag: Option<String>,

        /// Journal entries as JSON array
        #[arg(long)]
        journal_entries_json: Option<String>,

        /// JSON body for the request
        #[arg(long)]
        data: Option<String>,
    },

    /// Delete a journal set
    Delete {
        /// Journal set ID
        id: String,

        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl JournalSetCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List {
                from_date,
                to_date,
                updated_since,
                tag,
                page,
                per_page,
            } => {
                let params = QueryBuilder::new()
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .add("updated_since", updated_since.clone())
                    .add("tag", tag.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("journal_sets", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("journal_sets/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::OpeningBalances => {
                let result = client.get("journal_sets/opening_balances", None).await?;
                print_output(&result, format);
            }
            Self::Create {
                description,
                dated_on,
                tag,
                journal_entries_json,
                data,
            } => {
                let body = build_journal_set_body(
                    description.clone(),
                    dated_on.clone(),
                    tag.clone(),
                    journal_entries_json.clone(),
                    data.clone(),
                    true,
                )?;
                let result = client.post("journal_sets", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                id,
                description,
                dated_on,
                tag,
                journal_entries_json,
                data,
            } => {
                let body = build_journal_set_body(
                    description.clone(),
                    dated_on.clone(),
                    tag.clone(),
                    journal_entries_json.clone(),
                    data.clone(),
                    false,
                )?;
                let result = client.put(&format!("journal_sets/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("journal_sets/{}", id)).await?;
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
    fn journal_set_requires_description_and_entries() {
        let result = build_journal_set_body(None, None, None, None, None, true);
        assert!(result.is_err());
    }

    #[test]
    fn journal_entries_must_be_array() {
        let result = build_journal_set_body(
            Some("Test".to_string()),
            None,
            None,
            Some("{\"not\":\"array\"}".to_string()),
            None,
            true,
        );
        assert!(result.is_err());
    }

    #[test]
    fn journal_set_rejects_data_and_structured() {
        let result = build_journal_set_body(
            Some("Test".to_string()),
            None,
            None,
            Some("[]".to_string()),
            Some("{\"journal_set\":{}}".to_string()),
            true,
        );
        assert!(result.is_err());
    }
}
