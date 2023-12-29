use day7::get_total_winnings;

fn main() {
    let input = include_str!("../input.txt");

    let winnings = get_total_winnings(input);

    // First puzzle answer is 253638586.
    println!("These hands have a total winnings count of {}", winnings);
}
