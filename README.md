CSV File Splitter written in Rust

There is an existing CSV Splitter written in ruby.
It's included in this repo under csv-split.rb
You can view it [here](https://github.com/imartingraham/csv-split).

I noticed that it was quick slow when processing large files, so I wanted to try
writing a similar program in Rust to compare the time difference.

| Language | File Size | Line Count | Batch Size | Time |
| Rust | 4.0K | 5 | 1 | 6.79 micro seconds |
| Ruby | 4.0K | 5 | 1 | |
| Rust | 28K | 1000 | 1 | 122.15 micro seconds |
| Ruby | 28K | 1000 | 1 | |
| Rust |  289MB | 10,790,000 | 10,000 | 1.05s |
| Ruby | 289MB | 10,790,000 | 10,000 | |

