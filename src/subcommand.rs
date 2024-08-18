use clap::ArgMatches;
use crate::method::Method;
use crate::pages_cmd::Pages;
use crate::databases_cmd::Databases;
use crate::query_databases_cmd::QueryDatabases;

pub enum NotionSubCommand {
    Pages(Pages),
    Databases(Databases),
    QueryDatabases(QueryDatabases),
}

// pagesもしくはdatabasesを受け取り、それぞれの構造体を返す
impl NotionSubCommand {
    pub fn from_args(matches: &ArgMatches) -> Self {
        if let Some(matches) = matches.subcommand_matches("databases") {
            let databases_opt = Databases {
                method: Method::match_method(matches.value_of("x").unwrap_or("GET")),
                id: matches.value_of("id").unwrap_or("").to_string(),
                file_path: matches.value_of("file").unwrap_or("").to_string(),
            };

            NotionSubCommand::Databases(databases_opt)
        } else if let Some(matches) = matches.subcommand_matches("qdatabases") {
            let query_databases_opt = QueryDatabases {
                id: matches.value_of("id").unwrap_or("").to_string(),
                file_path: matches.value_of("file").unwrap_or("").to_string(),
            };
    
            NotionSubCommand::QueryDatabases(query_databases_opt)
        } else if let Some(matches) = matches.subcommand_matches("pages") {
            let pages_opt = Pages {
                method: Method::match_method(matches.value_of("x").unwrap_or("GET")),
                file_path: matches.value_of("file").unwrap_or("").to_string(),
            };

            NotionSubCommand::Pages(pages_opt)
        } else {
            panic!("Error: subcommand is empty");
        }
    }
}
