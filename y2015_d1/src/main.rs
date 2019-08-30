use std::fs;

fn count_floor(input: &String) -> i32 {
    let mut floor_count = 0;
    for c in input.chars() {
        match c {
            '(' => floor_count += 1,
            ')' => floor_count -= 1,
            _ => {},
        };
    }

    floor_count
}

fn get_position_target(input: &String, target: i32) -> Option<usize> {
    let mut floor_count = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor_count += 1,
            ')' => floor_count -= 1,
            _ => {},
        };
        if floor_count == target {
            return Some(i + 1)
        }
    }

    None
}

fn read_input_file() -> String {
    let filename = "input.txt";
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")   
}

fn main() {
    let input = read_input_file();

    let floor_count = count_floor(&input);
    println!("floor_count = {}", floor_count);

    let target = -1;
    let index = get_position_target(&input, target);
    match index {
        Some(i) => { println!("reached floor {} at position {}", target, i)},
        None => { println!("never reached floor {}", target) }
    }
}
