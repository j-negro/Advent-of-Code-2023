use day6::get_steps_amount;

fn main() {
    let input = include_str!("../input.txt");

    let steps = get_steps_amount(input);
    println!("The amount of steps needed is {}", steps);
}
