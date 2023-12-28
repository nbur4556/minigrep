use std::env;
use std::process;

mod filecontents;

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn parse_arguments(args: &[String]) -> Self {
        let query = match args.get(1) {
            Some(arg) => arg,
            None => "",
        };
        let file_path = match args.get(2) {
            Some(arg) => arg,
            None => "",
        };

        Self {
            query: query.to_string(),
            file_path: file_path.to_string(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Please provide the arguments; search_query file_path");
        process::exit(1);
    }

    let config = Config::parse_arguments(&args);
    let file_contents =
        filecontents::get_contents_as_text(&config.file_path).unwrap_or_else(|err| {
            println!("{}", err.to_string());
            process::exit(1);
        });

    println!("{}", file_contents.to_string());
    dbg!(config.query);
}
