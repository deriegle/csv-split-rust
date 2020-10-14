# CSV File Splitter written in Rust

This repo contains both the Rust implementation of the CSV splitter as well as the
`csv-split.rb` file from [this repo](https://github.com/imartingraham/csv-split).

When working with really large CSV files, it takes a very long time for the ruby version to finish. I decided to rewrite this tool in
Rust to improve the performance and to improve my Rust knowledge.

## Benchmarks

I ran each sample file 5 times through the programs to give me the average run time for each file size.
I've included a `./sample-data.csv` file in this repo to provide a sample of the data that was used while running these tests.
The only difference is the number of rows in the file, but the data was consistent in each row.

| Language | File Size | Line Count | Batch Size | Avg. Time |
| -- | -- | -- | -- | -- |
| Rust | 4.0KB | 5 | 1 | 0.0003829 seconds |
| Ruby | 4.0KB | 5 | 1 | 0.001946943 seconds |
| Rust | 28KB | 1,000 | 1 | 0.03352806 seconds |
| Ruby | 28KB | 1,000 | 1 | 0.094704671 seconds |
| Rust | 289MB | 10,790,000 | 10,000 | 1.678728042 seconds |
| Ruby | 289MB | 10,790,000 | 10,000 | |
