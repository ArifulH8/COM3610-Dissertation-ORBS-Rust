use std::error::Error;
use std::fs;

use crate::Config;

pub fn start(config: Config) {
    println!("Slicer Start");

    let contents = match read_file(config.file_path) {
        Ok(contents) => contents,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut string_lines = file_lines(contents);
    string_lines.reverse();
    println!("{:?}", string_lines);
}

fn read_file(file_path: String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn file_lines(contents: String) -> Vec<String> {
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    lines
}
