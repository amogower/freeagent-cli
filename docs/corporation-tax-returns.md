# corporation-tax-returns

## Overview

```
Corporation tax return management

Usage: freeagent corporation-tax-returns [OPTIONS] <COMMAND>

Commands:
  list             List corporation tax returns
  get              Get a corporation tax return by period end date
  mark-as-filed    Mark a corporation tax return as filed
  mark-as-unfiled  Mark a corporation tax return as unfiled
  mark-as-paid     Mark a corporation tax return as paid
  mark-as-unpaid   Mark a corporation tax return as unpaid
  help             Print this message or the help of the given subcommand(s)

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

### corporation-tax-returns list

```
List corporation tax returns

Usage: freeagent corporation-tax-returns list [OPTIONS]

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
### corporation-tax-returns get

```
Get a corporation tax return by period end date

Usage: freeagent corporation-tax-returns get [OPTIONS] <PERIOD_ENDS_ON>

Arguments:
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
### corporation-tax-returns mark-as-filed

```
Mark a corporation tax return as filed

Usage: freeagent corporation-tax-returns mark-as-filed [OPTIONS] <PERIOD_ENDS_ON>

Arguments:
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
### corporation-tax-returns mark-as-unfiled

```
Mark a corporation tax return as unfiled

Usage: freeagent corporation-tax-returns mark-as-unfiled [OPTIONS] <PERIOD_ENDS_ON>

Arguments:
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
### corporation-tax-returns mark-as-paid

```
Mark a corporation tax return as paid

Usage: freeagent corporation-tax-returns mark-as-paid [OPTIONS] <PERIOD_ENDS_ON>

Arguments:
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
### corporation-tax-returns mark-as-unpaid

```
Mark a corporation tax return as unpaid

Usage: freeagent corporation-tax-returns mark-as-unpaid [OPTIONS] <PERIOD_ENDS_ON>

Arguments:
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
