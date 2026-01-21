# timeslips

## Overview

```
Timeslip management

Usage: freeagent timeslips [OPTIONS] <COMMAND>

Commands:
  list         List all timeslips
  get          Get a timeslip by ID
  create       Create a new timeslip
  update       Update a timeslip
  delete       Delete a timeslip
  start-timer  Start timer for a timeslip
  stop-timer   Stop timer for a timeslip
  help         Print this message or the help of the given subcommand(s)

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

### timeslips list

```
List all timeslips

Usage: freeagent timeslips list [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --user <USER>
          Filter by user URL

      --project <PROJECT>
          Filter by project URL

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --task <TASK>
          Filter by task URL

      --from-date <FROM_DATE>
          Filter from date (YYYY-MM-DD)

      --to-date <TO_DATE>
          Filter to date (YYYY-MM-DD)

      --page <PAGE>
          Page number

      --per-page <PER_PAGE>
          Items per page

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### timeslips get

```
Get a timeslip by ID

Usage: freeagent timeslips get [OPTIONS] <ID>

Arguments:
  <ID>
          Timeslip ID

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
### timeslips create

```
Create a new timeslip

Usage: freeagent timeslips create [OPTIONS] --user <USER> --project <PROJECT> --task <TASK> --dated-on <DATED_ON>

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --user <USER>
          User URL (required)

      --project <PROJECT>
          Project URL (required)

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --task <TASK>
          Task URL (required)

      --dated-on <DATED_ON>
          Date (YYYY-MM-DD) (required)

      --hours <HOURS>
          Hours worked

      --minutes <MINUTES>
          Minutes worked

      --comment <COMMENT>
          Comment

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### timeslips update

```
Update a timeslip

Usage: freeagent timeslips update [OPTIONS] <ID>

Arguments:
  <ID>
          Timeslip ID

Options:
      --dated-on <DATED_ON>
          Date (YYYY-MM-DD)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --hours <HOURS>
          Hours worked

      --sandbox
          Use sandbox API instead of production

      --comment <COMMENT>
          Comment

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### timeslips delete

```
Delete a timeslip

Usage: freeagent timeslips delete [OPTIONS] <ID>

Arguments:
  <ID>
          Timeslip ID

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
### timeslips start-timer

```
Start timer for a timeslip

Usage: freeagent timeslips start-timer [OPTIONS] <ID>

Arguments:
  <ID>
          Timeslip ID

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
### timeslips stop-timer

```
Stop timer for a timeslip

Usage: freeagent timeslips stop-timer [OPTIONS] <ID>

Arguments:
  <ID>
          Timeslip ID

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
