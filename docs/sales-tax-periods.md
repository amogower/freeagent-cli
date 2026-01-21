# sales-tax-periods

## Overview

```
Sales tax period management

Usage: freeagent sales-tax-periods [OPTIONS] <COMMAND>

Commands:
  list    List sales tax periods
  get     Get a sales tax period by ID
  create  Create a sales tax period (JSON payload)
  update  Update a sales tax period (JSON payload)
  delete  Delete a sales tax period
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

### sales-tax-periods list

```
List sales tax periods

Usage: freeagent sales-tax-periods list [OPTIONS]

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
### sales-tax-periods get

```
Get a sales tax period by ID

Usage: freeagent sales-tax-periods get [OPTIONS] <ID>

Arguments:
  <ID>
          Sales tax period ID

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
### sales-tax-periods create

```
Create a sales tax period (JSON payload)

Usage: freeagent sales-tax-periods create [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --sales-tax-name <SALES_TAX_NAME>
          Sales tax name (e.g., VAT, GST, Sales Tax)

      --sales-tax-registration-status <SALES_TAX_REGISTRATION_STATUS>
          Sales tax registration status (Registered, Not Registered)

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --sales-tax-rate-1 <SALES_TAX_RATE_1>
          First sales tax rate

      --sales-tax-rate-2 <SALES_TAX_RATE_2>
          Second sales tax rate

      --sales-tax-rate-3 <SALES_TAX_RATE_3>
          Third sales tax rate

      --sales-tax-is-value-added <SALES_TAX_IS_VALUE_ADDED>
          True if tax is value-added
          
          [possible values: true, false]

      --sales-tax-registration-number <SALES_TAX_REGISTRATION_NUMBER>
          Sales tax registration number

      --effective-date <EFFECTIVE_DATE>
          Effective date (YYYY-MM-DD)

      --second-sales-tax-name <SECOND_SALES_TAX_NAME>
          Second sales tax name (Universal accounts)

      --second-sales-tax-rate-1 <SECOND_SALES_TAX_RATE_1>
          Second sales tax rate 1 (Universal accounts)

      --second-sales-tax-rate-2 <SECOND_SALES_TAX_RATE_2>
          Second sales tax rate 2 (Universal accounts)

      --second-sales-tax-rate-3 <SECOND_SALES_TAX_RATE_3>
          Second sales tax rate 3 (Universal accounts)

      --second-sales-tax-is-compound <SECOND_SALES_TAX_IS_COMPOUND>
          Second sales tax is compound (Universal accounts)
          
          [possible values: true, false]

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### sales-tax-periods update

```
Update a sales tax period (JSON payload)

Usage: freeagent sales-tax-periods update [OPTIONS] <ID>

Arguments:
  <ID>
          Sales tax period ID

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --sales-tax-name <SALES_TAX_NAME>
          Sales tax name (e.g., VAT, GST, Sales Tax)

      --sales-tax-registration-status <SALES_TAX_REGISTRATION_STATUS>
          Sales tax registration status (Registered, Not Registered)

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --sales-tax-rate-1 <SALES_TAX_RATE_1>
          First sales tax rate

      --sales-tax-rate-2 <SALES_TAX_RATE_2>
          Second sales tax rate

      --sales-tax-rate-3 <SALES_TAX_RATE_3>
          Third sales tax rate

      --sales-tax-is-value-added <SALES_TAX_IS_VALUE_ADDED>
          True if tax is value-added
          
          [possible values: true, false]

      --sales-tax-registration-number <SALES_TAX_REGISTRATION_NUMBER>
          Sales tax registration number

      --effective-date <EFFECTIVE_DATE>
          Effective date (YYYY-MM-DD)

      --second-sales-tax-name <SECOND_SALES_TAX_NAME>
          Second sales tax name (Universal accounts)

      --second-sales-tax-rate-1 <SECOND_SALES_TAX_RATE_1>
          Second sales tax rate 1 (Universal accounts)

      --second-sales-tax-rate-2 <SECOND_SALES_TAX_RATE_2>
          Second sales tax rate 2 (Universal accounts)

      --second-sales-tax-rate-3 <SECOND_SALES_TAX_RATE_3>
          Second sales tax rate 3 (Universal accounts)

      --second-sales-tax-is-compound <SECOND_SALES_TAX_IS_COMPOUND>
          Second sales tax is compound (Universal accounts)
          
          [possible values: true, false]

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### sales-tax-periods delete

```
Delete a sales tax period

Usage: freeagent sales-tax-periods delete [OPTIONS] <ID>

Arguments:
  <ID>
          Sales tax period ID

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
