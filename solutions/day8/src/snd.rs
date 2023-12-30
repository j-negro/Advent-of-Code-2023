use std::iter;

use day8::{lcm, parse_file};

fn main() {
    let input = include_str!("../input.txt");

    let steps = get_steps_amount(input);
    // The correct answer is 8906539031197.
    println!("The amount of steps needed is {}", steps);
}

pub fn get_steps_amount(input: &str) -> u64 {
    let (directions, map) = parse_file(input);

    let mut steps = 0;
    let mut idx = 0;

    let mut currs = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();

    let mut loop_periods = iter::repeat(0).take(currs.len()).collect::<Vec<u64>>();

    loop {
        for i in 0..currs.len() {
            if loop_periods[i] == 0 && currs[i].ends_with('Z') {
                loop_periods[i] = steps;
            }
        }
        if loop_periods.iter().all(|v| *v != 0) {
            return lcm(loop_periods.as_slice());
        }

        let curr_dir = directions[idx].idx();

        let possible_dirs = currs
            .iter()
            .map(|curr| map.get(*curr).unwrap())
            .collect::<Vec<_>>();

        currs = possible_dirs
            .iter()
            .map(|dirs| &dirs[curr_dir])
            .collect::<Vec<_>>();

        steps += 1;
        idx = (idx + 1) % directions.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::get_steps_amount;

    #[test]
    fn test_example() {
        let input = include_str!("../example_input_3.txt");

        assert_eq!(get_steps_amount(input), 6);
    }
}
