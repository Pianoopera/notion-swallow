use crate::{args::{block_id::BlockIdArg, file::File, x::X}, method::Method};

use super::i_cmd::ICommand;

pub struct Blocks {
    pub method: Method,
    pub block_id: BlockIdArg,
    pub file_path: String,
}

impl ICommand for Blocks {
    fn generate_url(&self) -> String {
        format!("https://api.notion.com/v1/blocks/{}", &self.block_id.get_id())
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
                &self.generate_url(),
                notion_api_key,
                notion_version,
                &self.get_file()
            );
            println!("{}", curl);
            // handler(&curl);
        } else if &self.method == &Method::DELETE {
            let curl = format!(
                "curl {} '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}' \\\n -H 'Content-Type: application/json'",
                &self.generate_mthod(),
                &self.generate_url(),
                notion_api_key,
                notion_version,
            );
            println!("{}", curl);
        } else {
            // error
            println!("Error: Invalid method");
        }
    }
}

pub fn blocks_subcommand() -> clap::App<'static, 'static> {
    clap::SubCommand::with_name("blocks")
        .about("Output Notion API URLs for blocks")
        .arg(X::x_option())
        .arg(BlockIdArg::id_option())
        .arg(File::file_option())
}