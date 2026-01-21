# credit-notes

## Overview

```
Credit note management

Usage: freeagent credit-notes [OPTIONS] <COMMAND>

Commands:
  list    List all credit notes
  get     Get a credit note by ID
  create  Create a new credit note
  update  Update a credit note
  delete  Delete a credit note
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

### credit-notes list

```
List all credit notes

Usage: freeagent credit-notes list [OPTIONS]

Options:
      --contact <CONTACT>
          Filter by contact URL

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --project <PROJECT>
          Filter by project URL

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --page <PAGE>
          Page number

      --per-page <PER_PAGE>
          Items per page

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### credit-notes get

```
Get a credit note by ID

Usage: freeagent credit-notes get [OPTIONS] <ID>

Arguments:
  <ID>
          Credit note ID

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
### credit-notes create

```
Create a new credit note

Usage: freeagent credit-notes create [OPTIONS] --contact <CONTACT>

Options:
      --contact <CONTACT>
          Contact URL (required)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --dated-on <DATED_ON>
          Credit note date (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --reference <REFERENCE>
          Reference

      --currency <CURRENCY>
          Currency code

      --comments <COMMENTS>
          Comments

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### credit-notes update

```
Update a credit note

Usage: freeagent credit-notes update [OPTIONS] <ID>

Arguments:
  <ID>
          Credit note ID

Options:
      --dated-on <DATED_ON>
          Credit note date (YYYY-MM-DD)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --reference <REFERENCE>
          Reference

      --sandbox
          Use sandbox API instead of production

      --comments <COMMENTS>
          Comments

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### credit-notes delete

```
Delete a credit note

Usage: freeagent credit-notes delete [OPTIONS] <ID>

Arguments:
  <ID>
          Credit note ID

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
