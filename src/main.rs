use clap::{App, Arg};

fn main() {
    let matches = App::new("rf-notion")
        .version("0.1.0")
        .author("teto <https://github.com/Pianoopera>")
        .about("Output Notion API URLs")
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .help("Set the type of Notion API")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    println!("https://api.notion.com/v1/{}", matches.value_of("type").unwrap());
}
