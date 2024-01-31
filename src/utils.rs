use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_words(filename: &str) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let file = File::open(filename).expect("File not found!");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => res.push(line),
            Err(e) => println!("Error rezading line: {}", e),
        }
    }
    res
}