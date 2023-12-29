use std::collections::BTreeSet;

use types::hand::Hand;

mod types;

pub fn get_total_winnings(input: &str) -> u64 {
    let hands = parse_file(input);

    return hands
        .iter()
        .enumerate()
        .map(|(idx, hand)| (idx as u64 + 1) * hand.bid)
        .sum();
}

fn parse_file(input: &str) -> BTreeSet<Hand> {
    let hands: BTreeSet<Hand> = input.lines().map(Hand::new).collect();
    return hands;
}

#[cfg(test)]
mod tests {
    use crate::get_total_winnings;

    #[test]
    fn test_example() {
        let input = include_str!("../example_input.txt");

        assert_eq!(get_total_winnings(input), 6440)
    }
}
