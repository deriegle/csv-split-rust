use std::env;

pub struct Config {
    pub filename: String
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // skip first arg which has name of file we're running
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive a filename.")
        };

        Ok(Config { filename })
    }
}
