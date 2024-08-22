use clap::{App, Arg, SubCommand};

use crate::{
    args::notion_id::NotionIdArg,
    method::Method,
    // cmds::execute::handler
};

pub struct Pages {
    pub method: Method,
    pub file_path: String,
    pub notion_id: NotionIdArg
}

impl Pages {
    pub fn generate_url(&self) -> String {
        "https://api.notion.com/v1/pages".to_string()
    }
    pub fn generate_mthod(&self) -> String {
        format!("-X {}", &self.method.fmt())
    }
    pub fn generate_url_with_id(&self) -> String {
        format!("https://api.notion.com/v1/pages/{}", &self.notion_id.get_id())
    }
    pub fn get_file(&self) -> String {
        std::fs::read_to_string(&self.file_path).unwrap()
    }
    pub fn print_curl(&self, notion_api_key: String, notion_version: String) {
        if &self.method == &Method::PATCH {
            let curl = format!(
                "curl {} '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}' \\\n -H 'Content-Type: application/json' \\\n -d '{}'",
                &self.generate_mthod(),
                &self.generate_url_with_id(),
                notion_api_key,
                notion_version,
                &self.get_file()
            );
            println!("{}", curl);
            // handler(&curl);
        } else if &self.method == &Method::GET {
            let curl =  format!(
                "curl {} '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}'",
                &self.generate_mthod(),
                &self.generate_url_with_id(),
                notion_api_key,
                notion_version
            );
            println!("{}", curl);
            // handler(&curl);
        } else if &self.method == &Method::POST {
            let curl = format!(
                "curl {} '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}' \\\n -H 'Content-Type: application/json' \\\n -d '{}'",
                &self.generate_mthod(),
                &self.generate_url(),
                notion_api_key,
                notion_version,
                &self.get_file()
            );
            println!("{}", curl);
            // handler(&curl);
        } else {
            // throw error
            println!("Not supported method");
        }
    }
}

pub fn pages_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("pages")
        .about("Output Notion API URLs for pages")
        .arg(
            Arg::with_name("x")
                .short("x")
                .long("x")
                .value_name("METHOD")
                .help("Set the method of Notion API")
                .takes_value(true)
                .required(false),
        )
        .arg(NotionIdArg::id_option())
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