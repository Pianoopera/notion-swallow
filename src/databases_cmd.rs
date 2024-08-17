use clap::{App, Arg, SubCommand};

pub struct Databases {
    pub method: String,
    pub id: String,
    pub file_path: String,
}

impl Databases {
    pub fn generate_url(&self) -> String {
        format!("https://api.notion.com/v1/databases/{}", &self.id)
    }
    pub fn generate_mthod(&self) -> String {
        format!("-L -X {}", &self.method)
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
                "curl {} '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}' \\\n -H 'Content-Type: application/json' \\\n -d '{}'",
                &self.generate_mthod(),
                &self.generate_url(),
                notion_api_key,
                notion_version,
                &self.get_file()
            );
        } else {
            println!(
                "curl {} '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}'",
                &self.generate_mthod(),
                &self.generate_url(),
                notion_api_key,
                notion_version
            );
        }
    }
}

pub fn databases_subcommand() -> App<'static, 'static> {
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
}