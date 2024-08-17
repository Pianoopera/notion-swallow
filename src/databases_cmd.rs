use clap::{App, Arg, SubCommand};

pub struct DatabasesOpt {
    pub method: String,
    pub id: String,
    pub file_path: String,
}

impl DatabasesOpt {
    pub fn generate_url(&self) -> String {
        format!("https://api.notion.com/v1/databases/{}", &self.id)
    }
    pub fn generate_mthod(&self) -> String {
        format!("-L -X {}", &self.method)
    }
    pub fn get_file_path(&self) -> String {
        self.file_path.clone()
    }
    pub fn get_file(&self) -> String {
        std::fs::read_to_string(&self.file_path).unwrap()
    }
}

pub fn databases_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("databases")
        .about("Output Notion API URLs for databases")
        .arg(
            Arg::with_name("id")
                .short("i")
                .long("id")
                .value_name("ID")
                .help("Set the ID of Notion API")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("x")
                .short("x")
                .long("x")
                .value_name("METHOD")
                .help("Set the method of Notion API")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Set the file of Notion API")
                .takes_value(true)
                .required(false),
        )
}