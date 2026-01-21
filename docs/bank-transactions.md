# bank-transactions

## Overview

```
Bank transaction management

Usage: freeagent bank-transactions [OPTIONS] <COMMAND>

Commands:
  list    List bank transactions
  get     Get a bank transaction by ID
  create  Create a new bank transaction
  update  Update a bank transaction
  delete  Delete a bank transaction
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

### bank-transactions list

```
List bank transactions

Usage: freeagent bank-transactions list [OPTIONS] --bank-account <BANK_ACCOUNT>

Options:
      --bank-account <BANK_ACCOUNT>
          Bank account URL (required)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --sandbox
          Use sandbox API instead of production

      --view <VIEW>
          Filter by view
          
          [possible values: all, unexplained, explained, manually-added, imported]

      --from-date <FROM_DATE>
          Filter from date (YYYY-MM-DD)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --to-date <TO_DATE>
          Filter to date (YYYY-MM-DD)

      --page <PAGE>
          Page number

      --per-page <PER_PAGE>
          Items per page

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### bank-transactions get

```
Get a bank transaction by ID

Usage: freeagent bank-transactions get [OPTIONS] <ID>

Arguments:
  <ID>
          Bank transaction ID

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
### bank-transactions create

```
Create a new bank transaction

Usage: freeagent bank-transactions create [OPTIONS] --bank-account <BANK_ACCOUNT> --dated-on <DATED_ON> --amount <AMOUNT>

Options:
      --bank-account <BANK_ACCOUNT>
          Bank account URL (required)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --dated-on <DATED_ON>
          Transaction date (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --amount <AMOUNT>
          Amount (positive for credit, negative for debit)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --description <DESCRIPTION>
          Description

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### bank-transactions update

```
Update a bank transaction

Usage: freeagent bank-transactions update [OPTIONS] <ID>

Arguments:
  <ID>
          Bank transaction ID

Options:
      --dated-on <DATED_ON>
          Transaction date (YYYY-MM-DD)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --amount <AMOUNT>
          Amount

      --sandbox
          Use sandbox API instead of production

      --description <DESCRIPTION>
          Description

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### bank-transactions delete

```
Delete a bank transaction

Usage: freeagent bank-transactions delete [OPTIONS] <ID>

Arguments:
  <ID>
          Bank transaction ID

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
