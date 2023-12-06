fn main() {
    let input = include_str!("../input.txt");

    let result = lowest_location_number(input);

    println!("The lowest location number is {result}");
}

fn lowest_location_number(input: &str) -> u64 {
    let garden = parse_file(input);

    // Answer for the First Puzzle, which is 199602917.
    // return garden.seeds.iter().map(|s| garden.translate(*s)).min().expect("Unexpected error");

    // Answer for the Second Puzzle, which is 2254687
    let mut seeds = garden.seeds.clone();

    let mut seed_ranges = Vec::new();

    while !seeds.is_empty() {
        let seed_info: Vec<u64> = seeds.drain(0..2).collect();

        let range_start = seed_info[0];
        let range_lenght = seed_info[1];

        let range_end = range_start.wrapping_add(range_lenght);

        seed_ranges.push(range_start..range_end)
    }

    let mut min_location = u64::MAX;

    for range in seed_ranges {
        for seed in range {
            let location = garden.translate(seed);
            if location < min_location {
                min_location = location;
            }
        }
    }
    return min_location;
}

fn parse_file(input: &str) -> GiantGarden {
    let mut lines = input.lines();

    let first_line = lines.next().expect("Error reading file");

    let seeds_str: Vec<&str> = first_line.split(": ").collect();
    let seeds_str: Vec<&str> = seeds_str
        .get(1)
        .expect("Error parsing file")
        .split_whitespace()
        .collect();
    let seeds: Vec<u64> = seeds_str
        .iter()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();

    let mut almanac_maps: Vec<Vec<AlmanacMap>> = Vec::new();
    let mut idx = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let first_char = line.as_bytes()[0];

        if first_char.is_ascii_alphabetic() {
            almanac_maps.push(Vec::new());
            if !almanac_maps[idx].is_empty() {
                idx += 1;
            }
        } else if first_char.is_ascii_digit() {
            almanac_maps[idx].push(parse_map_line(line));
        }
    }

    return GiantGarden::new(seeds, almanac_maps);
}

fn parse_map_line(line: &str) -> AlmanacMap {
    let nums: Vec<u64> = line
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    if nums.len() < 3 {
        panic!("Error in file format.");
    }

    return AlmanacMap::new(nums[0], nums[1], nums[2]);
}

#[cfg(test)]
mod tests {
    use crate::lowest_location_number;

    #[test]
    fn test_example() {
        let input = include_str!("../example_input.txt");

        assert_eq!(lowest_location_number(input), 46)
    }
}

#[derive(Debug)]
struct GiantGarden {
    seeds: Vec<u64>,
    almanac_maps: Vec<Vec<AlmanacMap>>,
}

impl GiantGarden {
    fn new(seeds: Vec<u64>, almanac_maps: Vec<Vec<AlmanacMap>>) -> Self {
        Self {
            seeds: seeds,
            almanac_maps: almanac_maps,
        }
    }

    fn translate(&self, input: u64) -> u64 {
        let mut result = input;

        for category_maps in &self.almanac_maps {
            for map in category_maps {
                if let Some(inner_result) = map.translate(result) {
                    result = inner_result;
                    break;
                }
            }
        }
        return result;
    }
}

#[derive(Debug)]
struct AlmanacMap {
    destination_start: u64,
    source_start: u64,
    range_lenght: u64,
}

impl AlmanacMap {
    fn new(destination_start: u64, source_start: u64, range_lenght: u64) -> Self {
        Self {
            source_start: source_start,
            destination_start: destination_start,
            range_lenght: range_lenght,
        }
    }

    fn translate(&self, input: u64) -> Option<u64> {
        if input < self.source_start {
            return None;
        }
        
        let distance = input - self.source_start;

        if self.range_lenght < distance {
            return None;
        }


        return distance.checked_add(self.destination_start);
    }
}
