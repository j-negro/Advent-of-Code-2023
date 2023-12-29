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

    let mut currs = map
        .keys()
        .filter(|key| key.ends_with("A"))
        .collect::<Vec<_>>();

    loop {
        if currs.iter().all(|cur| cur.ends_with("Z")) {
            return steps;
        }

        if idx == directions.len() {
            idx = 0;
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
        idx += 1;
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
