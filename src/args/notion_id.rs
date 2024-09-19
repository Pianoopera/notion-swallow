use clap::Arg;

pub struct NotionIdArg(pub String);

impl NotionIdArg {
    pub fn id_option() -> Arg {
        Arg::new("id")
            .short('i')
            .long("id")
            .value_name("ID")
            .help("Set the id of Notion API")
            .required(false)
    }
    pub fn get_id(&self) -> String {
        self.0.to_string()
    }
    pub fn match_arg(matches: &clap::ArgMatches) -> Self {
        NotionIdArg(
            matches
                .get_one::<String>("id")
                .map(String::as_str)
                .unwrap_or("")
                .to_string(),
        )
    }
}
