//! Timeslip commands.

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum TimeslipCommands {
    /// List all timeslips
    List {
        /// Filter by user URL
        #[arg(long)]
        user: Option<String>,
        
        /// Filter by project URL
        #[arg(long)]
        project: Option<String>,
        
        /// Filter by task URL
        #[arg(long)]
        task: Option<String>,
        
        /// Filter from date (YYYY-MM-DD)
        #[arg(long)]
        from_date: Option<String>,
        
        /// Filter to date (YYYY-MM-DD)
        #[arg(long)]
        to_date: Option<String>,
        
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get a timeslip by ID
    Get {
        /// Timeslip ID
        id: String,
    },
    
    /// Create a new timeslip
    Create {
        /// User URL (required)
        #[arg(long)]
        user: String,
        
        /// Project URL (required)
        #[arg(long)]
        project: String,
        
        /// Task URL (required)
        #[arg(long)]
        task: String,
        
        /// Date (YYYY-MM-DD) (required)
        #[arg(long)]
        dated_on: String,
        
        /// Hours worked
        #[arg(long)]
        hours: Option<String>,
        
        /// Minutes worked
        #[arg(long)]
        minutes: Option<i32>,
        
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    
    /// Update a timeslip
    Update {
        /// Timeslip ID
        id: String,
        
        /// Date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,
        
        /// Hours worked
        #[arg(long)]
        hours: Option<String>,
        
        /// Comment
        #[arg(long)]
        comment: Option<String>,
    },
    
    /// Delete a timeslip
    Delete {
        /// Timeslip ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
    
    /// Start timer for a timeslip
    StartTimer {
        /// Timeslip ID
        id: String,
    },
    
    /// Stop timer for a timeslip
    StopTimer {
        /// Timeslip ID
        id: String,
    },
}

impl TimeslipCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List {
                user,
                project,
                task,
                from_date,
                to_date,
                page,
                per_page,
            } => {
                let params = QueryBuilder::new()
                    .add("user", user.clone())
                    .add("project", project.clone())
                    .add("task", task.clone())
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("timeslips", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("timeslips/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                user,
                project,
                task,
                dated_on,
                hours,
                minutes,
                comment,
            } => {
                let mut timeslip = serde_json::Map::new();
                timeslip.insert("user".to_string(), json!(user));
                timeslip.insert("project".to_string(), json!(project));
                timeslip.insert("task".to_string(), json!(task));
                timeslip.insert("dated_on".to_string(), json!(dated_on));
                
                if let Some(v) = hours {
                    timeslip.insert("hours".to_string(), json!(v));
                }
                if let Some(v) = minutes {
                    timeslip.insert("minutes".to_string(), json!(v));
                }
                if let Some(v) = comment {
                    timeslip.insert("comment".to_string(), json!(v));
                }
                
                let body = json!({ "timeslip": timeslip });
                let result = client.post("timeslips", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update { id, dated_on, hours, comment } => {
                let mut timeslip = serde_json::Map::new();
                
                if let Some(v) = dated_on {
                    timeslip.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = hours {
                    timeslip.insert("hours".to_string(), json!(v));
                }
                if let Some(v) = comment {
                    timeslip.insert("comment".to_string(), json!(v));
                }
                
                let body = json!({ "timeslip": timeslip });
                let result = client.put(&format!("timeslips/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("timeslips/{}", id)).await?;
                print_output(&result, format);
            }
            Self::StartTimer { id } => {
                let result = client.put(&format!("timeslips/{}/timer", id), None::<()>).await?;
                print_output(&result, format);
            }
            Self::StopTimer { id } => {
                let result = client.delete(&format!("timeslips/{}/timer", id)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
