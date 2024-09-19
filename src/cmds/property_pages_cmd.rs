use clap::Command;

use crate::{
    args::{notion_id::NotionIdArg, property_id::PropertyIdArg, x::X},
    // cmds::execute::handler,
    method::MethodArg
};


pub struct PropertyPages {
    pub method: MethodArg,
    pub notion_id: NotionIdArg,
    pub property_id: PropertyIdArg,
}
impl PropertyPages {
    pub fn generate_mthod(&self) -> String {
        format!("-X {}", &self.method.fmt())
    }
    pub fn generate_url(&self) -> String {
        format!("https://api.notion.com/v1/pages/{}/properties/{}", &self.notion_id.get_id(), &self.property_id.get_id())
    }
    pub fn print_curl(&self, notion_api_key: String, notion_version: String) {
        let curl = format!(
            "curl {} '{}' \\\n -H 'Authorization: Bearer {}' \\\n -H 'Notion-Version: {}'",
            &self.generate_mthod(),
            &self.generate_url(),
            notion_api_key,
            notion_version
        );
        println!("{}", curl);
        // handler(&curl);
    }
}

pub fn property_pages_subcommand() -> Command {
    Command::new("property_pages")
        .about("Output Notion API URLs for property pages")
        .arg(X::x_option())
        .arg(NotionIdArg::id_option())
        .arg(PropertyIdArg::id_option())
}
