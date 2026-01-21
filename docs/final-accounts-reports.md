# final-accounts-reports

## Overview

```
Final accounts report management

Usage: freeagent final-accounts-reports [OPTIONS] <COMMAND>

Commands:
  list             List final accounts reports
  get              Get a final accounts report by period end date
  mark-as-filed    Mark a final accounts report as filed
  mark-as-unfiled  Mark a final accounts report as unfiled
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

### final-accounts-reports list

```
List final accounts reports

Usage: freeagent final-accounts-reports list [OPTIONS]

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
### final-accounts-reports get

```
Get a final accounts report by period end date

Usage: freeagent final-accounts-reports get [OPTIONS] <PERIOD_ENDS_ON>

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
### final-accounts-reports mark-as-filed

```
Mark a final accounts report as filed

Usage: freeagent final-accounts-reports mark-as-filed [OPTIONS] <PERIOD_ENDS_ON>

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
### final-accounts-reports mark-as-unfiled

```
Mark a final accounts report as unfiled

Usage: freeagent final-accounts-reports mark-as-unfiled [OPTIONS] <PERIOD_ENDS_ON>

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
