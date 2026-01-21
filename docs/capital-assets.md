# capital-assets

## Overview

```
Capital asset management

Usage: freeagent capital-assets [OPTIONS] <COMMAND>

Commands:
  list    List all capital assets
  get     Get a capital asset by ID
  create  Create a new capital asset
  update  Update a capital asset
  delete  Delete a capital asset
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

### capital-assets list

```
List all capital assets

Usage: freeagent capital-assets list [OPTIONS]

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
### capital-assets get

```
Get a capital asset by ID

Usage: freeagent capital-assets get [OPTIONS] <ID>

Arguments:
  <ID>
          Capital asset ID

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
### capital-assets create

```
Create a new capital asset

Usage: freeagent capital-assets create [OPTIONS] --description <DESCRIPTION> --asset-life-years <ASSET_LIFE_YEARS>

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

      --asset-life-years <ASSET_LIFE_YEARS>
          Asset life in years (required)

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --purchased-on <PURCHASED_ON>
          Purchase date (YYYY-MM-DD)

      --purchase-price <PURCHASE_PRICE>
          Purchase price

      --category <CATEGORY>
          Category URL

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### capital-assets update

```
Update a capital asset

Usage: freeagent capital-assets update [OPTIONS] <ID>

Arguments:
  <ID>
          Capital asset ID

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

      --asset-life-years <ASSET_LIFE_YEARS>
          Asset life in years

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
### capital-assets delete

```
Delete a capital asset

Usage: freeagent capital-assets delete [OPTIONS] <ID>

Arguments:
  <ID>
          Capital asset ID

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
