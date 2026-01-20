# FreeAgent CLI (Rust)

A comprehensive command-line interface for the FreeAgent accounting API, written in Rust. This CLI provides complete access to all FreeAgent API endpoints with built-in OAuth2 authentication and automatic token refresh.

## Features

- **Complete API Coverage**: All FreeAgent API endpoints including invoices, bills, expenses, contacts, projects, bank accounts, accounting reports, VAT returns, and more
- **OAuth2 Authentication**: Full OAuth2 flow with PKCE for enhanced security
- **Automatic Token Refresh**: Tokens are automatically refreshed when expired
- **Secure Token Storage**: Tokens stored in platform-specific config directory
- **Embedded Credentials**: OAuth client ID/secret can be embedded at compile time
- **Multiple Output Formats**: JSON (default), table, or compact JSON
- **Sandbox Support**: Test against the FreeAgent sandbox environment
- **Single Binary**: No runtime dependencies, easy to distribute

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd freeagent-cli-rust

# Build with your OAuth credentials
FREEAGENT_CLIENT_ID="your_client_id" \
FREEAGENT_CLIENT_SECRET="your_client_secret" \
cargo build --release

# The binary is at target/release/freeagent
```

### Pre-built Binary

Download the pre-built binary for your platform from the releases page.

## Configuration

### OAuth Credentials

OAuth credentials can be provided in two ways:

1. **Compile-time embedding** (recommended for distribution):
   ```bash
   FREEAGENT_CLIENT_ID="your_id" FREEAGENT_CLIENT_SECRET="your_secret" cargo build --release
   ```

2. **Edit the source** before building:
   Modify `src/auth/config.rs` to include your credentials.

### Getting OAuth Credentials

1. Log in to your FreeAgent account
2. Go to Settings → Developer Dashboard
3. Create a new OAuth application
4. Set the redirect URI to `http://localhost:8484/callback`
5. Copy the Client ID and Client Secret

## Usage

### Authentication

```bash
# Login to FreeAgent (opens browser for OAuth)
freeagent login

# Login to sandbox environment
freeagent login --sandbox

# Check authentication status
freeagent status

# Logout
freeagent logout
```

### Company

```bash
# Get company details
freeagent company get

# Get tax timeline
freeagent company tax-timeline
```

### Contacts

```bash
# List all contacts
freeagent contacts list

# List with filters
freeagent contacts list --view clients --sort name

# Get a specific contact
freeagent contacts get <contact_id>

# Create a contact
freeagent contacts create \
  --first-name "John" \
  --last-name "Doe" \
  --email "john@example.com" \
  --organisation-name "Acme Corp"

# Update a contact
freeagent contacts update <contact_id> --phone-number "+1234567890"

# Delete a contact
freeagent contacts delete <contact_id> --yes
```

### Invoices

```bash
# List all invoices
freeagent invoices list

# List with filters
freeagent invoices list --view open --from-date 2024-01-01

# Get an invoice
freeagent invoices get <invoice_id>

# Create an invoice
freeagent invoices create \
  --contact "https://api.freeagent.com/v2/contacts/123" \
  --dated-on "2024-01-15" \
  --payment-terms-in-days 30

# Get invoice PDF URL
freeagent invoices pdf <invoice_id>

# Send invoice by email
freeagent invoices send-email <invoice_id> --email-to "client@example.com"

# Mark as sent
freeagent invoices mark-as-sent <invoice_id>
```

### Bills

```bash
# List bills
freeagent bills list --view open

# Create a bill
freeagent bills create \
  --contact "https://api.freeagent.com/v2/contacts/123" \
  --dated-on "2024-01-15" \
  --total-value "500.00"
```

### Expenses

```bash
# List expenses
freeagent expenses list --from-date 2024-01-01

# Create an expense
freeagent expenses create \
  --user "https://api.freeagent.com/v2/users/1" \
  --category "https://api.freeagent.com/v2/categories/285" \
  --dated-on "2024-01-15" \
  --gross-value "50.00" \
  --description "Office supplies"
```

