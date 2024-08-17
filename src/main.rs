use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("rf-notion")
        .version("0.1.0")
        .author("teto <https://github.com/Pianoopera>")
        .about("Output Notion API URLs")
        .subcommand(
            SubCommand::with_name("databases")
                .about("Output Notion API URLs for databases")
                .arg(
                    Arg::with_name("id")
                        .short("i")
                        .long("id")
                        .value_name("ID")
                        .help("Set the ID of Notion API")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("x")
                        .short("x")
                        .long("x")
                        .value_name("METHOD")
                        .help("Set the method of Notion API")
                        .takes_value(true)
                        .required(false),
                )
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .long("file")
                        .value_name("FILE")
                        .help("Set the file of Notion API")
                        .takes_value(true)
                        .required(false),
                )
        )
        .get_matches();

    let notion_api_key = dotenv::var("NOTION_API_KEY").unwrap_or("default".to_string());
    let notion_version = dotenv::var("NOTION_VERSION").unwrap_or("2021-05-13".to_string());

    if let Some(matches) = matches.subcommand_matches("databases") {
        let _method = matches.value_of("x").unwrap_or("GET");
        let _id = matches.value_of("id").unwrap_or("");
        let _file = matches.value_of("file").unwrap_or("");

        let method = format!("-L -X {}", _method);
        let url = format!("https://api.notion.com/v1/databases/{}", _id);
        let file = _file;

        if !file.is_empty() {
            let file = std::fs::read_to_string(file).unwrap();
            println!(
                "curl {} '{}' \\ \n -H 'Authorization: Bearer {}' \\ \n -H 'Notion-Version: {}' \\ \n -H 'Content-Type: application/json' \\ \n -d '{}'",
                method,
                url,
                notion_api_key,
                notion_version,
                file
            );
        } else {
            println!(
                "curl {} '{}' \\ \n -H 'Authorization: Bearer {}' \\ \n -H 'Notion-Version: {}'",
                method,
                url,
                notion_api_key,
                notion_version
            );
        }
    }
}
