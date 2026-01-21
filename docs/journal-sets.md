# journal-sets

## Overview

```
Journal set management

Usage: freeagent journal-sets [OPTIONS] <COMMAND>

Commands:
  list              List journal sets
  get               Get a journal set by ID
  opening-balances  Get opening balances
  create            Create a journal set (JSON payload)
  update            Update a journal set (JSON payload)
  delete            Delete a journal set
  help              Print this message or the help of the given subcommand(s)

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

### journal-sets list

```
List journal sets

Usage: freeagent journal-sets list [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --from-date <FROM_DATE>
          Filter from date (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --to-date <TO_DATE>
          Filter to date (YYYY-MM-DD)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --updated-since <UPDATED_SINCE>
          Filter by update date (ISO 8601)

      --tag <TAG>
          Filter by tag

      --page <PAGE>
          Page number

      --per-page <PER_PAGE>
          Items per page

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### journal-sets get

```
Get a journal set by ID

Usage: freeagent journal-sets get [OPTIONS] <ID>

Arguments:
  <ID>
          Journal set ID

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
### journal-sets opening-balances

```
Get opening balances

Usage: freeagent journal-sets opening-balances [OPTIONS]

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
### journal-sets create

```
Create a journal set (JSON payload)

Usage: freeagent journal-sets create [OPTIONS]

Options:
      --description <DESCRIPTION>
          Description (required)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --dated-on <DATED_ON>
          Dated on (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --tag <TAG>
          Tag

      --journal-entries-json <JOURNAL_ENTRIES_JSON>
          Journal entries as JSON array

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### journal-sets update

```
Update a journal set (JSON payload)

Usage: freeagent journal-sets update [OPTIONS] <ID>

Arguments:
  <ID>
          Journal set ID

Options:
      --description <DESCRIPTION>
          Description

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --dated-on <DATED_ON>
          Dated on (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --tag <TAG>
          Tag

      --journal-entries-json <JOURNAL_ENTRIES_JSON>
          Journal entries as JSON array

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### journal-sets delete

```
Delete a journal set

Usage: freeagent journal-sets delete [OPTIONS] <ID>

Arguments:
  <ID>
          Journal set ID

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
