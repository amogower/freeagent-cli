# bills

## Overview

```
Bill management

Usage: freeagent bills [OPTIONS] <COMMAND>

Commands:
  list    List all bills
  get     Get a bill by ID
  create  Create a new bill
  update  Update a bill
  delete  Delete a bill
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

### bills list

```
List all bills

Usage: freeagent bills list [OPTIONS]

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
          
          [possible values: recent-open-or-overdue, open-or-overdue, open, overdue, draft, all]

      --contact <CONTACT>
          Filter by contact URL

      --sandbox
          Use sandbox API instead of production

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
### bills get

```
Get a bill by ID

Usage: freeagent bills get [OPTIONS] <ID>

Arguments:
  <ID>
          Bill ID

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
### bills create

```
Create a new bill

Usage: freeagent bills create [OPTIONS] --contact <CONTACT>

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

      --dated-on <DATED_ON>
          Bill date (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --due-on <DUE_ON>
          Due date (YYYY-MM-DD)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --reference <REFERENCE>
          Reference

      --currency <CURRENCY>
          Currency code

      --total-value <TOTAL_VALUE>
          Total value

      --category <CATEGORY>
          Category URL

      --comments <COMMENTS>
          Comments

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### bills update

```
Update a bill

Usage: freeagent bills update [OPTIONS] <ID>

Arguments:
  <ID>
          Bill ID

Options:
      --dated-on <DATED_ON>
          Bill date (YYYY-MM-DD)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --due-on <DUE_ON>
          Due date (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --reference <REFERENCE>
          Reference

      --comments <COMMENTS>
          Comments

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### bills delete

```
Delete a bill

Usage: freeagent bills delete [OPTIONS] <ID>

Arguments:
  <ID>
          Bill ID

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
