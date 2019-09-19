use std::fs;
use std::io::{BufRead, BufReader};

fn read_input_file() -> String {
    let filename = "input.txt";
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn read_input_lines() -> Vec<String> {
    let filename = "input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| { l.unwrap() }).collect()
}

fn main() {
    let lines = read_input_lines();
    for line in lines {
        println!("{}", line);
    }
}
