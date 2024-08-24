use clap::App;

use crate::{args::{block_id::BlockIdArg, x::X}, method::Method};

pub struct ChildrenBlocks {
    pub method: Method,
    pub block_id: BlockIdArg,
}

impl ChildrenBlocks {
    pub fn generate_url(&self) -> String {
        format!("https://api.notion.com/v1/blocks/{}", &self.block_id.get_id())
    }
    pub fn generate_mthod(&self) -> String {
        format!("-X {}", &self.method.fmt())
    }
    pub fn print_curl(&self, notion_api_key: String, notion_version: String) {
        let curl = format!(
            "curl {} '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}' \\\n -H 'Content-Type: application/json'",
            &self.generate_mthod(),
            &self.generate_url(),
            notion_api_key,
            notion_version,
        );
        println!("{}", curl);
    }
}

pub fn children_blocks_subcommand() -> App<'static, 'static> {
    clap::SubCommand::with_name("children_blocks")
        .about("Output Notion API URLs for children blocks")
        .arg(X::x_option())
        .arg(BlockIdArg::id_option())
}