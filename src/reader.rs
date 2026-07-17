use std::{fs, io::Error};

pub fn get_text_data(path: String) -> Result<String, Error> {
    fs::read_to_string(path)
}
