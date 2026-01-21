# expenses

## Overview

```
Expense management

Usage: freeagent expenses [OPTIONS] <COMMAND>

Commands:
  list    List all expenses
  get     Get an expense by ID
  create  Create a new expense
  update  Update an expense
  delete  Delete an expense
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

### expenses list

```
List all expenses

Usage: freeagent expenses list [OPTIONS]

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
          
          [possible values: all, unbilled, billed]

      --sandbox
          Use sandbox API instead of production

      --user <USER>
          Filter by user URL

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --project <PROJECT>
          Filter by project URL

      --from-date <FROM_DATE>
          Filter from date (YYYY-MM-DD)

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
### expenses get

```
Get an expense by ID

Usage: freeagent expenses get [OPTIONS] <ID>

Arguments:
  <ID>
          Expense ID

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
### expenses create

```
Create a new expense

Usage: freeagent expenses create [OPTIONS] --user <USER> --category <CATEGORY> --dated-on <DATED_ON> --gross-value <GROSS_VALUE>

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --user <USER>
          User URL (required)

      --category <CATEGORY>
          Category URL (required)

      --sandbox
          Use sandbox API instead of production

      --dated-on <DATED_ON>
          Expense date (YYYY-MM-DD)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --gross-value <GROSS_VALUE>
          Gross value

      --description <DESCRIPTION>
          Description

      --project <PROJECT>
          Project URL

      --currency <CURRENCY>
          Currency code

      --rebill-to-project <REBILL_TO_PROJECT>
          Rebill to project
          
          [possible values: true, false]

      --manual-sales-tax-amount <MANUAL_SALES_TAX_AMOUNT>
          Manual sales tax amount

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### expenses update

```
Update an expense

Usage: freeagent expenses update [OPTIONS] <ID>

Arguments:
  <ID>
          Expense ID

Options:
      --dated-on <DATED_ON>
          Expense date (YYYY-MM-DD)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --gross-value <GROSS_VALUE>
          Gross value

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
### expenses delete

```
Delete an expense

Usage: freeagent expenses delete [OPTIONS] <ID>

Arguments:
  <ID>
          Expense ID

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
