mod args;
mod cmds;
mod method;
mod subcommand;

use std::env;

use clap::Command;
use cmds::{
    append_blocks_cmd, blocks_cmd, children_blocks_cmd, databases_cmd, pages_cmd,
    property_pages_cmd, query_databases_cmd, search_cmd,
};
use subcommand::{NotionSubCommand, Nurl};

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

    <NotionSubCommand as Nurl>::print_curl(&arg_matches, notion_api_key, notion_version);
}
