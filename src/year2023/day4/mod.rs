use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::utils::ParseInputError;

#[derive(Debug)]
pub struct Card {
    id: usize,
    win_numbers: HashSet<usize>,
    hand_numbers: HashSet<usize>,
}
impl Card {
    fn matching_numbers_count(&self) -> usize {
        self.win_numbers.intersection(&self.hand_numbers).count()
    }
    fn score(&self) -> usize {
        let count = self.matching_numbers_count();
        if count == 0 {
            return 0;
        }

        (2 as usize).pow((count as u32) - 1)
    }
}

impl FromStr for Card {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_id_str, numbers_str) = s.split_once(":").unwrap();

        let id: usize = card_id_str.split_once(" ").unwrap().1.trim().parse().unwrap();

        let (win_numbers_str, hand_numbers_str) = numbers_str.split_once("|").unwrap();

        let mut win_numbers: HashSet<usize> = HashSet::new();
        for s in win_numbers_str.trim().split(" ") {
            if s != "" {
                win_numbers.insert(s.parse::<usize>().unwrap());
            }
        }

        let mut hand_numbers: HashSet<usize> = HashSet::new();
        for s in hand_numbers_str.trim().split(" ") {
            if s != "" {
                hand_numbers.insert(s.parse::<usize>().unwrap());
            }
        }

        Ok(Card {
            id,
            win_numbers,
            hand_numbers,
        })
    }
}

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .map(|g| g.score())
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let cards: Vec<Card> = input.lines().map(|line| line.parse::<Card>().unwrap()).collect();

    let mut cards_count_map: HashMap<usize, usize> = HashMap::new();

    for card in cards.iter() {
        cards_count_map.insert(card.id, 1);
    }

    for card in cards.iter() {
        for i in (card.id + 1)..(card.id + 1 + card.matching_numbers_count()) {
            let src_card_count = cards_count_map.get(&card.id).unwrap();
            let dst_card_count = cards_count_map.get(&i).unwrap();
            cards_count_map.insert(i, src_card_count + dst_card_count);
        }
    }

    cards_count_map.values().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 13);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 20_107);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 30);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 8_172_507);
    }
}
