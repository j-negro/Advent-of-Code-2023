use day10::TileGrid;

fn main() {
    let input = include_str!("../input.txt");

    // The correct answer is 381.
    println!(
        "The number of tiles enclosed by the loop is {}",
        get_tiles_enclosed_by_loop(input)
    );
}

fn get_tiles_enclosed_by_loop(input: &str) -> usize {
    let grid = input.parse::<TileGrid>().unwrap();

    let start = grid.get_animal_starting_location().unwrap();

    let positions = grid.loop_positions(start);

    let curve_area = shoelace_formula(&positions);

    pick_theorem(&positions, curve_area)
}

fn shoelace_formula(positions: &Vec<[usize; 2]>) -> f64 {
    let len = positions.len();

    (0..len)
        .map(|i| {
            let sum_ys = (positions[i][1] + positions[(i + 1) % len][1]) as i64;
            let diff_xs = positions[i][0] as i64 - positions[(i + 1) % len][0] as i64;

            sum_ys * diff_xs
        })
        .sum::<i64>()
        .abs() as f64
        / 2f64
}

fn pick_theorem(positions: &Vec<[usize; 2]>, area: f64) -> usize {
    let boundary_points = positions.len();

    (area - boundary_points as f64 / 2f64 + 1f64).ceil() as usize
}

#[cfg(test)]
mod tests {

    use crate::get_tiles_enclosed_by_loop;

    #[test]
    fn test_example() {
        let input = include_str!("../example_input_3.txt");

        assert_eq!(8, get_tiles_enclosed_by_loop(input));

        let input = include_str!("../example_input_4.txt");

        assert_eq!(10, get_tiles_enclosed_by_loop(input));
    }
}
