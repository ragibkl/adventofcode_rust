use std::fs;
use std::io::{BufRead, BufReader};

mod lib;
use lib::is_nice;

fn read_input_lines() -> Vec<String> {
    let filename = "input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = fs::File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

fn main() {
    let lines = read_input_lines();
    let nice_list: Vec<String> = lines
        .iter()
        .filter(|x| is_nice(&x))
        .map(|x| x.clone())
        .collect();

    println!("nice count = {}", nice_list.len());
}
