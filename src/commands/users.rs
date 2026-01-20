//! User commands.

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum UserCommands {
    /// List all users
    List {
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get current user
    Me,
    
    /// Get a user by ID
    Get {
        /// User ID
        id: String,
    },
    
    /// Create a new user
    Create {
        /// First name
        #[arg(long)]
        first_name: String,
        
        /// Last name
        #[arg(long)]
        last_name: String,
        
        /// Email address
        #[arg(long)]
        email: String,
        
        /// User role
        #[arg(long)]
        role: Option<String>,
        
        /// Permission level (0-8)
        #[arg(long)]
        permission_level: Option<i32>,
        
        /// Opening mileage
        #[arg(long)]
        opening_mileage: Option<String>,
    },
    
    /// Update a user
    Update {
        /// User ID
        id: String,
        
        /// First name
        #[arg(long)]
        first_name: Option<String>,
        
        /// Last name
        #[arg(long)]
        last_name: Option<String>,
        
        /// Email address
        #[arg(long)]
        email: Option<String>,
        
        /// User role
        #[arg(long)]
        role: Option<String>,
        
        /// Permission level (0-8)
        #[arg(long)]
        permission_level: Option<i32>,
    },
    
    /// Delete a user
    Delete {
        /// User ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl UserCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List { page, per_page } => {
                let params = QueryBuilder::new()
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("users", params).await?;
                print_output(&result, format);
            }
            Self::Me => {
                let result = client.get("users/me", None).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("users/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                first_name,
                last_name,
                email,
                role,
                permission_level,
                opening_mileage,
            } => {
                let mut user = serde_json::Map::new();
                user.insert("first_name".to_string(), json!(first_name));
                user.insert("last_name".to_string(), json!(last_name));
                user.insert("email".to_string(), json!(email));
                
                if let Some(v) = role {
                    user.insert("role".to_string(), json!(v));
                }
                if let Some(v) = permission_level {
                    user.insert("permission_level".to_string(), json!(v));
                }
                if let Some(v) = opening_mileage {
                    user.insert("opening_mileage".to_string(), json!(v));
                }
                
                let body = json!({ "user": user });
                let result = client.post("users", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                id,
                first_name,
                last_name,
                email,
                role,
                permission_level,
            } => {
                let mut user = serde_json::Map::new();
                
                if let Some(v) = first_name {
                    user.insert("first_name".to_string(), json!(v));
                }
                if let Some(v) = last_name {
                    user.insert("last_name".to_string(), json!(v));
                }
                if let Some(v) = email {
                    user.insert("email".to_string(), json!(v));
                }
                if let Some(v) = role {
                    user.insert("role".to_string(), json!(v));
                }
                if let Some(v) = permission_level {
                    user.insert("permission_level".to_string(), json!(v));
                }
                
                let body = json!({ "user": user });
                let result = client.put(&format!("users/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("users/{}", id)).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
