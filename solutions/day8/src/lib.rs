

fn get_steps_amount(input: &str) -> u64 {
    todo!();
}



fn parse_file(input: &str) {
    for line in input.lines() {
        let line_arr: Vec<&str> = line
            .split_whitespace()
            .collect();


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
