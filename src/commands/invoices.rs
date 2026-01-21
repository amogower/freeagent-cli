//! Invoice commands.

use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use serde_json::json;

use crate::api::{FreeAgentClient, QueryBuilder};
use crate::output::{print_output, OutputFormat};

#[derive(Debug, Clone, ValueEnum)]
pub enum InvoiceView {
    RecentOpenOrOverdue,
    OpenOrOverdue,
    Open,
    Overdue,
    Draft,
    ScheduledToEmail,
    ThankYou,
    Reminded,
    All,
}

impl InvoiceView {
    fn as_str(&self) -> &'static str {
        match self {
            Self::RecentOpenOrOverdue => "recent_open_or_overdue",
            Self::OpenOrOverdue => "open_or_overdue",
            Self::Open => "open",
            Self::Overdue => "overdue",
            Self::Draft => "draft",
            Self::ScheduledToEmail => "scheduled_to_email",
            Self::ThankYou => "thank_you",
            Self::Reminded => "reminded",
            Self::All => "all",
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum EcStatus {
    Uk,
    EcGoods,
    EcServices,
    EcMoss,
    NonEc,
}

impl EcStatus {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Uk => "UK",
            Self::EcGoods => "EC Goods",
            Self::EcServices => "EC Services",
            Self::EcMoss => "EC Moss",
            Self::NonEc => "Non-EC",
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum InvoiceCommands {
    /// List all invoices
    List {
        /// Filter by view
        #[arg(long)]
        view: Option<InvoiceView>,
        
        /// Filter by contact URL
        #[arg(long)]
        contact: Option<String>,
        
        /// Filter by project URL
        #[arg(long)]
        project: Option<String>,
        
        /// Filter from date (YYYY-MM-DD)
        #[arg(long)]
        from_date: Option<String>,
        
        /// Filter to date (YYYY-MM-DD)
        #[arg(long)]
        to_date: Option<String>,
        
        /// Filter by update date (ISO 8601)
        #[arg(long)]
        updated_since: Option<String>,
        
        /// Include nested invoice items
        #[arg(long)]
        nested_invoice_items: bool,
        
        /// Page number
        #[arg(long)]
        page: Option<i32>,
        
        /// Items per page
        #[arg(long)]
        per_page: Option<i32>,
    },
    
    /// Get an invoice by ID
    Get {
        /// Invoice ID
        id: String,
        
        /// Include nested invoice items
        #[arg(long)]
        nested_invoice_items: bool,
    },
    
    /// Create a new invoice
    Create {
        /// Contact URL (required)
        #[arg(long)]
        contact: String,
        
        /// Project URL
        #[arg(long)]
        project: Option<String>,
        
        /// Invoice date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,
        
        /// Due date (YYYY-MM-DD)
        #[arg(long)]
        due_on: Option<String>,
        
        /// Invoice reference
        #[arg(long)]
        reference: Option<String>,
        
        /// Currency code
        #[arg(long)]
        currency: Option<String>,
        
        /// Payment terms in days
        #[arg(long)]
        payment_terms_in_days: Option<i32>,
        
        /// EC status
        #[arg(long)]
        ec_status: Option<EcStatus>,
        
        /// Comments
        #[arg(long)]
        comments: Option<String>,
        
        /// Invoice items as JSON array
        #[arg(long)]
        items_json: Option<String>,
    },
    
    /// Update an invoice
    Update {
        /// Invoice ID
        id: String,
        
        /// Invoice date (YYYY-MM-DD)
        #[arg(long)]
        dated_on: Option<String>,
        
        /// Due date (YYYY-MM-DD)
        #[arg(long)]
        due_on: Option<String>,
        
        /// Invoice reference
        #[arg(long)]
        reference: Option<String>,
        
        /// Comments
        #[arg(long)]
        comments: Option<String>,
    },
    
    /// Delete an invoice
    Delete {
        /// Invoice ID
        id: String,
        
        /// Skip confirmation
        #[arg(long, short)]
        yes: bool,
    },
    
    /// Get invoice PDF URL
    Pdf {
        /// Invoice ID
        id: String,
    },
    
    /// Send invoice by email
    SendEmail {
        /// Invoice ID
        id: String,
        
        /// Email recipient
        #[arg(long)]
        email_to: String,
        
        /// Email subject
        #[arg(long)]
        subject: Option<String>,
        
        /// Email body
        #[arg(long)]
        body: Option<String>,
    },
    
    /// Mark invoice as sent
    MarkAsSent {
        /// Invoice ID
        id: String,
    },
    
    /// Mark invoice as draft
    MarkAsDraft {
        /// Invoice ID
        id: String,
    },
    
    /// Mark invoice as cancelled
    MarkAsCancelled {
        /// Invoice ID
        id: String,
    },
    
    /// Mark invoice as scheduled
    MarkAsScheduled {
        /// Invoice ID
        id: String,
    },
}

impl InvoiceCommands {
    pub async fn execute(&self, client: &FreeAgentClient, format: OutputFormat) -> Result<()> {
        match self {
            Self::List {
                view,
                contact,
                project,
                from_date,
                to_date,
                updated_since,
                nested_invoice_items,
                page,
                per_page,
            } => {
                let params = QueryBuilder::new()
                    .add("view", view.as_ref().map(|v| v.as_str()))
                    .add("contact", contact.clone())
                    .add("project", project.clone())
                    .add("from_date", from_date.clone())
                    .add("to_date", to_date.clone())
                    .add("updated_since", updated_since.clone())
                    .add_bool("nested_invoice_items", if *nested_invoice_items { Some(true) } else { None })
                    .add_i32("page", *page)
                    .add_i32("per_page", *per_page)
                    .build();
                let result = client.get("invoices", params).await?;
                print_output(&result, format);
            }
            Self::Get { id, nested_invoice_items } => {
                let params = QueryBuilder::new()
                    .add_bool("nested_invoice_items", if *nested_invoice_items { Some(true) } else { None })
                    .build();
                let result = client.get(&format!("invoices/{}", id), params).await?;
                print_output(&result, format);
            }
            Self::Create {
                contact,
                project,
                dated_on,
                due_on,
                reference,
                currency,
                payment_terms_in_days,
                ec_status,
                comments,
                items_json,
            } => {
                let mut invoice = serde_json::Map::new();
                invoice.insert("contact".to_string(), json!(contact));
                
                if let Some(v) = project {
                    invoice.insert("project".to_string(), json!(v));
                }
                if let Some(v) = dated_on {
                    invoice.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = due_on {
                    invoice.insert("due_on".to_string(), json!(v));
                }
                if let Some(v) = reference {
                    invoice.insert("reference".to_string(), json!(v));
                }
                if let Some(v) = currency {
                    invoice.insert("currency".to_string(), json!(v));
                }
                if let Some(v) = payment_terms_in_days {
                    invoice.insert("payment_terms_in_days".to_string(), json!(v));
                }
                if let Some(v) = ec_status {
                    invoice.insert("ec_status".to_string(), json!(v.as_str()));
                }
                if let Some(v) = comments {
                    invoice.insert("comments".to_string(), json!(v));
                }
                if let Some(v) = items_json {
                    let items: serde_json::Value = serde_json::from_str(v)?;
                    invoice.insert("invoice_items".to_string(), items);
                }
                
                let body = json!({ "invoice": invoice });
                let result = client.post("invoices", Some(body)).await?;
                print_output(&result, format);
            }
            Self::Update {
                id,
                dated_on,
                due_on,
                reference,
                comments,
            } => {
                let mut invoice = serde_json::Map::new();
                
                if let Some(v) = dated_on {
                    invoice.insert("dated_on".to_string(), json!(v));
                }
                if let Some(v) = due_on {
                    invoice.insert("due_on".to_string(), json!(v));
                }
                if let Some(v) = reference {
                    invoice.insert("reference".to_string(), json!(v));
                }
                if let Some(v) = comments {
                    invoice.insert("comments".to_string(), json!(v));
                }
                
                let body = json!({ "invoice": invoice });
                let result = client.put(&format!("invoices/{}", id), Some(body)).await?;
                print_output(&result, format);
            }
            Self::Delete { id, yes } => {
                if !yes {
                    eprintln!("Use --yes to confirm deletion");
                    return Ok(());
                }
                let result = client.delete(&format!("invoices/{}", id)).await?;
                print_output(&result, format);
            }
            Self::Pdf { id } => {
                let result = client.get(&format!("invoices/{}/pdf", id), None).await?;
                print_output(&result, format);
            }
            Self::SendEmail { id, email_to, subject, body } => {
                let mut email = serde_json::Map::new();
                email.insert("email".to_string(), json!({
                    "to": email_to,
                    "subject": subject,
                    "body": body,
                }));
                
                let result = client.post(&format!("invoices/{}/send_email", id), Some(json!(email))).await?;
                print_output(&result, format);
            }
            Self::MarkAsSent { id } => {
                let result = client.put(&format!("invoices/{}/transitions/mark_as_sent", id), None::<()>).await?;
                print_output(&result, format);
            }
            Self::MarkAsDraft { id } => {
                let result = client.put(&format!("invoices/{}/transitions/mark_as_draft", id), None::<()>).await?;
                print_output(&result, format);
            }
            Self::MarkAsCancelled { id } => {
                let result = client.put(&format!("invoices/{}/transitions/mark_as_cancelled", id), None::<()>).await?;
                print_output(&result, format);
            }
            Self::MarkAsScheduled { id } => {
                let result = client.put(&format!("invoices/{}/transitions/mark_as_scheduled", id), None::<()>).await?;
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
    fn invoice_view_as_str_matches_api() {
        assert_eq!(InvoiceView::RecentOpenOrOverdue.as_str(), "recent_open_or_overdue");
        assert_eq!(InvoiceView::ScheduledToEmail.as_str(), "scheduled_to_email");
        assert_eq!(InvoiceView::All.as_str(), "all");
    }

    #[test]
    fn ec_status_as_str_matches_api() {
        assert_eq!(EcStatus::Uk.as_str(), "UK");
        assert_eq!(EcStatus::EcGoods.as_str(), "EC Goods");
        assert_eq!(EcStatus::NonEc.as_str(), "Non-EC");
    }
}
