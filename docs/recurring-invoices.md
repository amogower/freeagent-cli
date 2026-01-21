# recurring-invoices

## Overview

```
Recurring invoice management

Usage: freeagent recurring-invoices [OPTIONS] <COMMAND>

Commands:
  list    List all recurring invoices
  get     Get a recurring invoice by ID
  create  Create a new recurring invoice
  update  Update a recurring invoice
  delete  Delete a recurring invoice
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

### recurring-invoices list

```
List all recurring invoices

Usage: freeagent recurring-invoices list [OPTIONS]

Options:
      --contact <CONTACT>
          Filter by contact URL

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

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
### recurring-invoices get

```
Get a recurring invoice by ID

Usage: freeagent recurring-invoices get [OPTIONS] <ID>

Arguments:
  <ID>
          Recurring invoice ID

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
### recurring-invoices create

```
Create a new recurring invoice

Usage: freeagent recurring-invoices create [OPTIONS] --contact <CONTACT>

Options:
      --contact <CONTACT>
          Contact URL (required)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --recurring-profile-name <RECURRING_PROFILE_NAME>
          Recurring profile name

      --sandbox
          Use sandbox API instead of production

      --frequency-period <FREQUENCY_PERIOD>
          Frequency period (week, month, year)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --frequency <FREQUENCY>
          Frequency (e.g., 1 for monthly, 2 for bi-monthly)

      --start-date <START_DATE>
          Start date (YYYY-MM-DD)

      --currency <CURRENCY>
          Currency code

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### recurring-invoices update

```
Update a recurring invoice

Usage: freeagent recurring-invoices update [OPTIONS] <ID>

Arguments:
  <ID>
          Recurring invoice ID

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --recurring-profile-name <RECURRING_PROFILE_NAME>
          Recurring profile name

      --frequency-period <FREQUENCY_PERIOD>
          Frequency period (week, month, year)

      --sandbox
          Use sandbox API instead of production

      --frequency <FREQUENCY>
          Frequency

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### recurring-invoices delete

```
Delete a recurring invoice

Usage: freeagent recurring-invoices delete [OPTIONS] <ID>

Arguments:
  <ID>
          Recurring invoice ID

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
