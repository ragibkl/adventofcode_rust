use std::collections::HashSet;
use std::fs;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct HouseGrid {
    current: Point,
    hash: HashSet<Point>,
}

impl HouseGrid {
    fn new() -> HouseGrid {
        let mut hash = HashSet::new();
        hash.insert(Point { x: 0, y: 0 });

        HouseGrid {
            current: Point { x: 0, y: 0 },
            hash,
        }
    }

    fn walk_char(&mut self, c: char) {
        match c {
            '^' => self.current.y += 1,
            'v' => self.current.y -= 1,
            '>' => self.current.x += 1,
            '<' => self.current.x -= 1,
            _ => {}
        };
        self.hash.insert(Point {
            x: self.current.x,
            y: self.current.y,
        });
    }

    fn count(&self) -> usize {
        self.hash.len()
    }
}

fn read_input_file() -> String {
    let filename = "input.txt";
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn main() {
    let mut grid = HouseGrid::new();
    for c in read_input_file().chars() {
        grid.walk_char(c);
    }

    println!("count = {}", grid.count());

    let mut santa_grid = HouseGrid::new();
    let mut robo_grid = HouseGrid::new();

    for (i, c) in read_input_file().chars().enumerate() {
        if i % 2 == 0 {
            santa_grid.walk_char(c);
        } else {
            robo_grid.walk_char(c);
        }
    }

    let joint_hash: HashSet<_> = santa_grid.hash.union(&robo_grid.hash).collect();
    println!("count_2 = {}", joint_hash.len());
}
