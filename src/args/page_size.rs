use clap::Arg;

pub struct PageSizeArg(pub String);

impl PageSizeArg {
    pub fn page_size_option() -> Arg<'static, 'static> {
        Arg::with_name("page_size")
            .short("ps")
            .long("page_size")
            .value_name("PAGE_SIZE")
            .help("Set the page size of Notion API")
            .takes_value(true)
            .required(false)
    }
    pub fn get_page_size(&self) -> String {
        self.0.to_string()
    }
}
