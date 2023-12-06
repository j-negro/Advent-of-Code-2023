use day6::{multiply_ways_to_win, PreviousRace};



fn main() {
    let input = include_str!("../input.txt");

    let races = parse_file(input);

    // Answer for the First Puzzle: 781200
    println!(
        "The product of all the number of ways to beat each record is: {}",
        multiply_ways_to_win(races)
    );
}

fn parse_file(input: &str) -> Vec<PreviousRace> {
    let lines: Vec<&str> = input.lines().collect();

    let times_arr: Vec<u64> = lines[0].split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();
    let distances_arr: Vec<u64> = lines[1].split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect();

    let mut races = Vec::new();
    for idx in 0..times_arr.len() {
        races.push(PreviousRace::new(times_arr[idx], distances_arr[idx]))
    }

    return races;
}


#[cfg(test)]
mod tests {
    use crate::{multiply_ways_to_win, parse_file};

    #[test]
    fn test_example() {
        let input = include_str!("../example_input.txt");

        let races = parse_file(input);

        assert_eq!(multiply_ways_to_win(races), 288)
    }
}