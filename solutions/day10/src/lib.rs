use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn parse_file(input: &str) -> Vec<Vec<Tile>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|n| n.to_string().parse::<Tile>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn get_animal_starting_location(tile_matrix: &Vec<Vec<Tile>>) -> Option<[usize; 2]> {
    tile_matrix
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|v| *v == Tile::Animal).map(|j| [i, j]))
}

pub fn loop_lenght(tile_matrix: &Vec<Vec<Tile>>, animal_starting_position: [usize; 2]) -> u64 {
    let mut loop_lenght = 0;

    // By definition, this tile will be connected to only 2 tiles.

    loop_lenght
}

fn step_on_loop(tile_matrix: &Vec<Vec<Tile>>, prev_direction: Direction) {
    for direction in Direction::iter() {
        if direction == prev_direction {
            continue;
        }
        match direction {
            Direction::North => todo!(),
            Direction::South => todo!(),
            Direction::East => todo!(),
            Direction::West => todo!(),
        }
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
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
