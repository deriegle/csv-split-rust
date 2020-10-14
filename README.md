# CSV File Splitter written in Rust

This repo contains both the Rust implementation of the CSV splitter as well as the
`csv-split.rb` file from [this repo](https://github.com/imartingraham/csv-split).

When working with really large CSV files, it takes a very long time for the ruby version to finish. I decided to rewrite this tool in
Rust to improve the performance and to improve my Rust knowledge.

## Benchmarks

I ran each sample file 5 times through the programs to give me the average run time for each file size.
I've included a `./sample-data.csv` file in this repo to provide a sample of the data that was used while running these tests.
The only difference is the number of rows in the file, but the data was consistent in each row.

*The highest number of rows that I was able to run the rust program was around 30 million. The file size was 818MB. It ran in about ~12 seconds when using `10000` as the batch size. I tried to add another 5 million to it (35 million total with around 920MB file size), but the process would get killed every time. There are probably some memory improvemnets that we could make to help with this limitation, but hopefully this won't be a huge problem for anyone.*

| Language | File Size | Line Count | Batch Size | Avg. Time in seconds |
| :--: | :--: | :--: | :--: | :--: |
| Rust | 4.0KB | 5 | 1 | 0.0003829 |
| Ruby | 4.0KB | 5 | 1 | 0.001946943 |
| - | - | - | - | - |
| Rust | 28KB | 1,000 | 1 | 0.03352806 |
| Ruby | 28KB | 1,000 | 1 | 0.094704671 |
| - | - | - | - | - |
| Rust | 1.7MB | 60,000 | 1 | 1.115898174 |
| Ruby | 1.7MB | 60,000 | 1 | 5.387614667 |
| - | - | - | - | - |
| Rust | 2.7MB | 1,000,000 | 100 | 0.042519370 |
| Ruby | 2.7MB | 1,000,000 | 100 | 1.710918996 |
| - | - | - | - | - |
| Rust | 289MB | 10,790,000 | 10,000 | 1.678728042 |
| Ruby | 289MB | 10,790,000 | 10,000 | 188.9817373 |

### Run Rust CSV splitter

It takes two arguments
1. File Name of CSV file to split
2. Batch size per split file

```bash
$ cargo run --release ./sample-data.csv 1
```

### Run Ruby CSV splitter

Copy the `csv-split.rb` file from [this repo](https://github.com/imartingraham/csv-split)

```bash
$ ruby ./csv-split.rb -f ./sample-data.csv -l 1 --include-headers
```
