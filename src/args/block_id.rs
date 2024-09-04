use clap::Arg;

pub struct BlockIdArg(pub String);

impl BlockIdArg {
    pub fn id_option() -> Arg {
        Arg::new("id")
            .long("id")
            .value_name("ID")
            .help("Set the id of Notion API")
            .required(false)
    }
    pub fn get_id(&self) -> String {
        self.0.to_string()
    }
}
