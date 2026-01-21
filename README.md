# FreeAgent CLI

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

### Quick Install (macOS + Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/amogower/freeagent-cli/main/scripts/install.sh | bash
```

Install a specific version:

```bash
curl -fsSL https://raw.githubusercontent.com/amogower/freeagent-cli/main/scripts/install.sh | bash -s -- 0.1.0
```

Use a fork or custom install directory:

```bash
FREEAGENT_GITHUB_REPO="yourname/freeagent-cli" INSTALL_DIR="$HOME/.local/bin" \
  curl -fsSL https://raw.githubusercontent.com/amogower/freeagent-cli/main/scripts/install.sh | bash
```

### Homebrew (macOS + Linux)

This project includes a Homebrew formula template at `packaging/homebrew/freeagent.rb`.
To publish via Homebrew, create a tap repo and copy the formula there after each release.

### Linux Packages (.deb / .rpm)

Download the `.deb` or `.rpm` asset from the GitHub Releases page and install with:

```bash
sudo dpkg -i freeagent_0.1.0_amd64.deb
```

```bash
sudo rpm -i freeagent-0.1.0-1.x86_64.rpm
```

### macOS Installer (.pkg)

Download the `.pkg` asset from the GitHub Releases page and install with:

```bash
sudo installer -pkg freeagent-0.1.0-aarch64-apple-darwin.pkg -target /
```

### From Source

```bash
# Clone the repository
git clone https://github.com/amogower/freeagent-cli
cd freeagent-cli

# Build with your OAuth credentials
FREEAGENT_CLIENT_ID="your_client_id" \
FREEAGENT_CLIENT_SECRET="your_client_secret" \
cargo build --release

# The binary is at target/release/freeagent
```

### Cargo Install

```bash
cargo install --git https://github.com/amogower/freeagent-cli
```

### Release Assets

Each GitHub Release includes:
- Platform tarballs (`freeagent-<version>-<target>.tar.gz`)
- macOS `.pkg` installers
- Linux `.deb` and `.rpm` packages

## Release Process

Releases are automated via GitHub Actions. Use the `prepare-release` workflow to
bump the version, commit, and tag, then the tag pipeline publishes assets.

1. In GitHub Actions, run the `prepare-release` workflow and choose `major`,
   `minor`, or `patch`.
2. The workflow runs `scripts/release.js`, which updates `Cargo.toml` (and
   `Cargo.lock`), commits `chore(release): vX.Y.Z`, and pushes tag `vX.Y.Z`.
3. The `release` workflow triggers on the tag and uploads assets to the GitHub
   Release.

If you prefer manual tagging, ensure the tag matches the version in
`Cargo.toml` and push the tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The workflow builds all targets, packages assets, generates `SHA256SUMS`, and produces
`dist/freeagent.rb` for Homebrew (copy it into your tap repo).


## Documentation

Detailed command reference lives in `docs/` with one markdown file per top-level command.

- Run `freeagent <command> --help` for quick usage
- See `docs/<command>.md` for full subcommand and flag details

### Structured Flags vs --data

Most write operations accept structured flags with validation. Use `--data` only when you need to pass fields not exposed as flags.

## Configuration

This CLI is configured for OAuth out of the box. If you are building
from source and need to supply your own credentials, you can provide them
at build time or runtime:

```bash
# Build-time embedding (recommended for distribution)
FREEAGENT_CLIENT_ID="your_id" FREEAGENT_CLIENT_SECRET="your_secret" cargo build --release

# Runtime credentials (useful with prebuilt binaries)
FREEAGENT_CLIENT_ID="your_id" FREEAGENT_CLIENT_SECRET="your_secret" freeagent login
```

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

## Rate Limit Handling

The CLI automatically handles FreeAgent API rate limits:
- **Automatic Retry**: Requests that hit rate limits (429 status) are automatically retried
- **Exponential Backoff**: Uses exponential backoff strategy with configurable parameters
- **Retry-After Support**: Respects the `Retry-After` header from the API
- **Configurable**: Customize retry behavior via environment variables

### Rate Limit Configuration

Configure retry behavior using environment variables:

```bash
# Maximum number of retry attempts (default: 3)
export FREEAGENT_MAX_RETRIES=5

# Initial backoff duration in seconds (default: 1)
export FREEAGENT_INITIAL_BACKOFF_SECS=2

# Maximum backoff duration in seconds (default: 60)
export FREEAGENT_MAX_BACKOFF_SECS=120

# Use exponential backoff (default: true)
export FREEAGENT_EXPONENTIAL_BACKOFF=true
```

### FreeAgent API Rate Limits

The FreeAgent API enforces the following limits:
- **120 requests per minute** per user
- **3600 requests per hour** per user
- **15 token refreshes per minute** per user

When these limits are exceeded, the CLI will automatically wait and retry according to the configured strategy.

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
