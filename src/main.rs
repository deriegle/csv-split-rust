use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

const BATCH_SIZE: i32 = 10_000;


fn main() {
    let mut header = String::new();
    let mut reader = read_csv_lines("./test.csv").unwrap();

    let reader_ref = &mut reader;

    println!("{} lines", reader_ref.lines().count());

    reader.read_line(&mut header).unwrap();

    let reader_ref_2 = &mut reader;

    println!("{}", header);
    println!("{} lines", reader_ref_2.lines().count());
}

fn read_csv_lines<P>(filename: P) -> io::Result<io::BufReader<File>> where P: AsRef<Path>, {
    let file = File::open(filename)?;

    Ok(io::BufReader::new(file))
}
