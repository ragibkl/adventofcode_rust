use std::fs;
use std::io::{BufRead, BufReader};

mod lib;

fn read_input_lines() -> Vec<String> {
    let filename = "input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

fn main() {
    let lines = read_input_lines();
    
    for l in lines {
        println!("{}", l);
    }
}
