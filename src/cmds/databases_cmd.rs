use clap::{App, SubCommand};

use crate::{
    args::{file::File, notion_id::NotionIdArg, x::X},
    method::Method,
    // cmds::execute::handler
};

pub struct Databases {
    pub method: Method,
    pub notion_id: NotionIdArg,
    pub file_path: String,
}

impl Databases {
    pub fn generate_url(&self) -> String {
        format!("https://api.notion.com/v1/databases/{}", &self.notion_id.get_id())
    }
    pub fn generate_mthod(&self) -> String {
        format!("-L -X {}", &self.method.fmt())
    }
    pub fn get_file(&self) -> String {
        std::fs::read_to_string(&self.file_path).unwrap()
    }
    pub fn print_curl(&self, notion_api_key: String, notion_version: String) {
        if &self.method == &Method::POST {
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
        // patch判定
        } else if &self.method == &Method::PATCH {
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
            let curl = format!(
                "curl {} '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}'",
                &self.generate_mthod(),
                &self.generate_url(),
                notion_api_key,
                notion_version
            );
            println!("{}", curl);
            // handler(&curl);
        }
    }
}

pub fn databases_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("databases")
        .about("Output Notion API URLs for databases")
        .arg(NotionIdArg::id_option())
        .arg(X::x_option())
        .arg(File::file_option())
}