mod args;
mod reader;

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
    let _text_data = reader::get_text_data(path)?;

    Ok(())
}
