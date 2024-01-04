use day10::{get_animal_starting_location, parse_file};

fn main() {
    let input = include_str!("../input.txt");

    get_max_lenght(input);

    // The correct answer is -.
    println!("The sum of all extrapolated values is {}", 1);
}

fn get_max_lenght(input: &str) -> u64 {
    let tile_matrix = parse_file(input);

    let animal_position =
        get_animal_starting_location(&tile_matrix).expect("Missing animal starting position");

    dbg!(animal_position);

    todo!();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {
        let input = include_str!("../example_input_1.txt");
    }
}
