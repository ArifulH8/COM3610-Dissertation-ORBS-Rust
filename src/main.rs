use std::env;
use std::process;

use orbs_slicer::slicer;
use orbs_slicer::Config;

fn main() {
    println!("Hello, observation based slicer!");

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Config folder path: {}", config.folder_path);
    println!("Config file path: {}", config.file_path);
    println!("Config test command: {}", config.test_command);
    println!("Config test args: {}", config.test_args);

    slicer::start(config);
}
