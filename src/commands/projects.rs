//! Project commands.

use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Clone, ValueEnum)]
pub enum ProjectView {
    Active,
    Completed,
    Cancelled,
    Hidden,
    All,
}

impl ProjectView {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Completed => "completed",
            Self::Cancelled => "cancelled",
            Self::Hidden => "hidden",
            Self::All => "all",
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum BudgetUnits {
    Hours,
    Days,
    Monetary,
}

impl BudgetUnits {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Hours => "Hours",
            Self::Days => "Days",
            Self::Monetary => "Monetary",
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum ProjectCommands {
    /// List all projects
    List {
        /// Filter by view
        #[arg(long)]
        view: Option<ProjectView>,
        
        /// Filter by contact URL
        #[arg(long)]
        contact: Option<String>,
        
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
    
    /// Get a project by ID
    Get {
        /// Project ID
        id: String,
    },
    
    /// Create a new project
    Create {
        /// Contact URL (required)
        #[arg(long)]
        contact: String,
        
        /// Project name (required)
        #[arg(long)]
        name: String,
        
        /// Budget amount
        #[arg(long)]
        budget: Option<String>,
        
        /// Budget units
        #[arg(long)]
        budget_units: Option<BudgetUnits>,
        
        /// Currency code
        #[arg(long)]
        currency: Option<String>,
        
        /// Billing period (hour, day, week, month, year)
        #[arg(long)]
        billing_period: Option<String>,
        
        /// Hours per day
        #[arg(long)]
        hours_per_day: Option<String>,
    },
    
    /// Update a project
    Update {
        /// Project ID
        id: String,
        
        /// Project name
        #[arg(long)]
        name: Option<String>,
        
        /// Budget amount
        #[arg(long)]
        budget: Option<String>,
        
        /// Status
        #[arg(long)]
        status: Option<String>,
    },
    
    /// Delete a project
    Delete {
        /// Project ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
    
    /// List project invoices
    Invoices {
        /// Project ID
        id: String,
    },
    
    /// Get project timeline
    Timeline {
        /// Project ID
        id: String,
    },
}

impl ProjectCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List {
                view,
                contact,
                updated_since,
                page,
                per_page,
            } => {
                let params = QueryBuilder::new()
                    .add("view", view.as_ref().map(|v| v.as_str()))
                    .add("contact", contact.clone())
                    .add("updated_since", updated_since.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("projects", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("projects/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                contact,
                name,
                budget,
                budget_units,
                currency,
                billing_period,
                hours_per_day,
            } => {
                let mut project = serde_json::Map::new();
                project.insert("contact".to_string(), json!(contact));
                project.insert("name".to_string(), json!(name));
                
                if let Some(v) = budget {
                    project.insert("budget".to_string(), json!(v));
                }
                if let Some(v) = budget_units {
                    project.insert("budget_units".to_string(), json!(v.as_str()));
                }
                if let Some(v) = currency {
                    project.insert("currency".to_string(), json!(v));
                }
                if let Some(v) = billing_period {
                    project.insert("billing_period".to_string(), json!(v));
                }
                if let Some(v) = hours_per_day {
                    project.insert("hours_per_day".to_string(), json!(v));
                }
                
                let body = json!({ "project": project });
                let result = client.post("projects", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, name, budget, status } => {
                let mut project = serde_json::Map::new();
                
                if let Some(v) = name {
                    project.insert("name".to_string(), json!(v));
                }
                if let Some(v) = budget {
                    project.insert("budget".to_string(), json!(v));
                }
                if let Some(v) = status {
                    project.insert("status".to_string(), json!(v));
                }
                
                let body = json!({ "project": project });
                let result = client.put(&format!("projects/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("projects/{}", id)).await?;
                print_output(&result, format);
            }
            Self::Invoices { id } => {
                let result = client.get(&format!("projects/{}/invoices", id), None).await?;
                print_output(&result, format);
            }
            Self::Timeline { id } => {
                let result = client.get(&format!("projects/{}/timeline", id), None).await?;
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
    fn project_view_as_str_matches_api() {
        assert_eq!(ProjectView::Active.as_str(), "active");
        assert_eq!(ProjectView::Cancelled.as_str(), "cancelled");
        assert_eq!(ProjectView::All.as_str(), "all");
    }

    #[test]
    fn budget_units_as_str_matches_api() {
        assert_eq!(BudgetUnits::Hours.as_str(), "Hours");
        assert_eq!(BudgetUnits::Days.as_str(), "Days");
        assert_eq!(BudgetUnits::Monetary.as_str(), "Monetary");
    }
}
