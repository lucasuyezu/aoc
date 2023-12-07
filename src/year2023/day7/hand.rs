use std::{collections::HashMap, str::FromStr};

use crate::utils::{
    self,
    deck::card::{Card, Rank},
    ParseInputError,
};

#[derive(Debug)]
pub struct Hand {
    pub id: usize,
    pub cards: Vec<Card>,
    pub bid: usize,
}

impl Hand {
    pub fn card_groupings(&self) -> HashMap<Rank, usize> {
        let mut card_groupings: HashMap<Rank, usize> = HashMap::new();

        for card in self.cards.iter() {
            card_groupings
                .entry(card.rank)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        card_groupings
    }
}

impl FromStr for Hand {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" ");

        let id: usize = split.next().unwrap().parse().unwrap();

        let cards: Vec<Card> = split
            .next()
            .unwrap()
            .chars()
            .map(|c| Card {
                suit: crate::utils::deck::card::Suit::CLUBS,
                rank: match c {
                    'X' => crate::utils::deck::card::Rank::JOKER,
                    '2' => crate::utils::deck::card::Rank::TWO,
                    '3' => crate::utils::deck::card::Rank::THREE,
                    '4' => crate::utils::deck::card::Rank::FOUR,
                    '5' => crate::utils::deck::card::Rank::FIVE,
                    '6' => crate::utils::deck::card::Rank::SIX,
                    '7' => crate::utils::deck::card::Rank::SEVEN,
                    '8' => crate::utils::deck::card::Rank::EIGHT,
                    '9' => crate::utils::deck::card::Rank::NINE,
                    'T' => crate::utils::deck::card::Rank::TEN,
                    'J' => crate::utils::deck::card::Rank::JACK,
                    'Q' => crate::utils::deck::card::Rank::QUEEN,
                    'K' => crate::utils::deck::card::Rank::KING,
                    'A' => crate::utils::deck::card::Rank::ACE,
                    x => panic!("Invalid card {}", x),
                },
            })
            .collect();

        let bid: usize = split.next().unwrap().parse().unwrap();

        Ok(Hand { id, cards, bid })
    }
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

pub fn card_strength(card: &Card) -> usize {
    match &card.rank {
        utils::deck::card::Rank::JOKER => 1,
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
