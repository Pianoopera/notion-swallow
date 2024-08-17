use clap::{App, Arg, SubCommand};

pub fn databases_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("databases")
        .about("Output Notion API URLs for databases")
        .arg(
            Arg::with_name("id")
                .short("i")
                .long("id")
                .value_name("ID")
                .help("Set the ID of Notion API")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("x")
                .short("x")
                .long("x")
                .value_name("METHOD")
                .help("Set the method of Notion API")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("Set the file of Notion API")
                .takes_value(true)
                .required(false),
        )
}