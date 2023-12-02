use std::{str::FromStr, string::ParseError};

use regex::Regex;

#[derive(Debug)]
pub struct GameSet {
    r_count: Option<usize>,
    g_count: Option<usize>,
    b_count: Option<usize>,
}
impl GameSet {
    fn is_possible(&self, max_r: usize, max_g: usize, max_b: usize) -> bool {
        (self.r_count.is_none() || self.r_count.is_some() && self.r_count.unwrap() <= max_r)
            && (self.g_count.is_none() || self.g_count.is_some() && self.g_count.unwrap() <= max_g)
            && (self.b_count.is_none() || self.b_count.is_some() && self.b_count.unwrap() <= max_b)
    }
}

impl FromStr for GameSet {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref R_RE: Regex = Regex::new(r"\d+ red").unwrap();
            static ref G_RE: Regex = Regex::new(r"\d+ green").unwrap();
            static ref B_RE: Regex = Regex::new(r"\d+ blue").unwrap();
        }

        let mut game_set = GameSet {
            r_count: None,
            g_count: None,
            b_count: None,
        };

        if let Some(str_match) = R_RE.find(s) {
            game_set.r_count = str_match.as_str().split(" ").next().unwrap().parse().ok();
        }

        if let Some(str_match) = G_RE.find(s) {
            game_set.g_count = str_match.as_str().split(" ").next().unwrap().parse().ok();
        }

        if let Some(str_match) = B_RE.find(s) {
            game_set.b_count = str_match.as_str().split(" ").next().unwrap().parse().ok();
        }

        return Ok(game_set);
    }
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

impl Game {
    fn is_possible(&self, max_r: usize, max_g: usize, max_b: usize) -> bool {
        return self.sets.iter().all(|set| set.is_possible(max_r, max_g, max_b));
    }

    fn power_level(&self) -> usize {
        let max_r = self.sets.iter().filter_map(|set| set.r_count).max().unwrap();
        let max_g = self.sets.iter().filter_map(|set| set.g_count).max().unwrap();
        let max_b = self.sets.iter().filter_map(|set| set.b_count).max().unwrap();

        return max_r * max_g * max_b;
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = Game { id: 0, sets: vec![] };

        let lines: Vec<&str> = s.split(":").collect();

        game.id = lines[0].split(" ").last().unwrap().parse().unwrap();
        game.sets = lines[1]
            .split(";")
            .into_iter()
            .map(|set_str| set_str.parse::<GameSet>().unwrap())
            .collect();

        return Ok(game);
    }
}

pub fn solve_part_1(input: &str) -> usize {
    return input
        .lines()
        .into_iter()
        .map(|line| line.parse::<Game>().unwrap())
        .filter(|game| game.is_possible(12, 13, 14))
        .map(|game| game.id)
        .sum();
}

pub fn solve_part_2(input: &str) -> usize {
    return input
        .lines()
        .into_iter()
        .map(|line| line.parse::<Game>().unwrap())
        .map(|game| game.power_level())
        .sum();
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
