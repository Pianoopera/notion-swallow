use clap::Arg;

pub struct X(pub String);

impl X {
    pub fn x_option() -> Arg<'static, 'static> {
        Arg::with_name("x")
                .short("x")
                .long("x")
                .value_name("METHOD")
                .help("Set the method of Notion API")
                .takes_value(true)
                .required(false)
    }
}