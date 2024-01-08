use day10::TileGrid;

fn main() {
    let input = include_str!("../input.txt");

    // The correct answer is -.
    println!(
        "The maximum distance away from the animal is {}",
        get_max_lenght(input)
    );
}

fn get_max_lenght(input: &str) -> usize {
    let grid = input.parse::<TileGrid>().unwrap();

    let start = grid.get_animal_starting_location().unwrap();

    let loop_positions = grid.loop_positions(start);

    loop_positions.len() / 2
}

#[cfg(test)]
mod tests {

    use crate::get_max_lenght;

    #[test]
    fn test_example() {
        let input = include_str!("../example_input_1.txt");

        assert_eq!(4, get_max_lenght(input));

        let input = include_str!("../example_input_2.txt");

        assert_eq!(8, get_max_lenght(input));
    }
}
