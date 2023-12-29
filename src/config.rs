use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn parse_arguments(args: &[String]) -> Self {
        let query = match args.get(1) {
            Some(arg) => arg,
            None => "",
        };
        let file_path = match args.get(2) {
            Some(arg) => arg,
            None => "",
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Self {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        }
    }
}
