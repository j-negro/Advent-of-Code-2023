use types::HandType;

mod types;

pub fn get_total_winnings(input: &str) -> u64 {
    todo!()
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
    use crate::get_total_winnings;

    #[test]
    fn test_example() {
        let input = include_str!("../example_input.txt");

        assert_eq!(get_total_winnings(input), 6440)
    }
}
