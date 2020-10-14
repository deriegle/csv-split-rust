pub mod config;

use std::fs::File;
use std::io::{self,BufRead,Lines};
use std::error::Error;

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.filename)?;
    let reader = io::BufReader::new(file);

    let start = std::time::Instant::now();

    let result = process_lines(reader.lines(), config.batch_size).unwrap();
    let mut file_number: i32 = 0;

    for batch in result.batches.iter() {
        file_number += 1;
        let file_name = format!("test-{}.csv", file_number);
        let mut file_contents = result.header.clone();

        file_contents.push('\n');
        file_contents.push_str(batch.join("\n").as_str());

        create_and_write_file(&file_name, &file_contents).expect(format!("Error creating file {}", file_name).as_str());
    }

    let elapsed = start.elapsed();

    println!(
        "Took {:.5} seconds to process {} batches.",
        elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * 1e-9,
        result.batches.len()
    );

    Ok(())
}

fn process_lines(mut lines: Lines<io::BufReader<File>>, batch_size: i32) -> Result<ProcessResult, Box<dyn Error>> {
    let mut current_line: i32 = 0;
    let mut batches: Vec<Vec<String>> = Vec::new();

    let header = match lines.next().expect("Something went wrong while parsing the file.") {
        Err(e) => return Err(Box::new(e)),
        Ok(s) => s,
    };

    for line in lines {
        current_line += 1;

        let contents = line.expect("Something went wrong while parsing the file");
        let batch_number = (current_line / batch_size) as usize;

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

    return Ok(ProcessResult { header, batches });
}

struct ProcessResult {
    header: String,
    batches: Vec<Vec<String>>
}

fn create_and_write_file(file_name: &String, contents: &String) -> Result<(), Box<dyn Error>> {
    std::fs::write(file_name, contents)?;
    return Ok(());
}
