//! Task commands.

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum TaskCommands {
    /// List all tasks
    List {
        /// Filter by project URL
        #[arg(long)]
        project: Option<String>,
        
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get a task by ID
    Get {
        /// Task ID
        id: String,
    },
    
    /// Create a new task
    Create {
        /// Project URL (required)
        #[arg(long)]
        project: String,
        
        /// Task name (required)
        #[arg(long)]
        name: String,
        
        /// Billable (default: true)
        #[arg(long)]
        billable: Option<bool>,
        
        /// Billing rate
        #[arg(long)]
        billing_rate: Option<String>,
        
        /// Billing period
        #[arg(long)]
        billing_period: Option<String>,
    },
    
    /// Update a task
    Update {
        /// Task ID
        id: String,
        
        /// Task name
        #[arg(long)]
        name: Option<String>,
        
        /// Billable
        #[arg(long)]
        billable: Option<bool>,
        
        /// Billing rate
        #[arg(long)]
        billing_rate: Option<String>,
    },
    
    /// Delete a task
    Delete {
        /// Task ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl TaskCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { project, page, per_page } => {
                let params = QueryBuilder::new()
                    .add("project", project.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("tasks", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("tasks/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                project,
                name,
                billable,
                billing_rate,
                billing_period,
            } => {
                let mut task = serde_json::Map::new();
                task.insert("project".to_string(), json!(project));
                task.insert("name".to_string(), json!(name));
                
                if let Some(v) = billable {
                    task.insert("is_billable".to_string(), json!(v));
                }
                if let Some(v) = billing_rate {
                    task.insert("billing_rate".to_string(), json!(v));
                }
                if let Some(v) = billing_period {
                    task.insert("billing_period".to_string(), json!(v));
                }
                
                let body = json!({ "task": task });
                let result = client.post("tasks", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, name, billable, billing_rate } => {
                let mut task = serde_json::Map::new();
                
                if let Some(v) = name {
                    task.insert("name".to_string(), json!(v));
                }
                if let Some(v) = billable {
                    task.insert("is_billable".to_string(), json!(v));
                }
                if let Some(v) = billing_rate {
                    task.insert("billing_rate".to_string(), json!(v));
                }
                
                let body = json!({ "task": task });
                let result = client.put(&format!("tasks/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("tasks/{}", id)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
