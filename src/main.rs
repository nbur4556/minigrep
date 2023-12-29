use std::env;

use minigrep::config::Config;
use minigrep::{handle_error, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        handle_error("Please provide the arguments: search_query file_path")
    }

    let config = Config::parse_arguments(&args);
    run(config).unwrap_or_else(|err| handle_error(&err.to_string()));
}
