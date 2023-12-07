use crate::utils::deck::card::Card;

use self::camel_cards::Hand;

pub mod camel_cards;

fn parse_part_1_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards_str, bid_str) = line.split_once(" ").unwrap();

            let cards: Vec<Card> = cards_str
                .chars()
                .map(|c| Card {
                    suit: crate::utils::deck::card::Suit::CLUBS,
                    rank: match c {
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

            let bid: usize = bid_str.parse().unwrap();

            Hand { cards, bid }
        })
        .collect()
}

pub fn solve_part_1(input: &str) -> usize {
    let mut hands = parse_part_1_input(input);

    // Now, you can determine the total winnings of this set of **hands**
    // by adding up the result of multiplying each hand's **bid** with its **rank**
    // (765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5).
    // So the total winnings in this example are 6440.
    hands.sort();

    hands.iter().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum()
}

fn parse_part_2_input(input: &str) -> Vec<Hand> {
    vec![]
}

pub fn solve_part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 6_440);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 251_029_473);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 71_503);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 34_655_848);
    }
}
