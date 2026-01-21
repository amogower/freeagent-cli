# estimates

## Overview

```
Estimate management

Usage: freeagent estimates [OPTIONS] <COMMAND>

Commands:
  list                            List all estimates
  get                             Get an estimate by ID
  create                          Create a new estimate
  update                          Update an estimate
  delete                          Delete an estimate
  send-email                      Send estimate by email
  update-default-additional-text  Update default additional text (JSON payload)
  delete-default-additional-text  Delete default additional text
  help                            Print this message or the help of the given subcommand(s)

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

### estimates list

```
List all estimates

Usage: freeagent estimates list [OPTIONS]

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
### estimates get

```
Get an estimate by ID

Usage: freeagent estimates get [OPTIONS] <ID>

Arguments:
  <ID>
          Estimate ID

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
### estimates create

```
Create a new estimate

Usage: freeagent estimates create [OPTIONS] --contact <CONTACT>

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
          Estimate date (YYYY-MM-DD)

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
### estimates update

```
Update an estimate

Usage: freeagent estimates update [OPTIONS] <ID>

Arguments:
  <ID>
          Estimate ID

Options:
      --dated-on <DATED_ON>
          Estimate date (YYYY-MM-DD)

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
### estimates delete

```
Delete an estimate

Usage: freeagent estimates delete [OPTIONS] <ID>

Arguments:
  <ID>
          Estimate ID

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
### estimates send-email

```
Send estimate by email

Usage: freeagent estimates send-email [OPTIONS] --email-to <EMAIL_TO> <ID>

Arguments:
  <ID>
          Estimate ID

Options:
      --email-to <EMAIL_TO>
          Email recipient

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
### estimates update-default-additional-text

```
Update default additional text (JSON payload)

Usage: freeagent estimates update-default-additional-text [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --text <TEXT>
          Default additional text

      --data <DATA>
          JSON body for the request

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
### estimates delete-default-additional-text

```
Delete default additional text

Usage: freeagent estimates delete-default-additional-text [OPTIONS]

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
