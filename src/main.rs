use std::fs::File;
use std::io::{self,BufRead};
use std::env;
use std::process;

const BATCH_SIZE: i32 = 10_000;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let mut header: std::option::Option<String> = None;
    let file = File::open(config.filename)?;
    let reader = io::BufReader::new(file);
    let mut batches: Vec<Vec<String>> = Vec::new();
    let mut current_line: i32 = 0;

    let start = std::time::Instant::now();

    for line in reader.lines() {
        current_line += 1;

        let contents = line.expect("Something went wrong while parsing the file");

        if header.is_none() {
            header = Some(contents);
        } else {
            let batch_number = (current_line / BATCH_SIZE) as usize;

            match batches.get_mut(batch_number) {
                None => {
                    let mut new_batch = Vec::new();
                    new_batch.push(contents);
                    batches.insert(batch_number, new_batch);
                },
                Some(batch) => {
                    batch.push(contents);
                }
            }
        }
    }

    let elapsed = start.elapsed();

    match header {
        None => println!("File not found."),
        Some(header_text) => {
            println!("Header Text: {}", header_text);
        }
    }

    println!("{:.2?} for processing {} batches", elapsed, batches.len());

    Ok(())
}

struct Config {
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}
