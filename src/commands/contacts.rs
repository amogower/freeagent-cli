//! Contact commands.

use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Clone, ValueEnum)]
pub enum ContactView {
    All,
    Active,
    Clients,
    Suppliers,
    ActiveProjects,
    CompletedProjects,
    OpenClients,
    OpenSuppliers,
    Hidden,
}

impl ContactView {
    fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Active => "active",
            Self::Clients => "clients",
            Self::Suppliers => "suppliers",
            Self::ActiveProjects => "active_projects",
            Self::CompletedProjects => "completed_projects",
            Self::OpenClients => "open_clients",
            Self::OpenSuppliers => "open_suppliers",
            Self::Hidden => "hidden",
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ContactSort {
    Name,
    CreatedAt,
    UpdatedAt,
}

impl ContactSort {
    fn as_str(&self, descending: bool) -> String {
        let base = match self {
            Self::Name => "name",
            Self::CreatedAt => "created_at",
            Self::UpdatedAt => "updated_at",
        };
        if descending {
            format!("-{}", base)
        } else {
            base.to_string()
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum ContactCommands {
    /// List all contacts
    List {
        /// Filter by view
        #[arg(long)]
        view: Option<ContactView>,
        
        /// Sort field
        #[arg(long)]
        sort: Option<ContactSort>,
        
        /// Sort descending
        #[arg(long)]
        desc: bool,
        
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
    
    /// Get a contact by ID
    Get {
        /// Contact ID
        id: String,
    },
    
    /// Create a new contact
    Create {
        /// First name
        #[arg(long)]
        first_name: Option<String>,
        
        /// Last name
        #[arg(long)]
        last_name: Option<String>,
        
        /// Organisation name
        #[arg(long)]
        organisation_name: Option<String>,
        
        /// Email address
        #[arg(long)]
        email: Option<String>,
        
        /// Billing email
        #[arg(long)]
        billing_email: Option<String>,
        
        /// Phone number
        #[arg(long)]
        phone_number: Option<String>,
        
        /// Mobile number
        #[arg(long)]
        mobile: Option<String>,
        
        /// Address line 1
        #[arg(long)]
        address1: Option<String>,
        
        /// Address line 2
        #[arg(long)]
        address2: Option<String>,
        
        /// Town/City
        #[arg(long)]
        town: Option<String>,
        
        /// Region/State
        #[arg(long)]
        region: Option<String>,
        
        /// Postal code
        #[arg(long)]
        postcode: Option<String>,
        
        /// Country
        #[arg(long)]
        country: Option<String>,
        
        /// Default payment terms in days
        #[arg(long)]
        payment_terms_in_days: Option<i32>,
    },
    
    /// Update a contact
    Update {
        /// Contact ID
        id: String,
        
        /// First name
        #[arg(long)]
        first_name: Option<String>,
        
        /// Last name
        #[arg(long)]
        last_name: Option<String>,
        
        /// Organisation name
        #[arg(long)]
        organisation_name: Option<String>,
        
        /// Email address
        #[arg(long)]
        email: Option<String>,
        
        /// Phone number
        #[arg(long)]
        phone_number: Option<String>,
    },
    
    /// Delete a contact
    Delete {
        /// Contact ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
}

impl ContactCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List {
                view,
                sort,
                desc,
                updated_since,
                page,
                per_page,
            } => {
                let params = QueryBuilder::new()
                    .add("view", view.as_ref().map(|v| v.as_str()))
                    .add("sort", sort.as_ref().map(|s| s.as_str(*desc)))
                    .add("updated_since", updated_since.clone())
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("contacts", params).await?;
                print_output(&result, format);
            }
            Self::Get { id } => {
                let result = client.get(&format!("contacts/{}", id), None).await?;
                print_output(&result, format);
            }
            Self::Create {
                first_name,
                last_name,
                organisation_name,
                email,
                billing_email,
                phone_number,
                mobile,
                address1,
                address2,
                town,
                region,
                postcode,
                country,
                payment_terms_in_days,
            } => {
                let mut contact = serde_json::Map::new();
                
                if let Some(v) = first_name {
                    contact.insert("first_name".to_string(), json!(v));
                }
                if let Some(v) = last_name {
                    contact.insert("last_name".to_string(), json!(v));
                }
                if let Some(v) = organisation_name {
                    contact.insert("organisation_name".to_string(), json!(v));
                }
                if let Some(v) = email {
                    contact.insert("email".to_string(), json!(v));
                }
                if let Some(v) = billing_email {
                    contact.insert("billing_email".to_string(), json!(v));
                }
                if let Some(v) = phone_number {
                    contact.insert("phone_number".to_string(), json!(v));
                }
                if let Some(v) = mobile {
                    contact.insert("mobile".to_string(), json!(v));
                }
                if let Some(v) = address1 {
                    contact.insert("address1".to_string(), json!(v));
                }
                if let Some(v) = address2 {
                    contact.insert("address2".to_string(), json!(v));
                }
                if let Some(v) = town {
                    contact.insert("town".to_string(), json!(v));
                }
                if let Some(v) = region {
                    contact.insert("region".to_string(), json!(v));
                }
                if let Some(v) = postcode {
                    contact.insert("postcode".to_string(), json!(v));
                }
                if let Some(v) = country {
                    contact.insert("country".to_string(), json!(v));
                }
                if let Some(v) = payment_terms_in_days {
                    contact.insert("default_payment_terms_in_days".to_string(), json!(v));
                }
                
                let body = json!({ "contact": contact });
                let result = client.post("contacts", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                id,
                first_name,
                last_name,
                organisation_name,
                email,
                phone_number,
            } => {
                let mut contact = serde_json::Map::new();
                
                if let Some(v) = first_name {
                    contact.insert("first_name".to_string(), json!(v));
                }
                if let Some(v) = last_name {
                    contact.insert("last_name".to_string(), json!(v));
                }
                if let Some(v) = organisation_name {
                    contact.insert("organisation_name".to_string(), json!(v));
                }
                if let Some(v) = email {
                    contact.insert("email".to_string(), json!(v));
                }
                if let Some(v) = phone_number {
                    contact.insert("phone_number".to_string(), json!(v));
                }
                
                let body = json!({ "contact": contact });
                let result = client.put(&format!("contacts/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("contacts/{}", id)).await?;
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
    fn contact_view_as_str_matches_api() {
        assert_eq!(ContactView::All.as_str(), "all");
        assert_eq!(ContactView::ActiveProjects.as_str(), "active_projects");
        assert_eq!(ContactView::OpenSuppliers.as_str(), "open_suppliers");
    }

    #[test]
    fn contact_sort_as_str_supports_descending() {
        assert_eq!(ContactSort::Name.as_str(false), "name");
        assert_eq!(ContactSort::UpdatedAt.as_str(true), "-updated_at");
    }
}
