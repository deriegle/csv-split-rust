use std::fs::File;
use std::io::{self,BufRead};

const BATCH_SIZE: i32 = 10_000;

fn main() -> std::io::Result<()> {
    let mut header: std::option::Option<String> = None;
    let file = File::open("./test.csv")?;
    let reader = io::BufReader::new(file);
    let mut batches: std::vec::Vec<std::vec::Vec<String>> = Vec::new();
    let mut current_line: i32 = 0;

    let start = std::time::Instant::now();

    for line in reader.lines() {
        current_line += 1;

        if header.is_none() {
            header = Some(line.unwrap());
        } else {
            let batch_number = (current_line / BATCH_SIZE) as usize;

            match batches.get_mut(batch_number) {
                None => {
                    let mut new_batch = Vec::new();
                    new_batch.push(line?);
                    batches.insert(batch_number, new_batch);
                },
                Some(batch) => {
                    batch.push(line?);
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

    println!("{:.2?} seconds for processing {} batches", elapsed, batches.len());

    Ok(())
}
