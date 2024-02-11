pub mod slicer;

pub struct Config {
    pub folder_path: String,
    pub file_path: String,
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

        Ok(Config {
            folder_path,
            file_path,
        })
    }
}
