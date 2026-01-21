# bank-accounts

## Overview

```
Bank account management

Usage: freeagent bank-accounts [OPTIONS] <COMMAND>

Commands:
  list    List all bank accounts
  get     Get a bank account by ID
  create  Create a new bank account
  update  Update a bank account
  delete  Delete a bank account
  help    Print this message or the help of the given subcommand(s)

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Subcommands

### bank-accounts list

```
List all bank accounts

Usage: freeagent bank-accounts list [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --view <VIEW>
          Filter by view
          
          [possible values: standard, credit-card, paypal-account, all]

      --page <PAGE>
          Page number

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --per-page <PER_PAGE>
          Items per page

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### bank-accounts get

```
Get a bank account by ID

Usage: freeagent bank-accounts get [OPTIONS] <ID>

Arguments:
  <ID>
          Bank account ID

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### bank-accounts create

```
Create a new bank account

Usage: freeagent bank-accounts create [OPTIONS] --account-type <ACCOUNT_TYPE> --name <NAME>

Options:
      --account-type <ACCOUNT_TYPE>
          Account type (required)
          
          [possible values: standard-bank-account, credit-card-account, paypal-account, loan-account, merchant-account]

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --name <NAME>
          Account name (required)

      --sandbox
          Use sandbox API instead of production

      --bank-name <BANK_NAME>
          Bank name

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --currency <CURRENCY>
          Currency code

      --opening-balance <OPENING_BALANCE>
          Opening balance

      --is-primary <IS_PRIMARY>
          Is primary account
          
          [possible values: true, false]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### bank-accounts update

```
Update a bank account

Usage: freeagent bank-accounts update [OPTIONS] <ID>

Arguments:
  <ID>
          Bank account ID

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --name <NAME>
          Account name

      --bank-name <BANK_NAME>
          Bank name

      --sandbox
          Use sandbox API instead of production

      --is-primary <IS_PRIMARY>
          Is primary account
          
          [possible values: true, false]

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### bank-accounts delete

```
Delete a bank account

Usage: freeagent bank-accounts delete [OPTIONS] <ID>

Arguments:
  <ID>
          Bank account ID

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

  -y, --yes
          Skip confirmation

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
