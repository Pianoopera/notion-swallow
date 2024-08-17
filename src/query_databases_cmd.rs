use clap::{App, Arg, SubCommand};

pub struct QueryDatabases {
    pub id: String,
    pub file_path: String,
}

impl QueryDatabases {
    pub fn generate_url(&self) -> String {
        format!("https://api.notion.com/v1/databases/{}/query", &self.id)
    }
    pub fn get_file_path(&self) -> String {
        self.file_path.clone()
    }
    pub fn get_file(&self) -> String {
        std::fs::read_to_string(&self.file_path).unwrap()
    }
    pub fn print_curl(&self, notion_api_key: String, notion_version: String) {
        if !self.get_file_path().is_empty() {
            println!(
                "curl -X POST '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}' \\\n -H 'Content-Type: application/json' \\\n -d '{}'",
                &self.generate_url(),
                notion_api_key,
                notion_version,
                &self.get_file()
            );
        } else {
            // error pattern
            println!("Error: file path is empty");
        }
    }
}

pub fn query_databases_cmd() -> App<'static, 'static> {
    SubCommand::with_name("qdatabases")
        .about("Output Notion API URLs for query databases")
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
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Set the file of Notion API")
                .takes_value(true)
                .required(false),
        )
}