# income-tax-returns

## Overview

```
Income tax return management

Usage: freeagent income-tax-returns [OPTIONS] <COMMAND>

Commands:
  list                    List self-assessment returns for a user
  get                     Get a self-assessment return for a user
  mark-as-filed           Mark a self-assessment return as filed
  mark-as-unfiled         Mark a self-assessment return as unfiled
  mark-payment-as-paid    Mark a self-assessment payment as paid
  mark-payment-as-unpaid  Mark a self-assessment payment as unpaid
  help                    Print this message or the help of the given subcommand(s)

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

### income-tax-returns list

```
List self-assessment returns for a user

Usage: freeagent income-tax-returns list [OPTIONS] <USER_ID>

Arguments:
  <USER_ID>
          User ID

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
### income-tax-returns get

```
Get a self-assessment return for a user

Usage: freeagent income-tax-returns get [OPTIONS] <USER_ID> <PERIOD_ENDS_ON>

Arguments:
  <USER_ID>
          User ID

  <PERIOD_ENDS_ON>
          Period end date (YYYY-MM-DD)

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
### income-tax-returns mark-as-filed

```
Mark a self-assessment return as filed

Usage: freeagent income-tax-returns mark-as-filed [OPTIONS] <USER_ID> <PERIOD_ENDS_ON>

Arguments:
  <USER_ID>
          User ID

  <PERIOD_ENDS_ON>
          Period end date (YYYY-MM-DD)

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
### income-tax-returns mark-as-unfiled

```
Mark a self-assessment return as unfiled

Usage: freeagent income-tax-returns mark-as-unfiled [OPTIONS] <USER_ID> <PERIOD_ENDS_ON>

Arguments:
  <USER_ID>
          User ID

  <PERIOD_ENDS_ON>
          Period end date (YYYY-MM-DD)

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
### income-tax-returns mark-payment-as-paid

```
Mark a self-assessment payment as paid

Usage: freeagent income-tax-returns mark-payment-as-paid [OPTIONS] <USER_ID> <PERIOD_ENDS_ON> <PAYMENT_DATE>

Arguments:
  <USER_ID>
          User ID

  <PERIOD_ENDS_ON>
          Period end date (YYYY-MM-DD)

  <PAYMENT_DATE>
          Payment date (YYYY-MM-DD)

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
### income-tax-returns mark-payment-as-unpaid

```
Mark a self-assessment payment as unpaid

Usage: freeagent income-tax-returns mark-payment-as-unpaid [OPTIONS] <USER_ID> <PERIOD_ENDS_ON> <PAYMENT_DATE>

Arguments:
  <USER_ID>
          User ID

  <PERIOD_ENDS_ON>
          Period end date (YYYY-MM-DD)

  <PAYMENT_DATE>
          Payment date (YYYY-MM-DD)

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
