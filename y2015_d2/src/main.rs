use std::fs::File;
use std::io::{BufRead, BufReader};


struct GiftBox {
    x: i32,
    y: i32,
    z: i32,
}

impl GiftBox {
    fn new(x: i32, y: i32, z: i32) -> GiftBox {
        GiftBox { x, y, z}
    }

    fn from_str(input: String) -> GiftBox {
        let parts: Vec<String> = input.split("x").map(|s| s.to_string()).collect();
        let mut parts: Vec<i32> = parts.iter().map(|x| { x.parse::<i32>().unwrap() }).collect();
        parts.sort();

        GiftBox::new(parts[0], parts[1], parts[2])
    }

    fn get_min_area(&self) -> i32 {
        self.x * self.y
    }

    fn get_area(&self) -> i32 {
        2 * (self.x * self.y + self.y * self.z + self.z * self.x)
    }

    fn get_total_area(&self) -> i32 {
        self.get_area() + self.get_min_area()
    }

    fn get_ribbon_length(&self) -> i32 {
        2 * (self.x + self.y) + self.x * self.y * self.z
    }
}


fn read_input_lines() -> Vec<String> {
    let filename = "input.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|l| { l.unwrap() }).collect()
}

fn main() {
    println!("Hello, World!");
    let mut sum_area = 0;
    let mut sum_ribbon = 0;

    for line in read_input_lines() {
        let gift_box = GiftBox::from_str(line);
        sum_area += gift_box.get_total_area();
        sum_ribbon += gift_box.get_ribbon_length();
    }
    println!("total_area = {}", sum_area);
    println!("total_ribbon = {}", sum_ribbon);
}
