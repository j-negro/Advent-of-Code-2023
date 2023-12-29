use std::cmp::Ordering;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
                    CardType::Number(card as u8 - b'0')
                } else {
                    panic!("Error parsing the cards, found unexpected char {}", card)
                }
            }
        }
    }
}

impl PartialOrd for CardType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        match self {
            CardType::A => Ordering::Greater,
            CardType::K => match other {
                CardType::A => Ordering::Less,
                _ => Ordering::Greater,
            },
            CardType::Q => match other {
                CardType::A | CardType::K => Ordering::Less,
                _ => Ordering::Greater,
            },
            CardType::T => match other {
                CardType::Number(_) | CardType::J => Ordering::Greater,
                _ => Ordering::Less,
            },
            CardType::Number(a) => match other {
                CardType::Number(b) => a.cmp(b),
                CardType::J => Ordering::Greater,
                _ => Ordering::Less,
            },
            CardType::J => Ordering::Less,
        }
    }
}
