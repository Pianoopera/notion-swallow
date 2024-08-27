use crate::args::file::File;

pub struct Search {
    pub file_path: String,
}

impl Search {
    pub fn get_file(&self) -> String {
        std::fs::read_to_string(&self.file_path).unwrap()
    }
    pub fn print_curl(&self, notion_api_key: String, notion_version: String) {
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

pub fn search_subcommand() -> clap::Command<'static, 'static> {
    clap::Command::new()
}