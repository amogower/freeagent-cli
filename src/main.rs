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

    /// Account manager management
    #[command(subcommand)]
    AccountManagers(account_managers::AccountManagerCommands),

    /// Practice management
    #[command(subcommand)]
    Practice(practice::PracticeCommands),

    /// Practice client management
    #[command(subcommand)]
    Clients(clients::ClientCommands),

    /// Bank feed management
    #[command(subcommand)]
    BankFeeds(bank_feeds::BankFeedCommands),

    /// Bank transaction explanation management
    #[command(subcommand)]
    BankTransactionExplanations(bank_transaction_explanations::BankTransactionExplanationCommands),

    /// Corporation tax return management
    #[command(subcommand)]
    CorporationTaxReturns(corporation_tax_returns::CorporationTaxReturnCommands),

    /// Credit note reconciliation management
    #[command(subcommand)]
    CreditNoteReconciliations(credit_note_reconciliations::CreditNoteReconciliationCommands),

    /// Currency management
    #[command(subcommand)]
    Currencies(currencies::CurrencyCommands),

    /// Depreciation profile management
    #[command(subcommand)]
    DepreciationProfiles(depreciation_profiles::DepreciationProfileCommands),

    /// EC MOSS management
    #[command(subcommand)]
    EcMoss(ec_moss::EcMossCommands),

    /// Email address management
    #[command(subcommand)]
    EmailAddresses(email_addresses::EmailAddressCommands),

    /// Estimate item management
    #[command(subcommand)]
    EstimateItems(estimate_items::EstimateItemCommands),

    /// Final accounts report management
    #[command(subcommand)]
    FinalAccountsReports(final_accounts_reports::FinalAccountsReportCommands),

    /// Hire purchase management
    #[command(subcommand)]
    HirePurchases(hire_purchases::HirePurchaseCommands),

    /// Income tax return management
    #[command(subcommand)]
    IncomeTaxReturns(income_tax_returns::IncomeTaxReturnCommands),

    /// Journal set management
    #[command(subcommand)]
    JournalSets(journal_sets::JournalSetCommands),

    /// Payroll management
    #[command(subcommand)]
    Payroll(payroll::PayrollCommands),

    /// Payroll profile management
    #[command(subcommand)]
    PayrollProfiles(payroll_profiles::PayrollProfileCommands),

    /// Price list item management
    #[command(subcommand)]
    PriceListItems(price_list_items::PriceListItemCommands),

    /// Property management
    #[command(subcommand)]
    Properties(properties::PropertyCommands),

    /// Sales tax period management
    #[command(subcommand)]
    SalesTaxPeriods(sales_tax_periods::SalesTaxPeriodCommands),

    /// Capital asset type management
    #[command(subcommand)]
    CapitalAssetTypes(capital_asset_types::CapitalAssetTypeCommands),

    /// CIS band management
    #[command(subcommand)]
    CisBands(cis_bands::CisBandCommands),
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
                Commands::AccountManagers(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Practice(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Clients(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::BankFeeds(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::BankTransactionExplanations(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::CorporationTaxReturns(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::CreditNoteReconciliations(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Currencies(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::DepreciationProfiles(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::EcMoss(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::EmailAddresses(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::EstimateItems(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::FinalAccountsReports(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::HirePurchases(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::IncomeTaxReturns(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::JournalSets(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Payroll(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::PayrollProfiles(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::PriceListItems(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::Properties(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::SalesTaxPeriods(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::CapitalAssetTypes(cmd) => cmd.execute(&client, cli.format).await?,
                Commands::CisBands(cmd) => cmd.execute(&client, cli.format).await?,
                // Already handled above
                Commands::Login | Commands::Logout | Commands::Status | Commands::Update { .. } => {
                    unreachable!()
                }
            }
        }
    }

    Ok(())
}
