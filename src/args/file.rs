use clap::Arg;

pub struct File(pub String);

impl File {
    pub fn file_option() -> Arg {
        Arg::new("file")
                .long("file")
                .value_name("FILE")
                .help("Set the file of Notion API")
                .required(false)
    }
}