use std::env;

mod filecontents;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    let file_contents = filecontents::get_contents_as_text(file_path);

    dbg!(query);
    dbg!(file_path);
    dbg!(args);
    dbg!(file_contents);
}
