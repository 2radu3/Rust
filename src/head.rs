use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn head(arguments: &[String]) {
    let mut num_lines: usize = 10;

    let mut start_index = 0;

    if let Some(arg) = arguments.get(0) {
        if arg == "-n" || arg == "--number-of-lines" {
            if let Some(n_str) = arguments.get(1) {
                num_lines = n_str.parse().unwrap_or(10);
                start_index = 2;
            }
        }
    }

    for filename in &arguments[start_index..] {
        let file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("head: cannot open '{}': {}", filename, e);
                continue;
            }
        };

        let reader = BufReader::new(file);

        for line in reader.lines().take(num_lines) {
            match line {
                Ok(text) => println!("{}", text),
                Err(e) => {
                    eprintln!("head: error reading '{}': {}", filename, e);
                    break;
                }
            }
        }
    }
}