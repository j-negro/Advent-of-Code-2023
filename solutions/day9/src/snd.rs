use day9::parse_file;

fn main() {
    let input = include_str!("../input.txt");

    let steps = sum_extrapolated_values(input);
    // The correct answer is -.
    println!("The sum of all extrapolated values is {}", steps);
}

pub fn sum_extrapolated_values(input: &str) -> i64 {
    todo!()
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
