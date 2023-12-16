use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;

use super::{card_type::CardType, hand_type::HandType};

#[derive(PartialEq, Eq, Debug)]
pub struct Hand {
    cards: [CardType; 5],
    hand_type: HandType,
    pub bid: u64,
}

impl Hand {
    pub fn new(line: &str) -> Self {
        let line: Vec<&str> = line.split_whitespace().collect();

        let cards: Vec<CardType> = line[0].trim().chars().map(CardType::new).collect();

        let cards: [CardType; 5] = cards.try_into().unwrap_or_else(|v: Vec<CardType>| {
            panic!(
                "All hands must be of lenght {}, but found a hand with lenght {}",
                5,
                v.len()
            )
        });

        let bid: u64 = line[1].parse().unwrap();

        return Self {
            cards: cards,
            hand_type: Hand::get_hand_type(cards),
            bid: bid,
        };
    }

    fn get_hand_type(cards: [CardType; 5]) -> HandType {
        let card_map: HashMap<CardType, u8> = cards.iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(*card).or_insert(0) += 1;
            map
        });

        let card_counts: Vec<&u8> = card_map
            .values()
            .sorted_by(|x, y| y.cmp(x))
            .collect::<Vec<&u8>>();

        return match card_counts[..] {
            [5] => HandType::FiveOAK,
            [4, 1] => HandType::FourOAK,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOAK,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!("Unexpected hand type"),
        };
    }

    fn cmp_hands_by_card(&self, other: &Hand) -> Ordering {
        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            match self_card.cmp(other_card) {
                Ordering::Equal => continue,
                otherwise => return otherwise,
            }
        }

        Ordering::Less
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cmp_hands_by_card(&other),
            otherwise => otherwise,
        }
    }
}
