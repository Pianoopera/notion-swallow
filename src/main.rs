mod method;
mod args;
mod cmds;
mod subcommand;

use clap::App;
use cmds::{
    append_blocks_cmd, blocks_cmd, children_blocks_cmd, databases_cmd, i_cmd::ICommand, pages_cmd, property_pages_cmd, query_databases_cmd
};

fn main() {
    let app = App::new("rf-notion")
        .version("0.1.0")
        .author("teto <https://github.com/Pianoopera>")
        .about("Output Notion API URLs")
        .subcommands(
            vec![
                databases_cmd::databases_subcommand(),
                query_databases_cmd::query_databases_cmd(),
                pages_cmd::pages_subcommand(),
                property_pages_cmd::property_pages_subcommand(),
                append_blocks_cmd::append_blocks_subcommand(),
                blocks_cmd::blocks_subcommand(),
                children_blocks_cmd::children_blocks_subcommand(),
            ]
        );
    let arg_matches = app.get_matches();

    let notion_api_key = dotenv::var("NOTION_API_KEY").unwrap_or("default".to_string());
    let notion_version = dotenv::var("NOTION_VERSION").unwrap_or("2021-05-13".to_string());

    let subcommand = subcommand::NotionSubCommand::from_args(&arg_matches);
    match subcommand {
        subcommand::NotionSubCommand::Databases(databases) => {
            databases.print_curl(notion_api_key, notion_version);
        },
        subcommand::NotionSubCommand::QueryDatabases(query_databases) => {
            query_databases.print_curl(notion_api_key, notion_version);
        },
        subcommand::NotionSubCommand::Pages(pages) => {
            pages.print_curl(notion_api_key, notion_version);
        },
        subcommand::NotionSubCommand::PropertyPages(property_pages) => {
            property_pages.print_curl(notion_api_key, notion_version);
        },
        subcommand::NotionSubCommand::BlocksAppend(append_blocks) => {
            append_blocks.print_curl(notion_api_key, notion_version);
        },
        subcommand::NotionSubCommand::Blocks(blocks) => {
            blocks.print_curl(notion_api_key, notion_version);
        },
        subcommand::NotionSubCommand::ChildrenBlocks(children_blocks) => {
            children_blocks.print_curl(notion_api_key, notion_version);
        }
    }
}
