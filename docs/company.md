# company

## Overview

```
Company management

Usage: freeagent company [OPTIONS] <COMMAND>

Commands:
  get           Get company details
  update        Update company details
  tax-timeline  Get company tax timeline
  help          Print this message or the help of the given subcommand(s)

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

### company get

```
Get company details

Usage: freeagent company get [OPTIONS]

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
### company update

```
Update company details

Usage: freeagent company update [OPTIONS]

Options:
  -f, --format <FORMAT>
          Output format

          Possible values:
          - json:    JSON output (default)
          - table:   Table output for lists
          - compact: Compact JSON (single line)
          
          [default: json]

      --name <NAME>
          Company name

      --company-type <COMPANY_TYPE>
          Company type

      --sandbox
          Use sandbox API instead of production

      --currency <CURRENCY>
          Currency code

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --mileage-units <MILEAGE_UNITS>
          Mileage units (miles or km)

      --company-registration-number <COMPANY_REGISTRATION_NUMBER>
          Company registration number

      --sales-tax-registration-number <SALES_TAX_REGISTRATION_NUMBER>
          Sales tax registration number

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### company tax-timeline

```
Get company tax timeline

Usage: freeagent company tax-timeline [OPTIONS]

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
