use std::fs::File;
use std::io::{self,BufRead,Lines};
use std::error::Error;

const BATCH_SIZE: i32 = 10_000;

pub struct Config {
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.filename)?;
    let reader = io::BufReader::new(file);

    let start = std::time::Instant::now();

    let batches = process_lines(reader.lines()).unwrap();

    let elapsed = start.elapsed();

    println!(
        "Took {:.5} seconds to process {} batches.",
        elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9,
        batches.len()
    );

    Ok(())
}

fn process_lines(lines: Lines<io::BufReader<File>>) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut current_line: i32 = 0;
    let mut header: std::option::Option<String> = None;
    let mut batches: Vec<Vec<String>> = Vec::new();

    for line in lines {
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

    match header {
        None => println!("File not found."),
        Some(header_text) => {
            println!("Header Text: {}", header_text);
        }
    }

    return Ok(batches);
}
