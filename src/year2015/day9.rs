use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn solve(input: &str, longest: bool) -> usize {
    let mut edges = HashMap::new();
    let mut nodes = HashSet::new();

    for line in input.lines() {
        let (edge, mut dist) = line.split_once("=").unwrap();
        let (mut from, mut to) = edge.split_once("to").unwrap();

        from = from.trim();
        to = to.trim();
        dist = dist.trim();

        nodes.insert(from);
        nodes.insert(to);

        let entry = edges.entry(from).or_insert(vec![]);
        entry.push((to, dist.parse::<usize>().unwrap()));
        let entry = edges.entry(to).or_insert(vec![]);
        entry.push((from, dist.parse::<usize>().unwrap()));
    }

    let node_count = nodes.len();

    let distances = nodes
        .into_iter()
        .collect_vec()
        .into_iter()
        .permutations(node_count)
        .map(|permutation| {
            permutation
                .windows(2)
                .map(|window| {
                    edges[window[0]]
                        .iter()
                        .find(|(edge, _dist)| *edge == window[1])
                        .unwrap()
                        .1
                })
                .sum::<usize>()
        });

    if longest {
        distances.max().unwrap()
    } else {
        distances.min().unwrap()
    }
}

pub fn solve_part_1(input: &str) -> usize {
    solve(input, false)
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day9/test_input")), 605);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day9/input")), 117);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day9/test_input")), 982);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day9/input")), 909);
    }
}
