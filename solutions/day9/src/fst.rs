use day9::{construct_extrapolation_sequences, parse_file};

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

pub fn get_extrapolated_value(history: &Vec<i64>) -> i64 {
    let mut sequences: Vec<Vec<i64>> = construct_extrapolation_sequences(history);
    let last_sequence_idx = sequences.len() - 1;

    let last_num = sequences[last_sequence_idx][0];

    sequences[last_sequence_idx].push(last_num);

    for rev_idx in (0..last_sequence_idx).rev() {
        let prev_num = sequences[rev_idx + 1].last().unwrap();

        let new_num = sequences[rev_idx].last().unwrap() + prev_num;

        sequences[rev_idx].push(new_num);
    }

    *sequences[0].last().unwrap()
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
