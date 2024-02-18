use std::error::Error;
use std::fs;
use std::process::Command;

use crate::Config;

pub fn start(config: Config) {
    println!("Slicer Start");

    let contents = match read_file(&config.file_path) {
        Ok(contents) => contents,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut string_lines = file_lines(contents);
    string_lines.reverse();

    string_lines = slice(&config, string_lines);
    string_lines.reverse();

    write_file(&config.file_path, string_lines).unwrap();
}

fn read_file(file_path: &String) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)
}

fn write_file(file_path: &String, file_vec: Vec<String>) -> Result<(), Box<dyn Error>> {
    let joined_contents = file_vec.join("\n");
    fs::write(file_path, joined_contents)?;

    Ok(())
}

fn file_lines(contents: String) -> Vec<String> {
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    lines
}

fn slice(config: &Config, mut file_vec: Vec<String>) -> Vec<String> {
    // let vec_lenth = file_vec.len();
    let mut counter = 0;
    while counter < file_vec.len() {
        println!("{}", counter);
        let file_vec_clone = file_vec.clone();

        let result = delete(&config, file_vec_clone, counter);

        if result {
            file_vec.remove(counter);
        }

        counter += 1;
    }

    file_vec
}

fn delete(config: &Config, file_vec: Vec<String>, counter: usize) -> bool {
    // let mut _bestDW = 0
    // let mut succeed = false;

    let mut file_vec_clone = file_vec.clone();
    file_vec_clone.remove(counter);
    file_vec_clone.reverse();
    write_file(&config.file_path, file_vec_clone).unwrap();

    // let binding = fs::canonicalize(&config.folder_path.clone()).unwrap();
    // let path = binding.to_str().unwrap();
    let path = &config.folder_path;

    let status= if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", &config.test_command, &config.test_args])
            .current_dir(path)
            .status()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .args(["-c", &config.test_command, &config.test_args])
            .current_dir(path)
            .status()
            .expect("failed to execute process")
    };

    status.success()
}
