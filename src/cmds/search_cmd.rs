use clap::Command;

use crate::args::file::FileArg;

pub struct Search {
    pub file: FileArg,
}

impl Search {
    pub fn get_file(&self) -> String {
        std::fs::read_to_string(&self.file.file_path()).unwrap()
    }
    pub fn print(&self, notion_api_key: String, notion_version: String) {
        let curl = format!(
            "curl -X POST 'https://api.notion.com/v1/search' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}' \\\n -H 'Content-Type: application/json' \\\n -d '{}'",
            notion_api_key,
            notion_version,
            &self.get_file()
        );
        println!("{}", curl);
        // handler(&curl);
    }
}

pub fn search_subcommand() -> Command {
    Command::new("search")
        .about("Output Notion API URLs for query databases")
        .arg(FileArg::file_option())
}