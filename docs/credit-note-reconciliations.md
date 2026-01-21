# credit-note-reconciliations

## Overview

```
Credit note reconciliation management

Usage: freeagent credit-note-reconciliations [OPTIONS] <COMMAND>

Commands:
  list    List credit note reconciliations
  get     Get a credit note reconciliation by ID
  create  Create a credit note reconciliation (JSON payload)
  update  Update a credit note reconciliation (JSON payload)
  delete  Delete a credit note reconciliation
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

### credit-note-reconciliations list

```
List credit note reconciliations

Usage: freeagent credit-note-reconciliations list [OPTIONS]

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

      --page <PAGE>
          Page number

      --per-page <PER_PAGE>
          Items per page

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### credit-note-reconciliations get

```
Get a credit note reconciliation by ID

Usage: freeagent credit-note-reconciliations get [OPTIONS] <ID>

Arguments:
  <ID>
          Reconciliation ID

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
### credit-note-reconciliations create

```
Create a credit note reconciliation (JSON payload)

Usage: freeagent credit-note-reconciliations create [OPTIONS]

Options:
      --credit-note <CREDIT_NOTE>
          Credit note URL

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --invoice <INVOICE>
          Invoice URL

      --sandbox
          Use sandbox API instead of production

      --gross-value <GROSS_VALUE>
          Gross value

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --dated-on <DATED_ON>
          Reconciliation date (YYYY-MM-DD)

      --exchange-rate <EXCHANGE_RATE>
          Exchange rate

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### credit-note-reconciliations update

```
Update a credit note reconciliation (JSON payload)

Usage: freeagent credit-note-reconciliations update [OPTIONS] <ID>

Arguments:
  <ID>
          Reconciliation ID

Options:
      --credit-note <CREDIT_NOTE>
          Credit note URL

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --invoice <INVOICE>
          Invoice URL

      --sandbox
          Use sandbox API instead of production

      --gross-value <GROSS_VALUE>
          Gross value

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --dated-on <DATED_ON>
          Reconciliation date (YYYY-MM-DD)

      --exchange-rate <EXCHANGE_RATE>
          Exchange rate

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### credit-note-reconciliations delete

```
Delete a credit note reconciliation

Usage: freeagent credit-note-reconciliations delete [OPTIONS] <ID>

Arguments:
  <ID>
          Reconciliation ID

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
