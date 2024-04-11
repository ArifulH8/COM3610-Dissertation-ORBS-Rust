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

    /// Names of files to skip when slicing multiple files
    #[arg(long)]
    pub skip_files: Option<Vec<String>>,

    /// Number of deletion windows (>0)
    #[arg(short, long, default_value_t = 3, value_parser = del_window_in_range)]
    pub del_win: usize,

    /// Set true when you need detailed output
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}

fn del_window_in_range(s: &str) -> Result<usize, String> {
    let del_win: usize = s.parse().map_err(|_| format!("`{s}` isn't a deletion window number"))?;
    if del_win > 0 {
        Ok(del_win)
    } else {
        Err(format!("Deletion window ({}) is less than 0", {del_win}))
    }
}
