mod args;
mod cmds;
mod method;
mod subcommand;

use std::env;

use clap::Command;
use cmds::{
    append_blocks_cmd, blocks_cmd, children_blocks_cmd, databases_cmd, i_cmd::ICommand, pages_cmd,
    property_pages_cmd, query_databases_cmd, search_cmd,
};

fn main() {
    let mut app = Command::new("notion-swallow")
        .version("0.3.5")
        .author("teto <https://github.com/Pianoopera>")
        .about("Output Notion API URLs")
        .subcommands(vec![
            databases_cmd::databases_subcommand(),
            query_databases_cmd::query_databases_cmd(),
            pages_cmd::pages_subcommand(),
            property_pages_cmd::property_pages_subcommand(),
            append_blocks_cmd::append_blocks_subcommand(),
            blocks_cmd::blocks_subcommand(),
            children_blocks_cmd::children_blocks_subcommand(),
            search_cmd::search_subcommand(),
        ]);
    let arg_matches = app.clone().get_matches();

    // サブコマンドが指定されていない場合はヘルプを表示
    if arg_matches.subcommand_name().is_none() {
        app.print_help().unwrap();
        return;
    }

    let notion_api_key = env::var("NOTION_SECRET_KEY").unwrap_or("secret_123".to_string());
    let notion_version = env::var("NOTION_VERSION").unwrap_or("2022-06-28".to_string());

    let subcommand = subcommand::NotionSubCommand::from_args(&arg_matches);
    match subcommand {
        subcommand::NotionSubCommand::Databases(databases) => {
            databases.print_curl(notion_api_key, notion_version);
        }
        subcommand::NotionSubCommand::QueryDatabases(query_databases) => {
            query_databases.print_curl(notion_api_key, notion_version);
        }
        subcommand::NotionSubCommand::Pages(pages) => {
            pages.print_curl(notion_api_key, notion_version);
        }
        subcommand::NotionSubCommand::PropertyPages(property_pages) => {
            property_pages.print_curl(notion_api_key, notion_version);
        }
        subcommand::NotionSubCommand::BlocksAppend(append_blocks) => {
            append_blocks.print_curl(notion_api_key, notion_version);
        }
        subcommand::NotionSubCommand::Blocks(blocks) => {
            blocks.print_curl(notion_api_key, notion_version);
        }
        subcommand::NotionSubCommand::ChildrenBlocks(children_blocks) => {
            children_blocks.print_curl(notion_api_key, notion_version);
        }
        subcommand::NotionSubCommand::Search(search) => {
            search.print_curl(notion_api_key, notion_version);
        }
    }
}
