pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn new(direc: char) -> Self {
        match direc {
            'L' => Direction::Left,
            'R' => Direction::Right,
            direc => {
                panic!(
                    "Error parsing the directions, found unexpected char {}",
                    direc
                )
            }
        }
    }

    pub fn idx(&self) -> usize {
        match self {
            Direction::Left => 0,
            Direction::Right => 1,
        }
    }
}
