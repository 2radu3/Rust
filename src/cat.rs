use std::fs::File;
use std::io::{BufRead, BufReader};

pub(crate) fn cat(arguments: &[String]) {
    
    if arguments.is_empty() {
        eprintln!("cat: missing file operand");
        eprintln!("Try './your_bin cat file.txt'");
        std::process::exit(1);
    }

    for filename in arguments {
      
        let file = match File::open(filename) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("cat: {}: {}", filename, e);
                continue; 
            }
        };

     
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(text) => println!("{}", text),
                Err(e) => {
                    eprintln!("cat: error reading {}: {}", filename, e);
                    break;
                }
            }
        }
    }
}