use clap::Command;

use crate::{
    args::{file::FileArg, notion_id::NotionIdArg, x::X},
    method::MethodArg,
};

use super::i_cmd::ICommand;

pub struct Databases {
    pub method: MethodArg,
    pub notion_id: NotionIdArg,
    pub file: FileArg,
}

impl ICommand for Databases {
    fn generate_url(&self) -> String {
        format!("https://api.notion.com/v1/databases/{}", &self.notion_id.get_id())
    }
    fn generate_mthod(&self) -> String {
        format!("-L -X {}", &self.method.fmt())
    }
    fn get_file(&self) -> String {
        std::fs::read_to_string(&self.file.file_path()).unwrap()
    }
    fn print_curl(&self, notion_api_key: String, notion_version: String) {
        if &self.method == &MethodArg::POST {
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
        } else if &self.method == &MethodArg::PATCH {
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

pub fn databases_subcommand() -> Command {
    Command::new("databases")
        .about("Output Notion API URLs for databases")
        .arg(NotionIdArg::id_option())
        .arg(X::x_option())
        .arg(FileArg::file_option())
}