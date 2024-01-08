use std::str::FromStr;

#[derive(Debug)]
pub struct TileGrid {
    tiles: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
}

impl FromStr for TileGrid {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let tiles = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|n| n.to_string().parse::<Tile>())
                    .collect()
            })
            .collect::<Result<Vec<Vec<Tile>>, _>>()?;

        let height = tiles.len();
        let width = tiles[0].len();

        Ok(Self {
            tiles,
            height,
            width,
        })
    }
}

impl TileGrid {
    pub fn get_animal_starting_location(&self) -> Option<[usize; 2]> {
        self.tiles
            .iter()
            .enumerate()
            .find_map(|(i, row)| row.iter().position(|v| *v == Tile::Animal).map(|j| [i, j]))
    }

    pub fn loop_positions(&self, animal_starting_position: [usize; 2]) -> Vec<[usize; 2]> {
        let mut loop_positions = vec![animal_starting_position];

        // HACK: In all inputs, the loop starts south of the animal.
        let mut position = [animal_starting_position[0] + 1, animal_starting_position[1]];
        let mut direction = Direction::North;

        loop {
            (position, direction) = self.loop_step(position, direction).unwrap();

            loop_positions.push(position);
            if self.tiles[position[0]][position[1]] == Tile::Animal {
                break;
            }
        }

        loop_positions
    }

    fn loop_step(
        &self,
        [x, y]: [usize; 2],
        prev_direction: Direction,
    ) -> Option<([usize; 2], Direction)> {
        let tile = &self.tiles[x][y];

        let next_direction = tile.next_direction(&prev_direction)?;

        let next_position = self.next_position([x, y], next_direction)?;

        Some((next_position, next_direction.opposing_direction()))
    }

    fn next_position(&self, [x, y]: [usize; 2], prev_direction: Direction) -> Option<[usize; 2]> {
        Some(match prev_direction {
            Direction::North => [x.checked_sub(1)?, y],
            Direction::South => [x.checked_add(1)?, y],
            Direction::West => [x, y.checked_sub(1)?],
            Direction::East => [x, y.checked_add(1)?],
        })
        .filter(|[new_x, new_y]| new_x < &self.height && new_y < &self.width)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposing_direction(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    pub fn connected_tiles(&self) -> [Tile; 3] {
        match self {
            Direction::North => [Tile::Vertical, Tile::SouthEast, Tile::SouthWest],
            Direction::South => [Tile::Vertical, Tile::NorthEast, Tile::NorthWest],
            Direction::East => [Tile::Horizontal, Tile::NorthWest, Tile::SouthWest],
            Direction::West => [Tile::Horizontal, Tile::NorthEast, Tile::SouthEast],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Vertical,
    Horizontal,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Ground,
    Animal,
}

impl FromStr for Tile {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "|" => Ok(Tile::Vertical),
            "-" => Ok(Tile::Horizontal),
            "L" => Ok(Tile::NorthEast),
            "J" => Ok(Tile::NorthWest),
            "7" => Ok(Tile::SouthWest),
            "F" => Ok(Tile::SouthEast),
            "." => Ok(Tile::Ground),
            "S" => Ok(Tile::Animal),
            _ => Err("Unknown tile character"),
        }
    }
}

impl Tile {
    pub fn connects(&self) -> [Direction; 2] {
        match self {
            Tile::Vertical => [Direction::North, Direction::South],
            Tile::Horizontal => [Direction::West, Direction::East],
            Tile::NorthWest => [Direction::North, Direction::West],
            Tile::NorthEast => [Direction::North, Direction::East],
            Tile::SouthWest => [Direction::South, Direction::West],
            Tile::SouthEast => [Direction::South, Direction::East],
            Tile::Ground | Tile::Animal => panic!("Unexpected error"),
        }
    }

    pub fn next_direction(&self, direction: &Direction) -> Option<Direction> {
        self.connects()
            .iter()
            .find(|direc| *direc != direction)
            .copied()
    }
}
