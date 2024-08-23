use clap::App;

use crate::{args::block_id::BlockIdArg, method::Method};

use super::i_cmd::ICommand;

pub struct ChildrenBlocks {
    pub method: Method,
    pub block_id: BlockIdArg,
}

impl ICommand for ChildrenBlocks {
    fn generate_url(&self) -> String {
        todo!()
    }

    fn generate_mthod(&self) -> String {
        todo!()
    }

    fn get_file(&self) -> String {
        todo!()
    }

    fn print_curl(&self, notion_api_key: String, notion_version: String) {
        todo!()
    }
}

pub fn children_blocks_subcommand() -> App<'static, 'static> {
    todo!()
}