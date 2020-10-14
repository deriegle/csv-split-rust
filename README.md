CSV File Splitter written in Rust

There is an existing CSV Splitter written in ruby.
It's included in this repo under csv-split.rb
You can view it [here](https://github.com/imartingraham/csv-split).

I noticed that it was quick slow when processing large files, so I wanted to try
writing a similar program in Rust to compare the time difference.

| Language | File Size | Line Count | Batch Size | Time in seconds |
| Rust | | 5 | 1 | |
| Ruby | | 5 | 1 | |
| Rust | | 1000 | 1 | |
| Ruby | | 1000 | 1 | |
| Rust |  289MB | 10,790,000 | 10,000 | 1.09 |
| Ruby | 289MB | 10,790,000 | 10,000 | |

