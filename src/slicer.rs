use std::error::Error;
use std::fs;
use std::process::Command;
use std::time::{Instant};

use walkdir::{DirEntry, WalkDir};


use crate::Config;

pub fn start(config: Config) {
    println!("Slicer Start");
    let current_time = Instant::now();

    match &config.file_path {
        Some(file_path) => match one_file(&config, file_path) {
            Ok(_) => {
                println!("Success slicing {}", &file_path)
            }
            Err(e) => {
                println!("Failure slicing due to {} -> Skipping {}", e, &file_path)
            }
        },
        None => multiple_files(&config),
    }

    let elapsed_time = current_time.elapsed();
    println!("Running slicer start took {} seconds.", elapsed_time.as_secs());
}

fn one_file(config: &Config, file_path: &String) -> Result<(), Box<dyn Error>> {
    println!("Slicing File {}", file_path);

    let contents = read_file(file_path)
        .unwrap_or_else(|error| format!("Problem opening the file: {:?}", error));

    let mut string_lines = file_lines(contents);
    string_lines.reverse();

    string_lines = slice(&config, file_path, string_lines);
    string_lines.reverse();

    write_file(file_path, string_lines).unwrap();

    Ok(())
}

fn multiple_files(config: &Config) {

    let mut names: Vec<String> = Vec::new();

    for e in WalkDir::new(&config.folder_path).into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() && !is_skipped(config, &e) {
            names.push(e.path().to_str().unwrap().to_string())
        }
    };

    for name in names {
        let full_path = &name;

        match one_file(&config, &full_path) {
            Ok(_) => {
                println!("Success slicing {}", &full_path)
            }
            Err(e) => {
                println!("Failure slicing due to {} -> Skipping {}", e, &name)
            }
        }
    }
}

fn is_skipped(config: &Config, entry: &DirEntry) -> bool {
    match &config.skip_files {
        Some(skip_file_vec) => {
            entry.path()
                .to_str()
                .map(|s| skip_file_vec.iter().any(|i| {
                    s.contains(i)
                }))
                .unwrap_or(false)

        }
        _ => {false}
    }
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

fn slice(config: &Config, file_path: &String, mut file_vec: Vec<String>) -> Vec<String> {
    let mut counter = 0;
    while counter < file_vec.len() {
        let file_vec_clone = file_vec.clone();
        println!("----------------------");
        println!("Current line {}", file_vec.len() - counter);
        let (result, best_dw) = delete(&config, file_path, file_vec_clone, counter);
        if result {
            let best_dw = counter + best_dw + 1;
            println!("Best dw {}", best_dw);
            file_vec.drain(counter..best_dw);
        } else {
            counter += 1;
        }
    }

    file_vec
}

fn delete(config: &Config, file_path: &String, file_vec: Vec<String>, counter: usize) -> (bool, usize) {
    let mut best_dw= 0;
    let mut succeed = false;

    for dw in 0..3 {
        println!("Current dw {}", dw);
        let mut file_vec_clone = file_vec.clone();
        let current_dw = counter + dw + 1;
        println!("Counter:{}, dw:{}", counter, dw);
        if current_dw > file_vec_clone.len() {
            continue;
        }
        file_vec_clone.drain(counter..current_dw);
        file_vec_clone.reverse();
        write_file(file_path, file_vec_clone).unwrap();
        match test_run(config) {
            Ok(true) => {
                println!("Success");
                best_dw = dw;
                succeed = true;
            }
            Ok(_) => println!("Failure"),
            Err(e) => println!("Failure due to error: {}", e),
        }
    }

    (succeed, best_dw)
}

fn test_run(config: &Config) -> Result<bool, Box<dyn Error>> {
    // let binding = fs::canonicalize(&config.folder_path.clone()).unwrap();
    // let path = binding.to_str().unwrap();
    let path = &config.folder_path;

    let output = if cfg!(target_os = "windows") {
        let test_command_args = match &config.test_args.clone() {
            None => ["/C", &config.test_command].map(String::from).to_vec(),
            Some(value) => ["/C", &config.test_command, &value.join(" ")]
                .map(String::from)
                .to_vec(),
        };

        Command::new("cmd")
            .args(test_command_args)
            .current_dir(path)
            .output()
            .expect("failed to execute process")
    } else {
        let test_command_args = match &config.test_args.clone() {
            None => ["-c", &config.test_command].map(String::from).to_vec(),
            Some(value) => ["-c", &config.test_command, &value.join(" ")]
                .map(String::from)
                .to_vec(),
        };

        Command::new("sh")
            .args(test_command_args)
            .current_dir(path)
            .output()
            .expect("failed to execute process")
    };

    Ok(output.status.success())
}
