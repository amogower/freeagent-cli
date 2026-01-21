# invoices

## Overview

```
Invoice management

Usage: freeagent invoices [OPTIONS] <COMMAND>

Commands:
  list                            List all invoices
  get                             Get an invoice by ID
  create                          Create a new invoice
  update                          Update an invoice
  delete                          Delete an invoice
  pdf                             Get invoice PDF URL
  send-email                      Send invoice by email
  mark-as-sent                    Mark invoice as sent
  mark-as-draft                   Mark invoice as draft
  mark-as-cancelled               Mark invoice as cancelled
  mark-as-scheduled               Mark invoice as scheduled
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

### invoices list

```
List all invoices

Usage: freeagent invoices list [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --view <VIEW>
          Filter by view
          
          [possible values: recent-open-or-overdue, open-or-overdue, open, overdue, draft, scheduled-to-email, thank-you, reminded, all]

      --contact <CONTACT>
          Filter by contact URL

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --project <PROJECT>
          Filter by project URL

      --from-date <FROM_DATE>
          Filter from date (YYYY-MM-DD)

      --to-date <TO_DATE>
          Filter to date (YYYY-MM-DD)

      --updated-since <UPDATED_SINCE>
          Filter by update date (ISO 8601)

      --nested-invoice-items
          Include nested invoice items

      --page <PAGE>
          Page number

      --per-page <PER_PAGE>
          Items per page

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### invoices get

```
Get an invoice by ID

Usage: freeagent invoices get [OPTIONS] <ID>

Arguments:
  <ID>
          Invoice ID

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --nested-invoice-items
          Include nested invoice items

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
### invoices create

```
Create a new invoice

Usage: freeagent invoices create [OPTIONS] --contact <CONTACT>

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

      --project <PROJECT>
          Project URL

      --sandbox
          Use sandbox API instead of production

      --dated-on <DATED_ON>
          Invoice date (YYYY-MM-DD)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --due-on <DUE_ON>
          Due date (YYYY-MM-DD)

      --reference <REFERENCE>
          Invoice reference

      --currency <CURRENCY>
          Currency code

      --payment-terms-in-days <PAYMENT_TERMS_IN_DAYS>
          Payment terms in days

      --ec-status <EC_STATUS>
          EC status
          
          [possible values: uk, ec-goods, ec-services, ec-moss, non-ec]

      --comments <COMMENTS>
          Comments

      --items-json <ITEMS_JSON>
          Invoice items as JSON array

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### invoices update

```
Update an invoice

Usage: freeagent invoices update [OPTIONS] <ID>

Arguments:
  <ID>
          Invoice ID

Options:
      --dated-on <DATED_ON>
          Invoice date (YYYY-MM-DD)

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --due-on <DUE_ON>
          Due date (YYYY-MM-DD)

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --reference <REFERENCE>
          Invoice reference

      --comments <COMMENTS>
          Comments

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### invoices delete

```
Delete an invoice

Usage: freeagent invoices delete [OPTIONS] <ID>

Arguments:
  <ID>
          Invoice ID

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
### invoices pdf

```
Get invoice PDF URL

Usage: freeagent invoices pdf [OPTIONS] <ID>

Arguments:
  <ID>
          Invoice ID

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
### invoices send-email

```
Send invoice by email

Usage: freeagent invoices send-email [OPTIONS] --email-to <EMAIL_TO> <ID>

Arguments:
  <ID>
          Invoice ID

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

      --subject <SUBJECT>
          Email subject

      --body <BODY>
          Email body

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### invoices mark-as-sent

```
Mark invoice as sent

Usage: freeagent invoices mark-as-sent [OPTIONS] <ID>

Arguments:
  <ID>
          Invoice ID

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
### invoices mark-as-draft

```
Mark invoice as draft

Usage: freeagent invoices mark-as-draft [OPTIONS] <ID>

Arguments:
  <ID>
          Invoice ID

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
### invoices mark-as-cancelled

```
Mark invoice as cancelled

Usage: freeagent invoices mark-as-cancelled [OPTIONS] <ID>

Arguments:
  <ID>
          Invoice ID

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
### invoices mark-as-scheduled

```
Mark invoice as scheduled

Usage: freeagent invoices mark-as-scheduled [OPTIONS] <ID>

Arguments:
  <ID>
          Invoice ID

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
### invoices update-default-additional-text

```
Update default additional text (JSON payload)

Usage: freeagent invoices update-default-additional-text [OPTIONS]

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
### invoices delete-default-additional-text

```
Delete default additional text

Usage: freeagent invoices delete-default-additional-text [OPTIONS]

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
