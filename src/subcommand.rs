use crate::args::block_id::BlockIdArg;
use crate::args::file::FileArg;
use crate::args::notion_id::NotionIdArg;
use crate::args::page_size::PageSizeArg;
use crate::args::property_id::PropertyIdArg;
use crate::cmds::append_blocks_cmd::BlocksAppend;
use crate::cmds::blocks_cmd::Blocks;
use crate::cmds::children_blocks_cmd::ChildrenBlocks;
use crate::cmds::property_pages_cmd::PropertyPages;
use crate::cmds::search_cmd::Search;
use crate::databases_cmd::Databases;
use crate::method::MethodArg;
use crate::pages_cmd::Pages;
use crate::query_databases_cmd::QueryDatabases;
use clap::ArgMatches;

pub enum NotionSubCommand {
    Pages(Pages),
    Databases(Databases),
    QueryDatabases(QueryDatabases),
    PropertyPages(PropertyPages),
    BlocksAppend(BlocksAppend),
    Blocks(Blocks),
    ChildrenBlocks(ChildrenBlocks),
    Search(Search),
}

// pagesもしくはdatabasesを受け取り、それぞれの構造体を返す
impl NotionSubCommand {
    pub fn from_args(matches: &ArgMatches) -> Self {
        if let Some(matches) = matches.subcommand_matches("databases") {
            let databases_opt = Databases {
                method: MethodArg::match_method(matches),
                notion_id: NotionIdArg::match_arg(matches),
                file: FileArg::match_arg(matches)
            };

            NotionSubCommand::Databases(databases_opt)
        } else if let Some(matches) = matches.subcommand_matches("query_databases") {
            let query_databases_opt = QueryDatabases {
                notion_id: NotionIdArg::match_arg(matches),
                file: FileArg::match_arg(matches)
            };

            NotionSubCommand::QueryDatabases(query_databases_opt)
        } else if let Some(matches) = matches.subcommand_matches("pages") {
            let pages_opt = Pages {
                method: MethodArg::match_method(matches),
                file: FileArg::match_arg(matches),
                notion_id: NotionIdArg::match_arg(matches),
            };

            NotionSubCommand::Pages(pages_opt)
        } else if let Some(matches) = matches.subcommand_matches("property_pages") {
            let property_pages_opt = PropertyPages {
                method: MethodArg::match_method(matches),
                notion_id: NotionIdArg::match_arg(matches),
                property_id: PropertyIdArg::match_arg(matches),
            };

            NotionSubCommand::PropertyPages(property_pages_opt)
        } else if let Some(matches) = matches.subcommand_matches("blocks") {
            let blocks_opt = Blocks {
                method: MethodArg::match_method(matches),
                block_id: BlockIdArg::match_arg(matches),
                file: FileArg::match_arg(matches)
            };

            NotionSubCommand::Blocks(blocks_opt)
        } else if let Some(matches) = matches.subcommand_matches("append_blocks") {
            let append_blocks_opt = BlocksAppend {
                method: MethodArg::match_method(matches),
                block_id: BlockIdArg::match_arg(matches),
                file: FileArg::match_arg(matches)
            };

            NotionSubCommand::BlocksAppend(append_blocks_opt)
        } else if let Some(matches) = matches.subcommand_matches("children_blocks") {
            let children_blocks_opt = ChildrenBlocks {
                method: MethodArg::match_method(matches),
                block_id: BlockIdArg::match_arg(matches),
                page_size: PageSizeArg::match_arg(matches),
            };

            NotionSubCommand::ChildrenBlocks(children_blocks_opt)
        } else if let Some(matches) = matches.subcommand_matches("search") {
            let search_opt = Search {
                file: FileArg::match_arg(matches)
            };

            NotionSubCommand::Search(search_opt)
        } else {
            panic!("Subcommand not found");
        }
    }
}
