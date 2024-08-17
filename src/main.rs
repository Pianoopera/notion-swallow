use clap::App;
mod databases_cmd;

fn main() {
    let matches = App::new("rf-notion")
        .version("0.1.0")
        .author("teto <https://github.com/Pianoopera>")
        .about("Output Notion API URLs")
        .subcommand(
            databases_cmd::databases_subcommand()
        )
        .get_matches();

    let notion_api_key = dotenv::var("NOTION_API_KEY").unwrap_or("default".to_string());
    let notion_version = dotenv::var("NOTION_VERSION").unwrap_or("2021-05-13".to_string());

    if let Some(matches) = matches.subcommand_matches("databases") {
        let databases_opt = databases_cmd::Databases {
            method: matches.value_of("x").unwrap_or("GET").to_string(),
            id: matches.value_of("id").unwrap_or("").to_string(),
            file_path: matches.value_of("file").unwrap_or("").to_string(),
        };

        databases_opt.print_curl(notion_api_key, notion_version);
    }
}
