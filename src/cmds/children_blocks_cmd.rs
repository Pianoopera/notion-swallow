use clap::Command;

use crate::{args::{block_id::BlockIdArg, page_size::PageSizeArg, x::X}, method::MethodArg};

pub struct ChildrenBlocks {
    pub method: MethodArg,
    pub block_id: BlockIdArg,
    pub page_size: PageSizeArg,
}

impl ChildrenBlocks {
    pub fn generate_url(&self) -> String {
        format!(
            "https://api.notion.com/v1/blocks/{}/children?page_size={}",
            &self.block_id.get_id(),
            &self.page_size.get_page_size(),
        )
    }
    pub fn generate_mthod(&self) -> String {
        format!("-X {}", &self.method.fmt())
    }
    pub fn print_curl(&self, notion_api_key: String, notion_version: String) {
        let curl = format!(
            "curl {} '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}'",
            &self.generate_mthod(),
            &self.generate_url(),
            notion_api_key,
            notion_version,
        );
        println!("{}", curl);
    }
}

pub fn children_blocks_subcommand() -> Command {
    Command::new("children_blocks")
        .about("Output Notion API URLs for children blocks")
        .arg(X::x_option())
        .arg(BlockIdArg::id_option())
        .arg(PageSizeArg::page_size_option())
}