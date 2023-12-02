use std::str::FromStr;

use regex::{Match, Regex};

use crate::utils::ParseInputError;

#[derive(Debug)]
pub struct GameSet {
    r_count: usize,
    g_count: usize,
    b_count: usize,
}
impl GameSet {
    fn is_possible(&self, max_r: usize, max_g: usize, max_b: usize) -> bool {
        self.r_count <= max_r && self.g_count <= max_g && self.b_count <= max_b
    }
}

impl FromStr for GameSet {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref R_RE: Regex = Regex::new(r"\d+ red").unwrap();
            static ref G_RE: Regex = Regex::new(r"\d+ green").unwrap();
            static ref B_RE: Regex = Regex::new(r"\d+ blue").unwrap();
        }

        let mut game_set = GameSet {
            r_count: 0,
            g_count: 0,
            b_count: 0,
        };

        if R_RE.is_match(s) {
            game_set.r_count = str_to_count(R_RE.find(s))?;
        }

        if G_RE.is_match(s) {
            game_set.g_count = str_to_count(G_RE.find(s))?;
        }

        if B_RE.is_match(s) {
            game_set.b_count = str_to_count(B_RE.find(s))?;
        }

        Ok(game_set)
    }
}

fn str_to_count(re_match: Option<Match>) -> Result<usize, ParseInputError> {
    re_match
        .and_then(|s| Some(s.as_str()))
        .and_then(|s| s.split(" ").next())
        .and_then(|s| s.parse::<usize>().ok())
        .ok_or(ParseInputError)
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

impl Game {
    fn is_possible(&self, max_r: usize, max_g: usize, max_b: usize) -> bool {
        self.sets.iter().all(|set| set.is_possible(max_r, max_g, max_b))
    }

    fn power_level(&self) -> usize {
        let max_r = self.sets.iter().map(|set| set.r_count).max().unwrap();
        let max_g = self.sets.iter().map(|set| set.g_count).max().unwrap();
        let max_b = self.sets.iter().map(|set| set.b_count).max().unwrap();

        max_r * max_g * max_b
    }
}

impl FromStr for Game {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game { id: 0, sets: vec![] };

        let split = s.split_once(":").ok_or(ParseInputError)?;

        game.id = split
            .0
            .split_once(" ")
            .and_then(|s| s.1.parse::<usize>().ok())
            .ok_or(ParseInputError)?;

        game.sets = split
            .1
            .split(";")
            .map(|set_str| set_str.parse::<GameSet>().unwrap())
            .collect();

        Ok(game)
    }
}

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .filter(|game| game.is_possible(12, 13, 14))
        .map(|game| game.id)
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .map(|game| game.power_level())
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 8);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 2685);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 2_286);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 83_707);
    }
}
