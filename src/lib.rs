pub mod config;

use std::fs::File;
use std::io::{self,BufRead,Lines};
use std::error::Error;

const BATCH_SIZE: i32 = 10_000;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
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

fn process_lines(mut lines: Lines<io::BufReader<File>>) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut current_line: i32 = 0;
    let mut batches: Vec<Vec<String>> = Vec::new();

    let header = match lines.next().expect("Something went wrong while parsing the file.") {
        Err(e) => return Err(Box::new(e)),
        Ok(s) => s,
    };

    for line in lines {
        current_line += 1;

        let contents = line.expect("Something went wrong while parsing the file");
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

    println!("Header Text: {}", header);

    return Ok(batches);
}
