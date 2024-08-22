use clap::{App, SubCommand};

use crate::{
    args::{file::File, notion_id::NotionIdArg, x::X},
    method::Method,
    // cmds::execute::handler
};

use super::i_cmd::ICommand;

pub struct Pages {
    pub method: Method,
    pub file_path: String,
    pub notion_id: NotionIdArg
}

impl ICommand for Pages {
    fn generate_url(&self) -> String {
        "https://api.notion.com/v1/pages".to_string()
    }
    fn generate_mthod(&self) -> String {
        format!("-X {}", &self.method.fmt())
    }
    fn get_file(&self) -> String {
        std::fs::read_to_string(&self.file_path).unwrap()
    }
    fn print_curl(&self, notion_api_key: String, notion_version: String) {
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

impl Pages {
    pub fn generate_url_with_id(&self) -> String {
        format!("https://api.notion.com/v1/pages/{}", &self.notion_id.get_id())
    }
}

pub fn pages_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("pages")
        .about("Output Notion API URLs for pages")
        .arg(X::x_option())
        .arg(NotionIdArg::id_option())
        .arg(File::file_option())
}