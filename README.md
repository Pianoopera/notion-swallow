# notion-swallow
cli to have the request converted to a request to NOTION

```sh
$ notion-swallow pages -i 123-123
curl -X GET 'https://api.notion.com/v1/pages/123-123' \
 -H 'Authorization: Bearer secret_123' \
 -H 'Notion-Version: 2022-06-28'
```

## Install

Download the corresponding binary

https://github.com/Pianoopera/notion-swallow/releases

```sh
mv download/file/name notion-swallow
```

### for mac

```
brew install Pianoopera/tap/notion-swallow
```

## Setup

Set the secret key in the environment variable named `NOTION_SECRET_KEY`,`NOTION_VERSION`

```sh
export NOTION_SECRET_KEY="secret_123"
export NOTION_VERSION="2022-06-28"
```

## Usage

```sh
notion-swallow 0.3.3
teto <https://github.com/Pianoopera>
Output Notion API URLs

USAGE:
    notion-swallow [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    append_blocks      Output Notion API URLs for append blocks
    blocks             Output Notion API URLs for blocks
    children_blocks    Output Notion API URLs for children blocks
    databases          Output Notion API URLs for databases
    help               Prints this shmessage or the help of the given subcommand(s)
    pages              Output Notion API URLs for pages
    property_pages     Output Notion API URLs for property pages
    query_databases    Output Notion API URLs for query databases
    search             Output Notion API URLs for search
```

## CURL

If you want to run curl, use the `sh` command in `|`.

example

```sh
notion-swallow pages -i 123-123 | sh
```

## Subcommands

subcommand details

### pages

```sh
Output Notion API URLs for pages

Usage: notion-swallow pages [OPTIONS]

Options:
  -x, --x <METHOD>   Set the method of Notion API
  -i, --id <ID>      Set the id of Notion API
  -f, --file <FILE>  Set the file of Notion API
  -h, --help         Print help
```

### property_pages

```sh
Output Notion API URLs for property pages

Usage: notion-swallow property_pages [OPTIONS]

Options:
  -x, --x <METHOD>                 Set the method of Notion API
  -i, --id <ID>                    Set the id of Notion API
  -p, --property_id <PROPERTY_ID>  Set the property_id of Notion API
  -h, --help                       Print help
```

### query_databases

```sh
Output Notion API URLs for query databases

Usage: notion-swallow query_databases [OPTIONS]

Options:
  -i, --id <ID>      Set the id of Notion API
  -f, --file <FILE>  Set the file of Notion API
  -h, --help         Print help
```

### search

```sh
Output Notion API URLs for query databases

Usage: notion-swallow search [OPTIONS]

Options:
  -f, --file <FILE>  Set the file of Notion API
  -h, --help         Print help
```

### blocks

```sh
Output Notion API URLs for blocks

Usage: notion-swallow blocks [OPTIONS]

Options:
  -x, --x <METHOD>   Set the method of Notion API
  -i, --id <ID>      Set the id of Notion API
  -f, --file <FILE>  Set the file of Notion API
  -h, --help         Print help
```

### append_blocks

```sh
Output Notion API URLs for append blocks

Usage: notion-swallow append_blocks [OPTIONS]

Options:
  -x, --x <METHOD>   Set the method of Notion API
  -i, --id <ID>      Set the id of Notion API
  -f, --file <FILE>  Set the file of Notion API
  -h, --help         Print help

```

### children_blocks

```sh
Output Notion API URLs for children blocks

Usage: notion-swallow children_blocks [OPTIONS]

Options:
  -x, --x <METHOD>             Set the method of Notion API
  -i, --id <ID>                Set the id of Notion API
  -p, --page_size <PAGE_SIZE>  Set the page size of Notion API
  -h, --help                   Print help

```

### databases

```sh
Output Notion API URLs for databases

Usage: notion-swallow databases [OPTIONS]

Options:
  -i, --id <ID>      Set the id of Notion API
  -x, --x <METHOD>   Set the method of Notion API
  -f, --file <FILE>  Set the file of Notion API
  -h, --help         Print help

```