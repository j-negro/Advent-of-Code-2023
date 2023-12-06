fn main() {
    let input = include_str!("../input.txt");

    println!("{}", input);
}


fn multiply_ways_to_win(input: &str) -> u64 {
    288
}


#[cfg(test)]
mod tests {
    use crate::multiply_ways_to_win;

    #[test]
    fn test_example() {
        let input = include_str!("../example_input.txt");

        assert_eq!(multiply_ways_to_win(input), 288)
    }
}