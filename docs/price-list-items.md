# price-list-items

## Overview

```
Price list item management

Usage: freeagent price-list-items [OPTIONS] <COMMAND>

Commands:
  list    List price list items
  get     Get a price list item by ID
  create  Create a price list item (JSON payload)
  update  Update a price list item (JSON payload)
  delete  Delete a price list item
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

### price-list-items list

```
List price list items

Usage: freeagent price-list-items list [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --sort <SORT>
          Sort by (code, description, price, quantity, created_at, updated_at)

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
### price-list-items get

```
Get a price list item by ID

Usage: freeagent price-list-items get [OPTIONS] <ID>

Arguments:
  <ID>
          Price list item ID

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
### price-list-items create

```
Create a price list item (JSON payload)

Usage: freeagent price-list-items create [OPTIONS]

Options:
      --code <CODE>
          Code

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --item-type <ITEM_TYPE>
          Item type (service, product, expense, time)

      --sandbox
          Use sandbox API instead of production

      --description <DESCRIPTION>
          Description

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --price <PRICE>
          Price

      --quantity <QUANTITY>
          Quantity

      --vat-status <VAT_STATUS>
          VAT status (VAT, EXEMPT, OUT_OF_SCOPE)

      --sales-tax-rate <SALES_TAX_RATE>
          Sales tax rate

      --second-sales-tax-rate <SECOND_SALES_TAX_RATE>
          Second sales tax rate

      --category <CATEGORY>
          Category URL

      --stock-item <STOCK_ITEM>
          Stock item URL

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### price-list-items update

```
Update a price list item (JSON payload)

Usage: freeagent price-list-items update [OPTIONS] <ID>

Arguments:
  <ID>
          Price list item ID

Options:
      --code <CODE>
          Code

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --item-type <ITEM_TYPE>
          Item type (service, product, expense, time)

      --sandbox
          Use sandbox API instead of production

      --description <DESCRIPTION>
          Description

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --price <PRICE>
          Price

      --quantity <QUANTITY>
          Quantity

      --vat-status <VAT_STATUS>
          VAT status (VAT, EXEMPT, OUT_OF_SCOPE)

      --sales-tax-rate <SALES_TAX_RATE>
          Sales tax rate

      --second-sales-tax-rate <SECOND_SALES_TAX_RATE>
          Second sales tax rate

      --category <CATEGORY>
          Category URL

      --stock-item <STOCK_ITEM>
          Stock item URL

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### price-list-items delete

```
Delete a price list item

Usage: freeagent price-list-items delete [OPTIONS] <ID>

Arguments:
  <ID>
          Price list item ID

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
