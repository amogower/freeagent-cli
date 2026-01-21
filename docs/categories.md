# categories

## Overview

```
Category management

Usage: freeagent categories [OPTIONS] <COMMAND>

Commands:
  list    List all categories
  get     Get a category by nominal code
  create  Create a new category (JSON payload)
  update  Update a category (JSON payload)
  delete  Delete a category
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

### categories list

```
List all categories

Usage: freeagent categories list [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --sub-accounts
          Include sub accounts in the list

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
### categories get

```
Get a category by nominal code

Usage: freeagent categories get [OPTIONS] <NOMINAL_CODE>

Arguments:
  <NOMINAL_CODE>
          Category nominal code

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
### categories create

```
Create a new category (JSON payload)

Usage: freeagent categories create [OPTIONS]

Options:
      --description <DESCRIPTION>
          Category description

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --nominal-code <NOMINAL_CODE>
          Category nominal code

      --sandbox
          Use sandbox API instead of production

      --category-group <CATEGORY_GROUP>
          Category group (income, cost_of_sales, admin_expenses)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --tax-reporting-name <TAX_REPORTING_NAME>
          Tax reporting name

      --allowable-for-tax <ALLOWABLE_FOR_TAX>
          Allowable for tax
          
          [possible values: true, false]

      --auto-sales-tax-rate <AUTO_SALES_TAX_RATE>
          Automatic sales tax rate

      --data <DATA>
          JSON body for the category request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### categories update

```
Update a category (JSON payload)

Usage: freeagent categories update [OPTIONS] <NOMINAL_CODE>

Arguments:
  <NOMINAL_CODE>
          Category nominal code

Options:
      --description <DESCRIPTION>
          Category description

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --category-group <CATEGORY_GROUP>
          Category group (income, cost_of_sales, admin_expenses)

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --tax-reporting-name <TAX_REPORTING_NAME>
          Tax reporting name

      --allowable-for-tax <ALLOWABLE_FOR_TAX>
          Allowable for tax
          
          [possible values: true, false]

      --auto-sales-tax-rate <AUTO_SALES_TAX_RATE>
          Automatic sales tax rate

      --data <DATA>
          JSON body for the category request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### categories delete

```
Delete a category

Usage: freeagent categories delete [OPTIONS] <NOMINAL_CODE>

Arguments:
  <NOMINAL_CODE>
          Category nominal code

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
