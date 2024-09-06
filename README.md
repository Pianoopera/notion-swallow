# notion-swallow
cli to have the request converted to a request to NOTION

## Install

Download the corresponding binary

https://github.com/Pianoopera/notion-swallow/releases

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

```
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
notion_swallow pages -i 123-123 | sh
```