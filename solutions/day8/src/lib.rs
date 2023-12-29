use std::collections::HashMap;

use regex::Regex;
use types::Direction;
mod types;

pub fn get_steps_amount(input: &str) -> u64 {
    let (directions, map) = parse_file(input);

    let mut steps = 0;
    let mut idx = 0;

    let mut curr = "AAA";

    loop {
        if curr == "ZZZ" {
            return steps;
        }

        if idx == directions.len() {
            idx = 0;
        }

        let curr_dir = directions[idx].idx();

        let possible_dirs = map.get(curr).unwrap();

        curr = possible_dirs[curr_dir];

        steps += 1;
        idx += 1;
    }
}

fn parse_file(input: &str) -> (Vec<Direction>, HashMap<&str, [&str; 2]>) {
    let mut lines = input.lines();
    // Parse first line, which is a sequence of left and right.

    let fst_line = lines.next().unwrap();

    let directions = fst_line.chars().map(Direction::new).collect::<Vec<_>>();

    // Parse the rest of the file
    let re = Regex::new(r"([A-Z]+) = \(([A-Z]+), ([A-Z]+)\)").unwrap();
    let mut map = HashMap::new();

    for line in lines {
        let Some(captures) = re.captures(line) else {
            continue;
        };

        let start = captures.get(1).unwrap().as_str();
        let left = captures.get(2).unwrap().as_str();
        let right = captures.get(3).unwrap().as_str();

        map.insert(start, [left, right]);
    }

    (directions, map)
}

#[cfg(test)]
mod tests {
    use crate::get_steps_amount;

    #[test]
    fn test_example() {
        let input = include_str!("../example_input_1.txt");

        assert_eq!(get_steps_amount(input), 2);

        let input = include_str!("../example_input_2.txt");

        assert_eq!(get_steps_amount(input), 6);
    }
}
