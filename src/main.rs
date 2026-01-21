//! FreeAgent CLI - A comprehensive command-line interface for the FreeAgent API.
//!
//! This CLI provides access to all FreeAgent API endpoints including:
//! - Invoices, Bills, Expenses, Credit Notes
//! - Contacts, Projects, Tasks, Timeslips
//! - Bank Accounts, Bank Transactions
//! - Accounting Reports (Balance Sheet, P&L, Trial Balance)
//! - VAT Returns
//! - And much more...
//!
//! # Authentication
//!
//! The CLI uses OAuth2 for authentication. Run `freeagent login` to authenticate
//! with your FreeAgent account. Tokens are stored securely and refreshed automatically.
//!
//! # Usage
//!
//! ```bash
//! freeagent login
//! freeagent company get
//! freeagent invoices list
//! freeagent contacts create --first-name "John" --last-name "Doe"
//! ```

mod api;
mod auth;
mod commands;
mod output;
mod update;

use anyhow::Result;
use clap::{Parser, Subcommand};

use api::FreeAgentClient;
use commands::*;
use output::OutputFormat;

/// FreeAgent CLI - Interact with the FreeAgent accounting API
#[derive(Parser)]
#[command(name = "freeagent")]
#[command(author = "FreeAgent CLI")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "A comprehensive command-line interface for the FreeAgent API", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Output format
    #[arg(short, long, value_enum, default_value = "json", global = true)]
    format: OutputFormat,

    /// Use sandbox API instead of production
    #[arg(long, global = true)]
    sandbox: bool,

    /// Disable automatic update checks
    #[arg(long, global = true, env = "FREEAGENT_NO_UPDATE")]
    no_update: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Login to FreeAgent using OAuth2
    Login,
    
    /// Logout and delete stored tokens
    Logout,
    
    /// Show current authentication status
    Status,

    /// Update the CLI to the latest release
    Update {
        /// Skip confirmation prompt
        #[arg(long)]
        yes: bool,
    },
    
    /// Company management
    #[command(subcommand)]
    Company(company::CompanyCommands),
    
    /// User management
    #[command(subcommand)]
    Users(users::UserCommands),
    
    /// Contact management
    #[command(subcommand)]
    Contacts(contacts::ContactCommands),
    
    /// Project management
    #[command(subcommand)]
    Projects(projects::ProjectCommands),
    
    /// Task management
    #[command(subcommand)]
    Tasks(tasks::TaskCommands),
    
    /// Invoice management
    #[command(subcommand)]
    Invoices(invoices::InvoiceCommands),
    
    /// Bill management
    #[command(subcommand)]
    Bills(bills::BillCommands),
    
    /// Expense management
    #[command(subcommand)]
    Expenses(expenses::ExpenseCommands),
    
    /// Credit note management
    #[command(subcommand)]
    CreditNotes(credit_notes::CreditNoteCommands),
    
    /// Estimate management
    #[command(subcommand)]
    Estimates(estimates::EstimateCommands),
    
    /// Recurring invoice management
    #[command(subcommand)]
    RecurringInvoices(recurring::RecurringInvoiceCommands),
    
    /// Bank account management
    #[command(subcommand)]
    BankAccounts(bank_accounts::BankAccountCommands),
    
    /// Bank transaction management
    #[command(subcommand)]
    BankTransactions(bank_transactions::BankTransactionCommands),
    
    /// Timeslip management
    #[command(subcommand)]
    Timeslips(timeslips::TimeslipCommands),
    
    /// Category management
    #[command(subcommand)]
    Categories(categories::CategoryCommands),
    
    /// Accounting reports
    #[command(subcommand)]
    Accounting(accounting::AccountingCommands),
    
    /// VAT return management
    #[command(subcommand)]
    Vat(vat::VatCommands),
    
    /// Attachment management
    #[command(subcommand)]
    Attachments(attachments::AttachmentCommands),
    
    /// Note management
    #[command(subcommand)]
    Notes(notes::NoteCommands),
    
    /// Capital asset management
    #[command(subcommand)]
    CapitalAssets(capital_assets::CapitalAssetCommands),
    
    /// Stock item management
    #[command(subcommand)]
    StockItems(stock_items::StockItemCommands),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if update::maybe_auto_update(cli.no_update).await? {
        return Ok(());
    }

    match cli.command {
        // Auth commands don't need a client
        Commands::Login => {
            let oauth = auth::OAuthManager::new(cli.sandbox)?;
            oauth.login().await?;
        }
        Commands::Logout => {
            let oauth = auth::OAuthManager::new(cli.sandbox)?;
            oauth.logout()?;
        }
        Commands::Status => {
            commands::auth::AuthCommands::Status.execute(cli.sandbox).await?;
        }
        Commands::Update { yes } => {
            update::run_update(yes).await?;
        }
        
        // All other commands need an authenticated client
        cmd => {
            let client = FreeAgentClient::new(cli.sandbox).await?;
            
            match cmd {
                Commands::Company(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Users(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Contacts(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Projects(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Tasks(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Invoices(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Bills(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Expenses(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::CreditNotes(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Estimates(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::RecurringInvoices(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::BankAccounts(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::BankTransactions(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Timeslips(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Categories(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Accounting(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Vat(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Attachments(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Notes(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::CapitalAssets(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::StockItems(cmd) => cmd.execute(&client, cli.format).await?,
                // Already handled above
                Commands::Login | Commands::Logout | Commands::Status | Commands::Update { .. } => {
                    unreachable!()
                }
            }
        }
    }

    Ok(())
}
