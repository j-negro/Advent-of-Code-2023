use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum CardType {
    A,
    K,
    Q,
    J,
    T,
    Number(u8),
}

impl CardType {
    pub fn new(card: char) -> Self {
        match card {
            'A' => CardType::A,
            'K' => CardType::K,
            'Q' => CardType::Q,
            'J' => CardType::J,
            'T' => CardType::T,
            _ => {
                if card.is_ascii_digit() {
                    CardType::Number(card as u8)
                } else {
                    panic!(
                        "Error parsing the cards, found unexpected char {}",
                        card.to_string()
                    )
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOAK,
    FourOAK,
    FullHouse,
    ThreeOAK,
    TwoOAK,
    TwoPair,
    OnePair,
    HighCard,
}

pub struct Hand {
    cards: Vec<CardType>,
}

impl Hand {
    fn get_hand_type(&self) -> HandType {
        let card_map: HashMap<CardType, u8> =
            self.cards.iter().fold(HashMap::new(), |mut map, card| {
                *map.entry(*card).or_insert(0) += 1;
                map
            });

        let highest_count_entry = card_map
            .iter()
            .max_by(|left, right| left.1.cmp(right.1))
            .unwrap();

        return match highest_count_entry.1 {
            5 => HandType::FiveOAK,
            4 => HandType::FourOAK,
            3 => {
                todo!()
            }
            2 => {
                todo!()
            }
            1 => HandType::HighCard,
            _ => panic!("Parsing Error")
        };
    }
}
