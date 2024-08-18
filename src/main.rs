use clap::App;

mod method;

mod subcommand;
mod databases_cmd;
mod query_databases_cmd;
mod pages_cmd;

fn main() {
    let matches = App::new("rf-notion")
        .version("0.1.0")
        .author("teto <https://github.com/Pianoopera>")
        .about("Output Notion API URLs")
        .subcommands(
            vec![
                databases_cmd::databases_subcommand(),
                query_databases_cmd::query_databases_cmd(),
                pages_cmd::pages_subcommand(),
            ]
        )
        .get_matches();

    let notion_api_key = dotenv::var("NOTION_API_KEY").unwrap_or("default".to_string());
    let notion_version = dotenv::var("NOTION_VERSION").unwrap_or("2021-05-13".to_string());

    let subcommand = subcommand::NotionSubCommand::from_args(&matches);
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
    }
}
