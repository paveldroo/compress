mod args;
mod counter;
mod reader;
mod stack;
mod tree;

use std::error::Error;

fn main() {
    match run() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("compress: {err}");
            std::process::exit(1)
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = args::parse_filepath()?;
    let text_data = reader::get_text_data(path)?;
    let chars_map = counter::get_chars_map(text_data.as_str());
    let huffman_tree = tree::get_tree(chars_map);

    Ok(())
}
