use std::error::Error;
use std::fs;

use crate::Config;

pub fn start(config: Config) {
    println!("Slicer Start");

    let contents = match read_file(config.file_path) {
        Ok(contents) => contents,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn read_file(file_path: String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}
