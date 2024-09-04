use clap::Arg;

pub struct X(pub String);

impl X {
    pub fn x_option() -> Arg {
        Arg::new("x")
            .long("x")
            .value_name("METHOD")
            .help("Set the method of Notion API")
            .required(false)
    }
}