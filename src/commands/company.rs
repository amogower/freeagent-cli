//! Company commands.

use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::api::FreeAgentClient;
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Subcommand)]
pub enum CompanyCommands {
    /// Get company details
    Get,
    
    /// Update company details
    Update {
        /// Company name
        #[arg(long)]
        name: Option<String>,
        
        /// Company type
        #[arg(long)]
        company_type: Option<String>,
        
        /// Currency code
        #[arg(long)]
        currency: Option<String>,
        
        /// Mileage units (miles or km)
        #[arg(long)]
        mileage_units: Option<String>,
        
        /// Company registration number
        #[arg(long)]
        company_registration_number: Option<String>,
        
        /// Sales tax registration number
        #[arg(long)]
        sales_tax_registration_number: Option<String>,
    },
    
    /// Get company tax timeline
    TaxTimeline,
}

impl CompanyCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::Get => {
                let result = client.get("company", None).await?;
                print_output(&result, format);
            }
            Self::Update {
                name,
                company_type,
                currency,
                mileage_units,
                company_registration_number,
                sales_tax_registration_number,
            } => {
                let mut company = serde_json::Map::new();
                
                if let Some(v) = name {
                    company.insert("name".to_string(), json!(v));
                }
                if let Some(v) = company_type {
                    company.insert("type".to_string(), json!(v));
                }
                if let Some(v) = currency {
                    company.insert("currency".to_string(), json!(v));
                }
                if let Some(v) = mileage_units {
                    company.insert("mileage_units".to_string(), json!(v));
                }
                if let Some(v) = company_registration_number {
                    company.insert("company_registration_number".to_string(), json!(v));
                }
                if let Some(v) = sales_tax_registration_number {
                    company.insert("sales_tax_registration_number".to_string(), json!(v));
                }
                
                let body = json!({ "company": company });
                let result = client.put("company", Some(body)).await?;
                print_output(&result, format);
            }
            Self::TaxTimeline => {
                let result = client.get("company/tax_timeline", None).await?;
                print_output(&result, format);
            }
        }
        
        Ok(())
    }
}
