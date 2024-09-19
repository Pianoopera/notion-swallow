use clap::{Arg, ArgMatches};

pub struct PageSizeArg(pub String);

impl PageSizeArg {
    pub fn page_size_option() -> Arg {
        Arg::new("page_size")
            .short('p')
            .long("page_size")
            .value_name("PAGE_SIZE")
            .help("Set the page size of Notion API")
            .required(false)
    }
    pub fn get_page_size(&self) -> String {
        self.0.to_string()
    }
    pub fn match_arg(matches: &ArgMatches) -> Self {
        PageSizeArg(
            matches
                .get_one::<String>("page_size")
                .map(String::as_str)
                .unwrap_or("100")
                .to_string(),
        )
    }
}
