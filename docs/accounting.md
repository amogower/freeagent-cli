# accounting

## Overview

```
Accounting reports

Usage: freeagent accounting [OPTIONS] <COMMAND>

Commands:
  balance-sheet         Get balance sheet
  profit-and-loss       Get profit and loss summary
  trial-balance         Get trial balance
  cashflow              Get cashflow report
  general-ledger        Get general ledger
  account-transactions  Get account transactions
  transactions          List all accounting transactions
  transaction           Get an accounting transaction by ID
  help                  Print this message or the help of the given subcommand(s)

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

### accounting balance-sheet

```
Get balance sheet

Usage: freeagent accounting balance-sheet [OPTIONS]

Options:
      --date <DATE>
          Date for the balance sheet (YYYY-MM-DD)

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
### accounting profit-and-loss

```
Get profit and loss summary

Usage: freeagent accounting profit-and-loss [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --from-date <FROM_DATE>
          From date (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --to-date <TO_DATE>
          To date (YYYY-MM-DD)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### accounting trial-balance

```
Get trial balance

Usage: freeagent accounting trial-balance [OPTIONS]

Options:
      --date <DATE>
          Date for the trial balance (YYYY-MM-DD)

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
### accounting cashflow

```
Get cashflow report

Usage: freeagent accounting cashflow [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --from-date <FROM_DATE>
          From date (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --to-date <TO_DATE>
          To date (YYYY-MM-DD)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### accounting general-ledger

```
Get general ledger

Usage: freeagent accounting general-ledger [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --from-date <FROM_DATE>
          From date (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --to-date <TO_DATE>
          To date (YYYY-MM-DD)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### accounting account-transactions

```
Get account transactions

Usage: freeagent accounting account-transactions [OPTIONS] --category <CATEGORY>

Options:
      --category <CATEGORY>
          Category nominal code

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --from-date <FROM_DATE>
          From date (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --to-date <TO_DATE>
          To date (YYYY-MM-DD)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### accounting transactions

```
List all accounting transactions

Usage: freeagent accounting transactions [OPTIONS]

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
### accounting transaction

```
Get an accounting transaction by ID

Usage: freeagent accounting transaction [OPTIONS] <ID>

Arguments:
  <ID>
          Transaction ID

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
