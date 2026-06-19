use super::load;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Spade,
    Heart,
    Diamond,
    Club,
}

impl Kind {
    fn from(s: &str) -> Self {
        match s {
            "S" => Kind::Spade,
            "H" => Kind::Heart,
            "D" => Kind::Diamond,
            "C" => Kind::Club,
            _ => panic!("invalid kind"),
        }
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Kind::Spade => "♠",
            Kind::Heart => "♥",
            Kind::Diamond => "♦",
            Kind::Club => "♣",
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Value {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Value {
    fn from(s: &str) -> Self {
        match s {
            "2" => Value::Two,
            "3" => Value::Three,
            "4" => Value::Four,
            "5" => Value::Five,
            "6" => Value::Six,
            "7" => Value::Seven,
            "8" => Value::Eight,
            "9" => Value::Nine,
            "T" => Value::Ten,
            "J" => Value::Jack,
            "Q" => Value::Queen,
            "K" => Value::King,
            "A" => Value::Ace,
            _ => panic!("invalid rank"),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Value::Two => "2".to_string(),
            Value::Three => "3".to_string(),
            Value::Four => "4".to_string(),
            Value::Five => "5".to_string(),
            Value::Six => "6".to_string(),
            Value::Seven => "7".to_string(),
            Value::Eight => "8".to_string(),
            Value::Nine => "9".to_string(),
            Value::Ten => "10".to_string(),
            Value::Jack => "J".to_string(),
            Value::Queen => "Q".to_string(),
            Value::King => "K".to_string(),
            Value::Ace => "A".to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Card {
    kind: Kind,
    value: Value,
}

impl Card {
    fn from(s: &str) -> Self {
        let value = Value::from(&s[0..1]);
        let kind = Kind::from(&s[1..2]);
        Card { kind, value }
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.kind, self.value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RankType {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    Straight = 5,
    Flush = 6,
    FullHouse = 7,
    FourOfAKind = 8,
    StraightFlush = 9,
    RoyalFlush = 10,
}

#[derive(Clone, PartialEq, Eq)]
struct Rank {
    rank_type: RankType,
    card: Card,
}

impl Rank {
    fn new(rank_type: RankType, card: &Card) -> Self {
        Rank { rank_type, card: *card }
    }
}

impl std::fmt::Debug for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({:?})", self.rank_type, self.card)
    }
}

impl std::cmp::PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let rank_cmp = self.rank_type.cmp(&other.rank_type);
        if rank_cmp != std::cmp::Ordering::Equal {
            return rank_cmp;
        }

        self.card.value.cmp(&other.card.value)
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn from(cards: &[Card]) -> Self {
        let mut card_list = cards.to_vec();
        card_list.sort_by_key(|c| c.value);
        Hand { cards: card_list }
    }

    fn is_flush(&self) -> Option<Rank> {
        let kind = &self.cards[0].kind;
        if self.cards.iter().all(|c| &c.kind == kind) {
            Some(Rank::new(RankType::Flush, &self.cards[4]))
        } else {
            None
        }
    }

    fn is_straight(&self) -> Option<Rank> {
        let base = self.cards[0].value as i64;
        for i in 1..5 {
            if self.cards[i].value as i64 != base + i as i64 {
                return None;
            }
        }
        Some(Rank::new(RankType::Straight, &self.cards[4]))
    }

    fn is_straight_or_flush(&self) -> Option<Rank> {
        let is_flush = self.is_flush();
        let is_straight = self.is_straight();

        match (is_flush, is_straight) {
            (Some(flush_rank), Some(straight_rank)) => {
                if straight_rank.card.value == Value::Ace {
                    Some(Rank::new(RankType::RoyalFlush, &flush_rank.card))
                } else {
                    Some(Rank::new(RankType::StraightFlush, &straight_rank.card))
                }
            },
            (_, Some(straight_rank)) => {
                Some(straight_rank)
            },
            (Some(flush_rank), _) => {
                Some(flush_rank)
            },
            _ => None,
        }
    }

    fn get_rank(&self) -> Vec<Rank> {
        if let Some(r) = self.is_straight_or_flush() {
            return vec![r];
        };

        let mut result = Vec::new();
        let mut value_map = [(0, None); 15];
        for card in &self.cards {
            let value = card.value as usize;
            let current = value_map[value];
            value_map[value] = (current.0 + 1, Some(card));
        }

        let mut pairs = 0;
        for (count, card) in value_map.iter() {
            let item = match *count {
                4 => Rank::new(RankType::FourOfAKind, card.unwrap()),
                3 => Rank::new(RankType::ThreeOfAKind, card.unwrap()),
                2 => {
                    pairs += 1;
                    Rank::new(RankType::OnePair, card.unwrap())
                },
                1 => Rank::new(RankType::HighCard, card.unwrap()),
                _ => continue,
            };
            result.push(item);
        }

        result.sort();
        result.reverse();

        if result.len() == 2 {
            // fix full house
            if result[0].rank_type == RankType::ThreeOfAKind && result[1].rank_type == RankType::OnePair {
                let card = result[1].card;
                result = vec![Rank::new(RankType::FullHouse, &card)]
            }

        } else if pairs == 2 {
            // fix two pairs
            let card0 = result[0].card;
            let card1 = result[1].card;
            result = vec![
                Rank::new(RankType::TwoPairs, &card0),
                Rank::new(RankType::TwoPairs, &card1)
            ];
        }

        result
    }
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.cards.iter()
            .map(|c| format!("{:?}", c))
            .collect::<Vec<_>>().join(" ");
        write!(f, "{}", s)
    }
}

fn check_hands(hand1: &Hand, hand2: &Hand) -> bool {
    let rank1 = hand1.get_rank();
    let rank2 = hand2.get_rank();

    let mut result = false;
    let mut i = 0;

    while i < rank1.len() && i < rank2.len() {
        let item1 = &rank1[i];
        let item2 = &rank2[i];

        if item1 != item2 {
            result = item1 > item2;
            break;
        }

        i += 1;
    }

    
    result
}

pub fn solve() -> i64 {
    let raw = load();
    let mut count = 0;

    for line in raw {
        let cards: Vec<Card> = line.iter()
            .map(|s| Card::from(s))
            .collect();

        let hand1 = Hand::from(&cards[0..5]);
        let hand2 = Hand::from(&cards[5..10]);

        if check_hands(&hand1, &hand2) {
            count += 1;
        }
    }

    count
}
