use std::{cmp::Ordering, collections::HashMap};

use std::str::FromStr;

use crate::utils::ParseInputError;

#[derive(Debug, Eq)]
pub struct Hand {
    pub cards: Vec<char>,
    pub bid: usize,
}

impl Hand {
    fn card_groupings(&self) -> HashMap<char, usize> {
        let mut card_groupings: HashMap<char, usize> = HashMap::new();

        for card in self.cards.iter() {
            card_groupings.entry(*card).and_modify(|count| *count += 1).or_insert(1);
        }

        card_groupings
    }

    fn hand_type(&self) -> HandType {
        let card_groupings = self.card_groupings();

        let card_groupings_without_jokers: HashMap<char, usize> = self
            .card_groupings()
            .into_iter()
            .filter(|(rank, _)| *rank != '0')
            .collect();

        let mut hands: Vec<&usize> = card_groupings.values().collect();
        hands.sort();

        let hands_without_jokers = card_groupings_without_jokers.values().collect::<Vec<&usize>>();

        let max_hand_without_jokers: usize = *card_groupings_without_jokers.values().max().unwrap_or(&&0);

        let jokers = *card_groupings.get(&'0').unwrap_or(&0);

        if max_hand_without_jokers + jokers == 5 {
            return HandType::FiveOfAKind;
        }

        if max_hand_without_jokers + jokers == 4 {
            return HandType::FourOfAKind;
        }

        if (hands == vec![&2, &3]) || (hands_without_jokers == vec![&2, &2] && jokers == 1) {
            return HandType::FullHouse;
        }

        if max_hand_without_jokers + jokers == 3 {
            return HandType::ThreeOfAKind;
        }

        if hands == vec![&1, &2, &2] {
            return HandType::TwoPair;
        }

        if max_hand_without_jokers + jokers == 2 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

impl FromStr for Hand {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(" ").unwrap();

        Ok(Hand {
            cards: cards_str.chars().collect(),
            bid: bid_str.parse().unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type()
            .cmp(&other.hand_type())
            .then(self.cards.cmp(&other.cards))
    }
}
