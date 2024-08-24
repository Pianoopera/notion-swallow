use clap::ArgMatches;
use crate::args::block_id::BlockIdArg;
use crate::args::notion_id::NotionIdArg;
use crate::args::page_size::PageSizeArg;
use crate::args::property_id::PropertyIdArg;
use crate::cmds::append_blocks_cmd::BlocksAppend;
use crate::cmds::blocks_cmd::Blocks;
use crate::cmds::children_blocks_cmd::ChildrenBlocks;
use crate::cmds::property_pages_cmd::PropertyPages;
use crate::cmds::search_cmd::Search;
use crate::method::Method;
use crate::pages_cmd::Pages;
use crate::databases_cmd::Databases;
use crate::query_databases_cmd::QueryDatabases;

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
                method: Method::match_method(matches.value_of("x").unwrap_or("GET")),
                notion_id: NotionIdArg(matches.value_of("id").unwrap_or("").to_string()),
                file_path: matches.value_of("file").unwrap_or("").to_string(),
            };

            NotionSubCommand::Databases(databases_opt)
        } else if let Some(matches) = matches.subcommand_matches("query_databases") {
            let query_databases_opt = QueryDatabases {
                notion_id: NotionIdArg(matches.value_of("id").unwrap_or("").to_string()),
                file_path: matches.value_of("file").unwrap_or("").to_string(),
            };
    
            NotionSubCommand::QueryDatabases(query_databases_opt)
        } else if let Some(matches) = matches.subcommand_matches("pages") {
            let pages_opt = Pages {
                method: Method::match_method(matches.value_of("x").unwrap_or("GET")),
                file_path: matches.value_of("file").unwrap_or("").to_string(),
                notion_id: NotionIdArg(matches.value_of("id").unwrap_or("").to_string()),
            };

            NotionSubCommand::Pages(pages_opt)
        } else if let Some(matches) = matches.subcommand_matches("property_pages") {
            let property_pages_opt = PropertyPages {
                method: Method::match_method(matches.value_of("x").unwrap_or("GET")),
                notion_id: NotionIdArg(matches.value_of("id").unwrap_or("").to_string()),
                property_id: PropertyIdArg(matches.value_of("property_id").unwrap_or("").to_string()),
            };

            NotionSubCommand::PropertyPages(property_pages_opt)
        } else if let Some(matches) = matches.subcommand_matches("blocks") {
            let blocks_opt = Blocks {
                method: Method::match_method(matches.value_of("x").unwrap_or("GET")),
                block_id: BlockIdArg(matches.value_of("id").unwrap_or("").to_string()),
                file_path: matches.value_of("file").unwrap_or("").to_string(),
            };
            
            NotionSubCommand::Blocks(blocks_opt)
        } else if let Some(matches) = matches.subcommand_matches("append_blocks") {
            let append_blocks_opt = BlocksAppend {
                method: Method::match_method(matches.value_of("x").unwrap_or("GET")),
                block_id: BlockIdArg(matches.value_of("id").unwrap_or("").to_string()),
                file_path: matches.value_of("file").unwrap_or("").to_string(),
            };
            
            NotionSubCommand::BlocksAppend(append_blocks_opt)
        } else if let Some(matches) = matches.subcommand_matches("children_blocks") {
            let children_blocks_opt = ChildrenBlocks {
                method: Method::match_method(matches.value_of("x").unwrap_or("GET")),
                block_id: BlockIdArg(matches.value_of("id").unwrap_or("").to_string()),
                page_size: PageSizeArg(matches.value_of("page_size").unwrap_or("100").to_string()),
            };
            
            NotionSubCommand::ChildrenBlocks(children_blocks_opt)
        } else if let Some(matches) = matches.subcommand_matches("search") {
            let search_opt = Search {
                file_path: matches.value_of("file").unwrap_or("").to_string(),
            };
            
            NotionSubCommand::Search(search_opt)
        } else {
            panic!("Error: subcommand is empty");
        }
    }
}
