use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::Regex;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).").unwrap();
}

const SELF: &str = "You";

pub fn solve(input: &str, include_yourself: bool) -> isize {
    let mut hash: HashMap<(&str, &str), isize> = HashMap::new();
    let mut names = HashSet::new();

    if include_yourself {
        names.insert(SELF);
    }

    for line in input.lines() {
        if RE.is_match(line) {
            let captures = RE.captures(line).unwrap();
            let from = captures.get(1).unwrap().as_str();
            let score_str = captures.get(3).unwrap().as_str();
            let score = score_str.parse::<isize>().unwrap();
            let to = captures.get(4).unwrap().as_str();

            names.insert(from);
            names.insert(to);

            if &captures[2] == "gain" {
                hash.insert((from, to), score);
            } else {
                hash.insert((from, to), -score);
            }
        }
    }

    names
        .iter()
        .permutations(names.len())
        .map(|permutation| {
            (0..permutation.len())
                .map(|i| {
                    let left_idx = if i == 0 { permutation.len() - 1 } else { i - 1 };
                    let right_idx = if i == permutation.len() - 1 { 0 } else { i + 1 };

                    let left_neighbour = permutation[left_idx];
                    let guest = permutation[i];
                    let right_neighbour = permutation[right_idx];

                    let left_score = if guest == &SELF || left_neighbour == &SELF {
                        0
                    } else {
                        hash[&(*guest, *left_neighbour)]
                    };

                    let right_score = if guest == &SELF || right_neighbour == &SELF {
                        0
                    } else {
                        hash[&(*guest, *right_neighbour)]
                    };

                    left_score + right_score
                })
                .sum()
        })
        .max()
        .unwrap()
}

pub fn solve_part_1(input: &str) -> isize {
    solve(input, false)
}

pub fn solve_part_2(input: &str) -> isize {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day13/test_input")), 330);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day13/input")), 733);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day13/test_input")), 286);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day13/input")), 725);
    }
}
