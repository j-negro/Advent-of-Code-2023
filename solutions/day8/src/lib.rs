use std::collections::HashMap;

use regex::Regex;
use types::Direction;
mod types;

pub fn parse_file(input: &str) -> (Vec<Direction>, HashMap<&str, [&str; 2]>) {
    let mut lines = input.lines();
    // Parse first line, which is a sequence of left and right.

    let fst_line = lines.next().unwrap();

    let directions = fst_line.chars().map(Direction::new).collect::<Vec<_>>();

    // Parse the rest of the file
    let re = Regex::new(r"([A-Z0-9]+) = \(([A-Z0-9]+), ([A-Z0-9]+)\)").unwrap();
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

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..]);

    a * b / gcd(a, b)
}

fn gcd(x: u64, y: u64) -> u64 {
    if y == 0 {
        return x;
    }
    gcd(y, x % y)
}
