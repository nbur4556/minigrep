use std::error::Error;
use std::process;

pub mod config;
mod filecontents;
mod search;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let file_contents = filecontents::get_contents_as_text(&config.file_path)?;

    let matched_lines: Vec<&str> = if config.ignore_case {
        search::case_insensitive(&config.query, &file_contents)
    } else {
        search::case_sensitive(&config.query, &file_contents)
    };

    for line in matched_lines {
        println!("{line}")
    }

    Ok(())
}

pub fn handle_error(error: &str) {
    eprintln!("{}", error);
    process::exit(1);
}
