pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<bool>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        let data = vec![vec![false; width]; height];
        Grid { width, height, data }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.data[x][y]
    }

    fn turn_on(&mut self, x: usize, y: usize) {
        self.data[x][y] = true;
    }

    fn turn_off(&mut self, x: usize, y: usize) {
        self.data[x][y] = false;
    }

    fn toggle(&mut self, x: usize, y: usize) {
        self.data[x][y] = !self.data[x][y];
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_on() {
        let mut g = Grid::new(10, 10);
        g.turn_on(0, 0);
        assert_eq!(true, g.get(0, 0));
    }
}
