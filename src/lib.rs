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
    pub file_path: String,

    /// Test command to be used for slicing
    #[arg(long)]
    pub test_command: String,

    /// Test args given to the test command
    #[arg(long)]
    pub test_args: Option<Vec<String>>,
}

// impl Config {
//     pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
//         args.next();
//
//         let folder_path = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get folder path"),
//         };
//
//         let file_path = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get file path"),
//         };
//
//         let test_command = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get test command"),
//         };
//
//         let test_args = match args.next() {
//             Some(arg) => arg,
//             None => return Err("Didn't get test args"),
//         };
//
//
//         Ok(Config {
//             folder_path,
//             file_path,
//             test_command,
//             test_args,
//         })
//     }
// }
