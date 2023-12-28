use std::error::Error;
use std::process;

pub mod config;
mod filecontents;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let file_contents = filecontents::get_contents_as_text(&config.file_path)?;
    println!("{}", file_contents.to_string());
    Ok(())
}

pub fn handle_error(error: &str) {
    println!("{}", error);
    process::exit(1);
}
