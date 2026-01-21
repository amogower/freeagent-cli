# bank-transaction-explanations

## Overview

```
Bank transaction explanation management

Usage: freeagent bank-transaction-explanations [OPTIONS] <COMMAND>

Commands:
  list    List bank transaction explanations
  get     Get a bank transaction explanation by ID
  create  Create a bank transaction explanation (JSON payload)
  update  Update a bank transaction explanation (JSON payload)
  delete  Delete a bank transaction explanation
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

### bank-transaction-explanations list

```
List bank transaction explanations

Usage: freeagent bank-transaction-explanations list [OPTIONS] --bank-account <BANK_ACCOUNT>

Options:
      --bank-account <BANK_ACCOUNT>
          Bank account URL (required)

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

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --to-date <TO_DATE>
          Filter to date (YYYY-MM-DD)

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
### bank-transaction-explanations get

```
Get a bank transaction explanation by ID

Usage: freeagent bank-transaction-explanations get [OPTIONS] <ID>

Arguments:
  <ID>
          Explanation ID

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
### bank-transaction-explanations create

```
Create a bank transaction explanation (JSON payload)

Usage: freeagent bank-transaction-explanations create [OPTIONS]

Options:
      --bank-account <BANK_ACCOUNT>
          Bank account URL

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --bank-transaction <BANK_TRANSACTION>
          Bank transaction URL

      --sandbox
          Use sandbox API instead of production

      --dated-on <DATED_ON>
          Explanation date (YYYY-MM-DD)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --gross-value <GROSS_VALUE>
          Gross value

      --description <DESCRIPTION>
          Description

      --category <CATEGORY>
          Category URL

      --sales-tax-rate <SALES_TAX_RATE>
          Sales tax rate

      --sales-tax-value <SALES_TAX_VALUE>
          Sales tax value

      --sales-tax-status <SALES_TAX_STATUS>
          Sales tax status (TAXABLE, EXEMPT, OUT_OF_SCOPE)

      --second-sales-tax-rate <SECOND_SALES_TAX_RATE>
          Second sales tax rate

      --second-sales-tax-value <SECOND_SALES_TAX_VALUE>
          Second sales tax value

      --second-sales-tax-status <SECOND_SALES_TAX_STATUS>
          Second sales tax status

      --ec-status <EC_STATUS>
          EC status

      --place-of-supply <PLACE_OF_SUPPLY>
          Place of supply (for EC VAT MOSS)

      --cheque-number <CHEQUE_NUMBER>
          Cheque number

      --project <PROJECT>
          Project URL

      --rebill-type <REBILL_TYPE>
          Rebill type (cost, markup, price)

      --rebill-factor <REBILL_FACTOR>
          Rebill factor

      --receipt-reference <RECEIPT_REFERENCE>
          Receipt reference

      --paid-invoice <PAID_INVOICE>
          Paid invoice URL

      --paid-bill <PAID_BILL>
          Paid bill URL

      --paid-user <PAID_USER>
          Paid user URL

      --transfer-bank-account <TRANSFER_BANK_ACCOUNT>
          Transfer bank account URL

      --stock-item <STOCK_ITEM>
          Stock item URL

      --stock-altering-quantity <STOCK_ALTERING_QUANTITY>
          Stock altering quantity

      --disposed-asset <DISPOSED_ASSET>
          Disposed asset URL

      --property <PROPERTY>
          Property URL

      --direct-contact <DIRECT_CONTACT>
          Direct contact URL

      --attachment-json <ATTACHMENT_JSON>
          Attachment as JSON

      --capital-asset-json <CAPITAL_ASSET_JSON>
          Capital asset as JSON (for depreciation profile)

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### bank-transaction-explanations update

```
Update a bank transaction explanation (JSON payload)

Usage: freeagent bank-transaction-explanations update [OPTIONS] <ID>

Arguments:
  <ID>
          Explanation ID

Options:
      --bank-account <BANK_ACCOUNT>
          Bank account URL

  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --bank-transaction <BANK_TRANSACTION>
          Bank transaction URL

      --sandbox
          Use sandbox API instead of production

      --dated-on <DATED_ON>
          Explanation date (YYYY-MM-DD)

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --gross-value <GROSS_VALUE>
          Gross value

      --description <DESCRIPTION>
          Description

      --category <CATEGORY>
          Category URL

      --sales-tax-rate <SALES_TAX_RATE>
          Sales tax rate

      --sales-tax-value <SALES_TAX_VALUE>
          Sales tax value

      --sales-tax-status <SALES_TAX_STATUS>
          Sales tax status (TAXABLE, EXEMPT, OUT_OF_SCOPE)

      --second-sales-tax-rate <SECOND_SALES_TAX_RATE>
          Second sales tax rate

      --second-sales-tax-value <SECOND_SALES_TAX_VALUE>
          Second sales tax value

      --second-sales-tax-status <SECOND_SALES_TAX_STATUS>
          Second sales tax status

      --ec-status <EC_STATUS>
          EC status

      --place-of-supply <PLACE_OF_SUPPLY>
          Place of supply (for EC VAT MOSS)

      --cheque-number <CHEQUE_NUMBER>
          Cheque number

      --project <PROJECT>
          Project URL

      --rebill-type <REBILL_TYPE>
          Rebill type (cost, markup, price)

      --rebill-factor <REBILL_FACTOR>
          Rebill factor

      --receipt-reference <RECEIPT_REFERENCE>
          Receipt reference

      --paid-invoice <PAID_INVOICE>
          Paid invoice URL

      --paid-bill <PAID_BILL>
          Paid bill URL

      --paid-user <PAID_USER>
          Paid user URL

      --transfer-bank-account <TRANSFER_BANK_ACCOUNT>
          Transfer bank account URL

      --stock-item <STOCK_ITEM>
          Stock item URL

      --stock-altering-quantity <STOCK_ALTERING_QUANTITY>
          Stock altering quantity

      --disposed-asset <DISPOSED_ASSET>
          Disposed asset URL

      --property <PROPERTY>
          Property URL

      --direct-contact <DIRECT_CONTACT>
          Direct contact URL

      --attachment-json <ATTACHMENT_JSON>
          Attachment as JSON

      --capital-asset-json <CAPITAL_ASSET_JSON>
          Capital asset as JSON (for depreciation profile)

      --data <DATA>
          JSON body for the request

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### bank-transaction-explanations delete

```
Delete a bank transaction explanation

Usage: freeagent bank-transaction-explanations delete [OPTIONS] <ID>

Arguments:
  <ID>
          Explanation ID

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
