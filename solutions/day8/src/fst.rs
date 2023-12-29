use day6::parse_file;

fn main() {
    let input = include_str!("../input.txt");

    let steps = get_steps_amount(input);
    println!("The amount of steps needed is {}", steps);
}

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
