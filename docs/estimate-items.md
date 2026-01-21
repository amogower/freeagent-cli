# estimate-items

## Overview

```
Estimate item management

Usage: freeagent estimate-items [OPTIONS] <COMMAND>

Commands:
  create  Create an estimate item (JSON payload)
  update  Update an estimate item (JSON payload)
  delete  Delete an estimate item
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

### estimate-items create

```
Create an estimate item (JSON payload)

Usage: freeagent estimate-items create [OPTIONS]

Options:
      --estimate <ESTIMATE>
          Estimate URL

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

      --quantity <QUANTITY>
          Quantity

      --price <PRICE>
          Price

      --sales-tax-rate <SALES_TAX_RATE>
          Sales tax rate

      --sales-tax-value <SALES_TAX_VALUE>
          Sales tax value

      --sales-tax-status <SALES_TAX_STATUS>
          Sales tax status (TAXABLE, EXEMPT, OUT_OF_SCOPE)

      --second-sales-tax-rate <SECOND_SALES_TAX_RATE>
          Second sales tax rate

      --second-sales-tax-value <SECOND_SALES_TAX_VALUE>
          Second sales tax value

      --second-sales-tax-status <SECOND_SALES_TAX_STATUS>
          Second sales tax status

      --category <CATEGORY>
          Category URL

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### estimate-items update

```
Update an estimate item (JSON payload)

Usage: freeagent estimate-items update [OPTIONS] <ID>

Arguments:
  <ID>
          Estimate item ID

Options:
      --estimate <ESTIMATE>
          Estimate URL

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

      --quantity <QUANTITY>
          Quantity

      --price <PRICE>
          Price

      --sales-tax-rate <SALES_TAX_RATE>
          Sales tax rate

      --sales-tax-value <SALES_TAX_VALUE>
          Sales tax value

      --sales-tax-status <SALES_TAX_STATUS>
          Sales tax status (TAXABLE, EXEMPT, OUT_OF_SCOPE)

      --second-sales-tax-rate <SECOND_SALES_TAX_RATE>
          Second sales tax rate

      --second-sales-tax-value <SECOND_SALES_TAX_VALUE>
          Second sales tax value

      --second-sales-tax-status <SECOND_SALES_TAX_STATUS>
          Second sales tax status

      --category <CATEGORY>
          Category URL

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### estimate-items delete

```
Delete an estimate item

Usage: freeagent estimate-items delete [OPTIONS] <ID>

Arguments:
  <ID>
          Estimate item ID

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
