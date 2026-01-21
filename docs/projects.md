# projects

## Overview

```
Project management

Usage: freeagent projects [OPTIONS] <COMMAND>

Commands:
  list      List all projects
  get       Get a project by ID
  create    Create a new project
  update    Update a project
  delete    Delete a project
  invoices  List project invoices
  timeline  Get project timeline
  help      Print this message or the help of the given subcommand(s)

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

### projects list

```
List all projects

Usage: freeagent projects list [OPTIONS]

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
          
          [possible values: active, completed, cancelled, hidden, all]

      --contact <CONTACT>
          Filter by contact URL

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --updated-since <UPDATED_SINCE>
          Filter by update date (ISO 8601)

      --page <PAGE>
          Page number

      --per-page <PER_PAGE>
          Items per page

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### projects get

```
Get a project by ID

Usage: freeagent projects get [OPTIONS] <ID>

Arguments:
  <ID>
          Project ID

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
### projects create

```
Create a new project

Usage: freeagent projects create [OPTIONS] --contact <CONTACT> --name <NAME>

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

      --name <NAME>
          Project name (required)

      --sandbox
          Use sandbox API instead of production

      --budget <BUDGET>
          Budget amount

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --budget-units <BUDGET_UNITS>
          Budget units
          
          [possible values: hours, days, monetary]

      --currency <CURRENCY>
          Currency code

      --billing-period <BILLING_PERIOD>
          Billing period (hour, day, week, month, year)

      --hours-per-day <HOURS_PER_DAY>
          Hours per day

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### projects update

```
Update a project

Usage: freeagent projects update [OPTIONS] <ID>

Arguments:
  <ID>
          Project ID

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --name <NAME>
          Project name

      --budget <BUDGET>
          Budget amount

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --status <STATUS>
          Status

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### projects delete

```
Delete a project

Usage: freeagent projects delete [OPTIONS] <ID>

Arguments:
  <ID>
          Project ID

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
### projects invoices

```
List project invoices

Usage: freeagent projects invoices [OPTIONS] <ID>

Arguments:
  <ID>
          Project ID

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
### projects timeline

```
Get project timeline

Usage: freeagent projects timeline [OPTIONS] <ID>

Arguments:
  <ID>
          Project ID

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
