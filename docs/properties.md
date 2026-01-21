# properties

## Overview

```
Property management

Usage: freeagent properties [OPTIONS] <COMMAND>

Commands:
  list    List properties
  get     Get a property by ID
  create  Create a property (JSON payload)
  update  Update a property (JSON payload)
  delete  Delete a property
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

### properties list

```
List properties

Usage: freeagent properties list [OPTIONS]

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
### properties get

```
Get a property by ID

Usage: freeagent properties get [OPTIONS] <ID>

Arguments:
  <ID>
          Property ID

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
### properties create

```
Create a property (JSON payload)

Usage: freeagent properties create [OPTIONS]

Options:
      --address1 <ADDRESS1>
          Address line 1 (required)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --address2 <ADDRESS2>
          Address line 2

      --sandbox
          Use sandbox API instead of production

      --address3 <ADDRESS3>
          Address line 3

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --town <TOWN>
          Town

      --region <REGION>
          Region

      --postcode <POSTCODE>
          Postcode

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### properties update

```
Update a property (JSON payload)

Usage: freeagent properties update [OPTIONS] <ID>

Arguments:
  <ID>
          Property ID

Options:
      --address1 <ADDRESS1>
          Address line 1

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --address2 <ADDRESS2>
          Address line 2

      --sandbox
          Use sandbox API instead of production

      --address3 <ADDRESS3>
          Address line 3

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --town <TOWN>
          Town

      --region <REGION>
          Region

      --postcode <POSTCODE>
          Postcode

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### properties delete

```
Delete a property

Usage: freeagent properties delete [OPTIONS] <ID>

Arguments:
  <ID>
          Property ID

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
