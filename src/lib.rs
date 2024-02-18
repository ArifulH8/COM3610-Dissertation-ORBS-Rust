pub mod slicer;

pub struct Config {
    pub folder_path: String,
    pub file_path: String,
    pub test_command: String,
    pub test_args: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let folder_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get folder path"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get file path"),
        };

        let test_command = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get test command"),
        };

        let test_args = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get test args"),
        };


        Ok(Config {
            folder_path,
            file_path,
            test_command,
            test_args,
        })
    }
}
