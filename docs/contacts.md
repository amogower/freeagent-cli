# contacts

## Overview

```
Contact management

Usage: freeagent contacts [OPTIONS] <COMMAND>

Commands:
  list    List all contacts
  get     Get a contact by ID
  create  Create a new contact
  update  Update a contact
  delete  Delete a contact
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

### contacts list

```
List all contacts

Usage: freeagent contacts list [OPTIONS]

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
          
          [possible values: all, active, clients, suppliers, active-projects, completed-projects, open-clients, open-suppliers, hidden]

      --sandbox
          Use sandbox API instead of production

      --sort <SORT>
          Sort field
          
          [possible values: name, created-at, updated-at]

      --desc
          Sort descending

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
### contacts get

```
Get a contact by ID

Usage: freeagent contacts get [OPTIONS] <ID>

Arguments:
  <ID>
          Contact ID

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
### contacts create

```
Create a new contact

Usage: freeagent contacts create [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --first-name <FIRST_NAME>
          First name

      --last-name <LAST_NAME>
          Last name

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --organisation-name <ORGANISATION_NAME>
          Organisation name

      --email <EMAIL>
          Email address

      --billing-email <BILLING_EMAIL>
          Billing email

      --phone-number <PHONE_NUMBER>
          Phone number

      --mobile <MOBILE>
          Mobile number

      --address1 <ADDRESS1>
          Address line 1

      --address2 <ADDRESS2>
          Address line 2

      --town <TOWN>
          Town/City

      --region <REGION>
          Region/State

      --postcode <POSTCODE>
          Postal code

      --country <COUNTRY>
          Country

      --payment-terms-in-days <PAYMENT_TERMS_IN_DAYS>
          Default payment terms in days

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### contacts update

```
Update a contact

Usage: freeagent contacts update [OPTIONS] <ID>

Arguments:
  <ID>
          Contact ID

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --first-name <FIRST_NAME>
          First name

      --last-name <LAST_NAME>
          Last name

      --sandbox
          Use sandbox API instead of production

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --organisation-name <ORGANISATION_NAME>
          Organisation name

      --email <EMAIL>
          Email address

      --phone-number <PHONE_NUMBER>
          Phone number

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### contacts delete

```
Delete a contact

Usage: freeagent contacts delete [OPTIONS] <ID>

Arguments:
  <ID>
          Contact ID

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
