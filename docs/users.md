# users

## Overview

```
User management

Usage: freeagent users [OPTIONS] <COMMAND>

Commands:
  list       List all users
  me         Get current user
  get        Get a user by ID
  create     Create a new user
  update     Update a user
  update-me  Update the current user
  delete     Delete a user
  help       Print this message or the help of the given subcommand(s)

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

### users list

```
List all users

Usage: freeagent users list [OPTIONS]

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
### users me

```
Get current user

Usage: freeagent users me [OPTIONS]

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
### users get

```
Get a user by ID

Usage: freeagent users get [OPTIONS] <ID>

Arguments:
  <ID>
          User ID

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
### users create

```
Create a new user

Usage: freeagent users create [OPTIONS] --first-name <FIRST_NAME> --last-name <LAST_NAME> --email <EMAIL>

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

      --email <EMAIL>
          Email address

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --role <ROLE>
          User role

      --permission-level <PERMISSION_LEVEL>
          Permission level (0-8)

      --opening-mileage <OPENING_MILEAGE>
          Opening mileage

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### users update

```
Update a user

Usage: freeagent users update [OPTIONS] <ID>

Arguments:
  <ID>
          User ID

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

      --email <EMAIL>
          Email address

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --role <ROLE>
          User role

      --permission-level <PERMISSION_LEVEL>
          Permission level (0-8)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### users update-me

```
Update the current user

Usage: freeagent users update-me [OPTIONS]

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

      --email <EMAIL>
          Email address

      --no-update
          Disable automatic update checks
          
          [env: FREEAGENT_NO_UPDATE=]

      --role <ROLE>
          User role

      --permission-level <PERMISSION_LEVEL>
          Permission level (0-8)

      --opening-mileage <OPENING_MILEAGE>
          Opening mileage

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
### users delete

```
Delete a user

Usage: freeagent users delete [OPTIONS] <ID>

Arguments:
  <ID>
          User ID

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
