use clap::Arg;

pub struct File(pub String);

impl File {
    pub fn file_option() -> Arg<'static, 'static> {
        Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Set the file of Notion API")
                .takes_value(true)
                .required(false)
    }
}