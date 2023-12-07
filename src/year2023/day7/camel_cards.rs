use std::{cmp::Ordering, collections::HashMap};

use crate::{
    utils::{self, deck::card::Rank},
    year2023::day7::hand::card_strength,
};

use super::hand::{Hand, HandType};

pub fn cmp(hand_a: &Hand, hand_b: &Hand) -> Ordering {
    let hand_type_a = hand_type(hand_a);
    // dbg!(&hand_a);
    // dbg!(&hand_type_a);
    let hand_type_b = hand_type(hand_b);
    // dbg!(&hand_b);
    // dbg!(&hand_type_b);

    let hands = hand_type_a.cmp(&hand_type_b);
    if hands != Ordering::Equal {
        return hands;
    }

    for (i, card_a) in hand_a.cards.iter().enumerate() {
        let card_b = hand_b.cards.get(i).unwrap();
        // println!("Comparing {:?} with {:?}", card_a, card_b);
        let strenghts = card_strength(card_a).cmp(&card_strength(card_b));
        if strenghts != Ordering::Equal {
            return strenghts;
        } else {
        }
    }

    Ordering::Equal
}

pub fn hand_type(hand: &Hand) -> HandType {
    let card_groupings = hand.card_groupings();
    dbg!(&card_groupings);

    let card_groupings_without_jokers: HashMap<Rank, usize> = hand
        .card_groupings()
        .into_iter()
        .filter(|(rank, _)| *rank != utils::deck::card::Rank::JOKER)
        .collect();
    dbg!(&card_groupings_without_jokers);

    let mut hands: Vec<&usize> = card_groupings.values().collect();
    hands.sort();
    dbg!(&hands);

    let hands_without_jokers = card_groupings_without_jokers.values().collect::<Vec<&usize>>();
    dbg!(&hands_without_jokers);

    let max_hand_without_jokers: usize = *card_groupings_without_jokers.values().max().unwrap_or(&&0);
    dbg!(&max_hand_without_jokers);

    let jokers = *card_groupings.get(&utils::deck::card::Rank::JOKER).unwrap_or(&0);
    dbg!(&jokers);

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

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::year2023::day7::{
        camel_cards,
        hand::{Hand, HandType},
    };
    #[test]
    fn cmp() {
        let hand_a = "1 T3Q33 11".parse::<Hand>().unwrap();
        let hand_b = "1 T3T3X 17".parse::<Hand>().unwrap();
        assert_eq!(super::cmp(&hand_a, &hand_b), Ordering::Less);
    }

    #[test]
    fn hand_types() {
        let hand = "1 T3Q33 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FullHouse);
        let hand = "1 T3T3X 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::ThreeOfAKind);
        let hand = "1 AAAAA 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FiveOfAKind);
        let hand = "1 AAAAX 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FiveOfAKind);
        let hand = "1 AAAXX 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FiveOfAKind);
        let hand = "1 AAXXX 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FiveOfAKind);
        let hand = "1 AXXXX 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FiveOfAKind);
        let hand = "1 XXXXX 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FiveOfAKind);
        let hand = "1 AAAA2 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FourOfAKind);
        let hand = "1 AAAX2 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FourOfAKind);
        let hand = "1 AAXX2 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FourOfAKind);
        let hand = "1 AXXX2 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FourOfAKind);
        let hand = "1 AXXX2 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FourOfAKind);
        let hand = "1 AAA22 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::FullHouse);
        let hand = "1 AAA23 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::ThreeOfAKind);
        let hand = "1 AA23X 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::ThreeOfAKind);
        let hand = "1 AA223 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::TwoPair);
        let hand = "1 A234X 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::OnePair);
        let hand = "1 A2345 123".parse::<Hand>().unwrap();
        assert_eq!(camel_cards::hand_type(&hand), HandType::HighCard);
    }
}
