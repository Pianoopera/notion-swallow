use clap::Arg;

pub struct PropertyIdArg(pub String);

impl PropertyIdArg {
    pub fn id_option() -> Arg {
        Arg::new("property_id")
            .short('p')
            .long("property_id")
            .value_name("PROPERTY_ID")
            .help("Set the property_id of Notion API")
            .required(false)
    }
    pub fn get_id(&self) -> String {
        self.0.to_string()
    }
}
