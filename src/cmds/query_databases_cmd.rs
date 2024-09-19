use clap::Command;

use crate::args::{file::FileArg, notion_id::NotionIdArg};

pub struct QueryDatabases {
    pub notion_id: NotionIdArg,
    pub file: FileArg,
}

impl QueryDatabases {
    pub fn generate_url(&self) -> String {
        format!("https://api.notion.com/v1/databases/{}/query", &self.notion_id.get_id())
    }
    pub fn get_file_path(&self) -> String {
        self.file.file_path().clone()
    }
    pub fn get_file(&self) -> String {
        std::fs::read_to_string(&self.file.file_path()).unwrap()
    }
    pub fn print(&self, notion_api_key: String, notion_version: String) {
        if !self.get_file_path().is_empty() {
            let curl = format!(
                "curl -X POST '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}' \\\n -H 'Content-Type: application/json' \\\n -d '{}'",
                &self.generate_url(),
                notion_api_key,
                notion_version,
                &self.get_file()
            );
            println!("{}", curl);
            // handler(&curl);
        } else {
            // error pattern
            println!("Error: file path is empty");
        }
    }
}

pub fn query_databases_cmd() -> Command {
    Command::new("query_databases")
        .about("Output Notion API URLs for query databases")
        .arg(NotionIdArg::id_option())
        .arg(FileArg::file_option())
}