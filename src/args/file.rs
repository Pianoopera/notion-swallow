use clap::{Arg, ArgMatches};

#[derive(Clone)]
pub struct FileArg(pub String);

impl FileArg {
    pub fn file_option() -> Arg {
        Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("Set the file of Notion API")
                .required(false)
    }
    pub fn match_arg(matches: &ArgMatches) -> Self {
        FileArg(
            matches
                .get_one::<String>("file")
                .map(String::as_str)
                .unwrap_or("")
                .to_string()
        )
    }
    pub fn file_path(&self) -> String {
        self.0.to_string()
    }
}