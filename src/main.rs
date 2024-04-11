use clap::Parser;

use orbs_slicer::slicer;
use orbs_slicer::Config;

fn main() {
    let config = Config::parse();

    println!("Hello, observation based slicer!");

    println!("Config folder path: {}", config.folder_path);
    println!("Config file path: {:?}", config.file_path);
    println!("Config test command: {}", config.test_command);
    println!("Config test args: {:?}", config.test_args);

    slicer::start(config);
}
