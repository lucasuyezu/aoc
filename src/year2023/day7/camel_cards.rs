use std::{cmp::Ordering, collections::HashMap};

use crate::utils::{
    self,
    deck::card::{Card, Rank},
};

#[derive(Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bid: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

fn card_strength(card: &Card) -> usize {
    match &card.rank {
        utils::deck::card::Rank::TWO => 2,
        utils::deck::card::Rank::THREE => 3,
        utils::deck::card::Rank::FOUR => 4,
        utils::deck::card::Rank::FIVE => 5,
        utils::deck::card::Rank::SIX => 6,
        utils::deck::card::Rank::SEVEN => 7,
        utils::deck::card::Rank::EIGHT => 8,
        utils::deck::card::Rank::NINE => 9,
        utils::deck::card::Rank::TEN => 10,
        utils::deck::card::Rank::JACK => 11,
        utils::deck::card::Rank::QUEEN => 12,
        utils::deck::card::Rank::KING => 13,
        utils::deck::card::Rank::ACE => 14,
    }
}

impl Hand {
    pub fn hand_type(&self) -> HandType {
        let mut card_groups: HashMap<&Rank, usize> = HashMap::new();

        for card in self.cards.iter() {
            card_groups
                .entry(&card.rank)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut hands: Vec<&usize> = card_groups.values().collect();
        hands.sort();

        let max_hand = hands.iter().max().unwrap();

        if **max_hand == 5 {
            return HandType::FiveOfAKind;
        }

        if **max_hand == 4 {
            return HandType::FourOfAKind;
        }

        if hands == vec![&2, &3] {
            return HandType::FullHouse;
        }

        if **max_hand == 3 {
            return HandType::ThreeOfAKind;
        }

        if hands == vec![&1, &2, &2] {
            return HandType::TwoPair;
        }

        if **max_hand == 2 {
            return HandType::OnePair;
        }

        HandType::HighCard
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type().eq(&other.hand_type())
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_types = self.hand_type().cmp(&other.hand_type());
        if hand_types != Ordering::Equal {
            return Some(hand_types);
        }

        for (i, card) in self.cards.iter().enumerate() {
            let strenghts = card_strength(card).cmp(&card_strength(&other.cards.get(i).unwrap()));
            if strenghts != Ordering::Equal {
                return Some(strenghts);
            }
        }

        Some(Ordering::Equal)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
