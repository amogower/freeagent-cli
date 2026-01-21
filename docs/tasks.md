# tasks

## Overview

```
Task management

Usage: freeagent tasks [OPTIONS] <COMMAND>

Commands:
  list    List all tasks
  get     Get a task by ID
  create  Create a new task
  update  Update a task
  delete  Delete a task
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

### tasks list

```
List all tasks

Usage: freeagent tasks list [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --project <PROJECT>
          Filter by project URL

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
### tasks get

```
Get a task by ID

Usage: freeagent tasks get [OPTIONS] <ID>

Arguments:
  <ID>
          Task ID

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
### tasks create

```
Create a new task

Usage: freeagent tasks create [OPTIONS] --project <PROJECT> --name <NAME>

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --project <PROJECT>
          Project URL (required)

      --name <NAME>
          Task name (required)

      --sandbox
          Use sandbox API instead of production

      --billable <BILLABLE>
          Billable (default: true)
          
          [possible values: true, false]

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --billing-rate <BILLING_RATE>
          Billing rate

      --billing-period <BILLING_PERIOD>
          Billing period

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### tasks update

```
Update a task

Usage: freeagent tasks update [OPTIONS] <ID>

Arguments:
  <ID>
          Task ID

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --name <NAME>
          Task name

      --billable <BILLABLE>
          Billable
          
          [possible values: true, false]

      --sandbox
          Use sandbox API instead of production

      --billing-rate <BILLING_RATE>
          Billing rate

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### tasks delete

```
Delete a task

Usage: freeagent tasks delete [OPTIONS] <ID>

Arguments:
  <ID>
          Task ID

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
