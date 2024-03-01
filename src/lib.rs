use clap::Parser;

pub mod slicer;

#[derive(Parser, Debug)]
#[command(version, about, long_about)]
pub struct Config {
    /// Folder path of the project to be sliced
    #[arg(long)]
    pub folder_path: String,

    /// File path of the file to be sliced
    #[arg(long)]
    pub file_path: Option<String>,

    /// Test command to be used for slicing
    #[arg(long)]
    pub test_command: String,

    /// Test args given to the test command
    #[arg(long)]
    pub test_args: Option<Vec<String>>,
}
