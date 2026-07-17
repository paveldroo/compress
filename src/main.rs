use std::error::Error;

mod args;
mod reader;

fn main() {
    match run() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("compress: {err}");
            std::process::exit(1)
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = args::parse_filepath()?;
    let text_data = reader::get_text_data(path)?;

    Ok(())
}
