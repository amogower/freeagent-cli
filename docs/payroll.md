# payroll

## Overview

```
Payroll management

Usage: freeagent payroll [OPTIONS] <COMMAND>

Commands:
  list-year               List payroll for a year
  get-period              Get payroll for a period
  mark-payment-as-paid    Mark payroll payment as paid
  mark-payment-as-unpaid  Mark payroll payment as unpaid
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

### payroll list-year

```
List payroll for a year

Usage: freeagent payroll list-year [OPTIONS] <YEAR>

Arguments:
  <YEAR>
          Payroll year

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
### payroll get-period

```
Get payroll for a period

Usage: freeagent payroll get-period [OPTIONS] <YEAR> <PERIOD>

Arguments:
  <YEAR>
          Payroll year

  <PERIOD>
          Payroll period

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
### payroll mark-payment-as-paid

```
Mark payroll payment as paid

Usage: freeagent payroll mark-payment-as-paid [OPTIONS] <YEAR> <PAYMENT_DATE>

Arguments:
  <YEAR>
          Payroll year

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
### payroll mark-payment-as-unpaid

```
Mark payroll payment as unpaid

Usage: freeagent payroll mark-payment-as-unpaid [OPTIONS] <YEAR> <PAYMENT_DATE>

Arguments:
  <YEAR>
          Payroll year

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
