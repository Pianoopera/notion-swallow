use clap::{App, Arg};

fn main() {
    let matches = App::new("rf-notion")
        .version("0.1.0")
        .author("teto <https://github.com/Pianoopera>")
        .about("Output Notion API URLs")
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .help("Set the type of Notion API")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("id")
                .short("i")
                .long("id")
                .value_name("ID")
                .help("Set the ID of Notion API")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    let _type = matches.value_of("type").unwrap();
    let _id = matches.value_of("id").unwrap_or("");

    // id が指定されている場合 
    let url = format!("https://api.notion.com/v1/{}/{}", _type, _id);
    let notion_api_key = dotenv::var("NOTION_API_KEY").unwrap_or("default".to_string());
    let notion_version = dotenv::var("NOTION_VERSION").unwrap_or("2021-05-13".to_string());

    println!(
        "curl '{}' \\ \n-H 'Authorization: Bearer {}' \\ \n-H 'Notion-Version: {}'",
        url,
        notion_api_key,
        notion_version
    );
}
