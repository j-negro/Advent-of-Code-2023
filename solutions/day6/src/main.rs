fn main() {
    let input = include_str!("../input.txt");

    // Answer for the First Puzzle: 781200
    println!(
        "The product of all the number of ways to beat each record is: {}",
        multiply_ways_to_win(input)
    );
}

fn multiply_ways_to_win(input: &str) -> u64 {
    let races = parse_file_snd_puzzle(input);

    return races
        .iter()
        .map(|race| race.possible_charge_times())
        .fold(1, |acc, x| acc * x);
}

fn parse_file_fst_puzzle(input: &str) -> Vec<PreviousRace> {
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

fn parse_file_snd_puzzle(input: &str) -> Vec<PreviousRace> {
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

#[derive(Debug)]
struct PreviousRace {
    time: u64,
    distance: u64,
}

impl PreviousRace {
    fn new(time: u64, distance: u64) -> Self {
        Self {
            time: time,
            distance: distance,
        }
    }

    fn possible_charge_times(&self) -> u64 {
        let discriminant: f64 = (self.time.pow(2) as f64) - 4.0 * (self.distance as f64);

        if discriminant < 1e-8 {
            return 0;
        }

        let delta = (discriminant as f64).sqrt();

        let range_start = ((self.time as f64) - delta) / 2.0;
        let range_end = ((self.time as f64) + delta) / 2.0;

        let range_start = if range_start.fract() == 0.0 {
            (range_start.ceil() as u64) + 1
        } else {
            range_start.ceil() as u64
        };

        let range_end = if range_end.fract() == 0.0 {
            (range_end.floor() as u64) - 1
        } else {
            range_end.floor() as u64
        };

        return range_end - range_start + 1;
    }
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
