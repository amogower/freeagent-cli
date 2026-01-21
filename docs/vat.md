# vat

## Overview

```
VAT return management

Usage: freeagent vat [OPTIONS] <COMMAND>

Commands:
  list                    List all VAT returns
  get                     Get a VAT return by ID
  mark-as-filed           Mark VAT return as filed
  mark-as-unfiled         Mark VAT return as unfiled
  mark-payment-as-paid    Mark a VAT return payment as paid
  mark-payment-as-unpaid  Mark a VAT return payment as unpaid
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

### vat list

```
List all VAT returns

Usage: freeagent vat list [OPTIONS]

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
### vat get

```
Get a VAT return by ID

Usage: freeagent vat get [OPTIONS] <ID>

Arguments:
  <ID>
          VAT return ID

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
### vat mark-as-filed

```
Mark VAT return as filed

Usage: freeagent vat mark-as-filed [OPTIONS] <ID>

Arguments:
  <ID>
          VAT return ID

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
### vat mark-as-unfiled

```
Mark VAT return as unfiled

Usage: freeagent vat mark-as-unfiled [OPTIONS] <ID>

Arguments:
  <ID>
          VAT return ID

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
### vat mark-payment-as-paid

```
Mark a VAT return payment as paid

Usage: freeagent vat mark-payment-as-paid [OPTIONS] <PERIOD_ENDS_ON> <PAYMENT_DATE>

Arguments:
  <PERIOD_ENDS_ON>
          VAT return period end date (YYYY-MM-DD)

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
### vat mark-payment-as-unpaid

```
Mark a VAT return payment as unpaid

Usage: freeagent vat mark-payment-as-unpaid [OPTIONS] <PERIOD_ENDS_ON> <PAYMENT_DATE>

Arguments:
  <PERIOD_ENDS_ON>
          VAT return period end date (YYYY-MM-DD)

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