### Projects

```bash
# List projects
freeagent projects list --view active

# Create a project
freeagent projects create \
  --contact "https://api.freeagent.com/v2/contacts/123" \
  --name "Website Redesign" \
  --budget "5000" \
  --budget-units hours
```

### Timeslips

```bash
# List timeslips
freeagent timeslips list --from-date 2024-01-01

# Create a timeslip
freeagent timeslips create \
  --user "https://api.freeagent.com/v2/users/1" \
  --project "https://api.freeagent.com/v2/projects/123" \
  --task "https://api.freeagent.com/v2/tasks/456" \
  --dated-on "2024-01-15" \
  --hours "2.5"

# Start timer
freeagent timeslips start-timer <timeslip_id>

# Stop timer
freeagent timeslips stop-timer <timeslip_id>
```

### Bank Accounts & Transactions

```bash
# List bank accounts
freeagent bank-accounts list

# List transactions
freeagent bank-transactions list \
  --bank-account "https://api.freeagent.com/v2/bank_accounts/123" \
  --view unexplained

# Create a transaction
freeagent bank-transactions create \
  --bank-account "https://api.freeagent.com/v2/bank_accounts/123" \
  --dated-on "2024-01-15" \
  --amount "100.00" \
  --description "Client payment"
```

### Accounting Reports

```bash
# Balance sheet
freeagent accounting balance-sheet --date 2024-12-31

# Profit and loss
freeagent accounting profit-and-loss \
  --from-date 2024-01-01 \
  --to-date 2024-12-31

# Trial balance
freeagent accounting trial-balance --date 2024-12-31

# Cashflow
freeagent accounting cashflow \
  --from-date 2024-01-01 \
  --to-date 2024-12-31
```

### VAT Returns

```bash
# List VAT returns
freeagent vat list

# Get a VAT return
freeagent vat get <vat_return_id>

# Mark as filed
freeagent vat mark-as-filed <vat_return_id>
```

## Output Formats

```bash
# JSON (default)
freeagent contacts list

# Table format
freeagent contacts list --format table

# Compact JSON (single line)
freeagent contacts list --format compact
```

## Sandbox Mode

Use the `--sandbox` flag to test against the FreeAgent sandbox environment:

```bash
freeagent --sandbox login
freeagent --sandbox invoices list
```

## Command Groups

| Group | Description |
|-------|-------------|
| `company` | Company details and tax timeline |
| `users` | User management |
| `contacts` | Contact management |
| `projects` | Project management |
| `tasks` | Task management |
| `invoices` | Invoice management |
| `bills` | Bill management |
| `expenses` | Expense management |
| `credit-notes` | Credit note management |
| `estimates` | Estimate management |
| `recurring-invoices` | Recurring invoice management |
| `bank-accounts` | Bank account management |
| `bank-transactions` | Bank transaction management |
| `timeslips` | Time tracking |
| `categories` | Category listing |
| `accounting` | Accounting reports |
| `vat` | VAT return management |
| `attachments` | Attachment management |
| `notes` | Note management |
| `capital-assets` | Capital asset management |
| `stock-items` | Stock item management |

## Token Storage

Tokens are stored in the platform-specific config directory:
- **Linux**: `~/.config/freeagent-cli/tokens.json`
- **macOS**: `~/Library/Application Support/freeagent-cli/tokens.json`
- **Windows**: `C:\Users\<User>\AppData\Roaming\freeagent-cli\tokens.json`

## Security

- OAuth2 with PKCE for enhanced security
- Client secret can be embedded at compile time (standard practice for native apps)
- Tokens are stored locally and never transmitted except to FreeAgent
- Automatic token refresh prevents credential exposure

## Building for Different Platforms

```bash
# Linux (current platform)
cargo build --release

# Cross-compile for macOS (requires cross-compilation setup)
cargo build --release --target x86_64-apple-darwin

# Cross-compile for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## License

MIT License
