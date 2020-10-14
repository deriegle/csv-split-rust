use std::env;

pub struct Config {
    pub batch_size: i32,
    pub filename: String
}

const DEFAULT_BATCH_SIZE: i32 = 10000;

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // skip first arg which has name of file we're running
        args.next();

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't receive a filename.")
        };

        let batch_size = match args.next() {
            Some(size) => size.parse::<i32>(),
            None => Ok(DEFAULT_BATCH_SIZE)
        }.unwrap_or(DEFAULT_BATCH_SIZE);


        Ok(Config { batch_size, filename })
    }
}
