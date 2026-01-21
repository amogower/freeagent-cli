# stock-items

## Overview

```
Stock item management

Usage: freeagent stock-items [OPTIONS] <COMMAND>

Commands:
  list    List all stock items
  get     Get a stock item by ID
  create  Create a new stock item
  update  Update a stock item
  delete  Delete a stock item
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

### stock-items list

```
List all stock items

Usage: freeagent stock-items list [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --page <PAGE>
          Page number

      --per-page <PER_PAGE>
          Items per page

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
### stock-items get

```
Get a stock item by ID

Usage: freeagent stock-items get [OPTIONS] <ID>

Arguments:
  <ID>
          Stock item ID

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
### stock-items create

```
Create a new stock item

Usage: freeagent stock-items create [OPTIONS] --description <DESCRIPTION>

Options:
      --description <DESCRIPTION>
          Description (required)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --opening-quantity <OPENING_QUANTITY>
          Opening quantity

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --opening-balance <OPENING_BALANCE>
          Opening balance

      --code <CODE>
          Stock item code

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### stock-items update

```
Update a stock item

Usage: freeagent stock-items update [OPTIONS] <ID>

Arguments:
  <ID>
          Stock item ID

Options:
      --description <DESCRIPTION>
          Description

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --code <CODE>
          Stock item code

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
### stock-items delete

```
Delete a stock item

Usage: freeagent stock-items delete [OPTIONS] <ID>

Arguments:
  <ID>
          Stock item ID

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
