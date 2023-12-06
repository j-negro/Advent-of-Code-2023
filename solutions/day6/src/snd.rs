use day6::{multiply_ways_to_win, PreviousRace};

fn main() {
    let input = include_str!("../input.txt");

    let races = parse_file(input);

    // Answer for the Second Puzzle: 49240091
    println!(
        "The number of ways to beat this record is: {}",
        multiply_ways_to_win(races)
    );
}

fn parse_file(input: &str) -> Vec<PreviousRace> {
    let lines: Vec<&str> = input.lines().collect();

    let time = lines[0].split(':').collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let distance = lines[1].split(':').collect::<Vec<&str>>()[1]
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    return vec![PreviousRace::new(time, distance)];
}

#[cfg(test)]
mod tests {
    use crate::{multiply_ways_to_win, parse_file};

    #[test]
    fn test_example() {
        let input = include_str!("../example_input.txt");

        let races = parse_file(input);

        assert_eq!(multiply_ways_to_win(races), 71503)
    }
}
