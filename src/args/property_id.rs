use clap::Arg;

pub struct PropertyIdArg(pub String);

impl PropertyIdArg {
    pub fn id_option() -> Arg<'static, 'static> {
        Arg::with_name("property_id")
            .short("p")
            .long("property_id")
            .value_name("PROPERTY_ID")
            .help("Set the property_id of Notion API")
            .takes_value(true)
            .required(false)
    }
    pub fn get_id(&self) -> String {
        self.0.to_string()
    }
}
