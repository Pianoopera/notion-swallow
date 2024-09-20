mod args;
mod cmds;
mod method;
mod subcommand;

use std::env;

use clap::{Command, Parser, Subcommand};
use cmds::{
    append_blocks_cmd, blocks_cmd, children_blocks_cmd, databases_cmd, pages_cmd,
    property_pages_cmd, query_databases_cmd, search_cmd,
};
use subcommand::{NotionSubCommand, Nurl};

#[derive(Parser)]
#[command(version, about, long_about = None, author)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Output Notion API URLs
    Pages {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    match &cli.command {
        Some(Commands::Pages { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }
    // let mut app = Command::new("notion-swallow")
    //     .version("0.3.6")
    //     .author("teto <https://github.com/Pianoopera>")
    //     .about("Output Notion API URLs")
    //     .subcommands(vec![
    //         databases_cmd::databases_subcommand(),
    //         query_databases_cmd::query_databases_cmd(),
    //         pages_cmd::pages_subcommand(),
    //         property_pages_cmd::property_pages_subcommand(),
    //         append_blocks_cmd::append_blocks_subcommand(),
    //         blocks_cmd::blocks_subcommand(),
    //         children_blocks_cmd::children_blocks_subcommand(),
    //         search_cmd::search_subcommand(),
    //     ]);
    // let arg_matches = app.clone().get_matches();

    // // サブコマンドが指定されていない場合はヘルプを表示
    // if arg_matches.subcommand_name().is_none() {
    //     app.print_help().unwrap();
    //     return;
    // }

    // let notion_api_key = env::var("NOTION_SECRET_KEY").unwrap_or("secret_123".to_string());
    // let notion_version = env::var("NOTION_VERSION").unwrap_or("2022-06-28".to_string());

    // <NotionSubCommand as Nurl>::print_curl(&arg_matches, notion_api_key, notion_version);
}
