use day9::{get_extrapolated_value, parse_file};

fn main() {
    let input = include_str!("../input.txt");

    let steps = sum_extrapolated_values(input);
    // The correct answer is 1637452029.
    println!("The sum of all extrapolated values is {}", steps);
}

pub fn sum_extrapolated_values(input: &str) -> i64 {
    let values = parse_file(input);

    values
        .iter()
        .map(|history| get_extrapolated_value(history))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::sum_extrapolated_values;

    #[test]
    fn test_example() {
        let input = include_str!("../example_input.txt");

        assert_eq!(sum_extrapolated_values(input), 114);
    }
}
