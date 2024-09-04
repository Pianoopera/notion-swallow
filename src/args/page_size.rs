use clap::Arg;

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
}
