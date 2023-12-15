use crate::utils::{calculate_hash, rotate};
use itertools::Itertools;
use std::collections::HashMap;

type Grid = Vec<Vec<char>>;

const ROUNDED_ROCK: char = 'O';
const EMPTY_SPACE: char = '.';
const CYCLES: usize = 1_000_000_000;

fn parse_input(input: &str) -> Option<Grid> {
    Some(
        input
            .lines()
            .map(|table_line| table_line.chars().collect_vec())
            .collect_vec(),
    )
}

fn tilt_north(mut grid: Grid) -> Option<Grid> {
    let x_len = grid.len();
    let y_len = grid[0].len();

    for y in 0..y_len {
        for x in 0..x_len {
            if grid[x][y] == ROUNDED_ROCK {
                let mut xx = x;
                while xx > 0 && grid[xx - 1][y] == EMPTY_SPACE {
                    grid[xx - 1][y] = ROUNDED_ROCK;
                    grid[xx][y] = EMPTY_SPACE;
                    xx -= 1;
                }
            }
        }
    }

    Some(grid)
}

fn cycle(grid: Grid) -> Option<Grid> {
    Some(grid)
        .and_then(tilt_north)
        .and_then(rotate)
        .and_then(tilt_north)
        .and_then(rotate)
        .and_then(tilt_north)
        .and_then(rotate)
        .and_then(tilt_north)
        .and_then(rotate)
}

fn find_cycle(mut grid: Grid) -> Option<Grid> {
    let iterations = 1_000;
    let mut idx_hash: HashMap<u64, usize> = HashMap::new();
    let mut grid_hash: HashMap<usize, Grid> = HashMap::new();

    for i in 0..iterations {
        let idx_hash_key = calculate_hash(&grid);

        if let Some(cycle_start) = idx_hash.get(&idx_hash_key) {
            let cycle_size = i - cycle_start;
            let grid_hash_key = CYCLES - ((CYCLES - cycle_start) / cycle_size) * cycle_size;

            return Some(grid_hash.get(&grid_hash_key).unwrap().clone());
        } else {
            grid_hash.insert(i, grid.clone());
            grid = cycle(grid).unwrap();
            idx_hash.insert(idx_hash_key, i);
        }
    }
    panic!("Did not find a cycle in {} iterations", iterations);
}

fn calculate_total_load(grid: Grid) -> Option<usize> {
    Some(
        grid.iter()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|c| **c == ROUNDED_ROCK).count() * (grid.len() - i))
            .sum(),
    )
}

pub fn solve_part_1(input: &str) -> usize {
    parse_input(input)
        .and_then(tilt_north)
        .and_then(calculate_total_load)
        .unwrap()
}

pub fn solve_part_2(input: &str) -> usize {
    parse_input(input)
        .and_then(find_cycle)
        .and_then(calculate_total_load)
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day14/test_input")), 136);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day14/input")), 107_951);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day14/test_input")), 64);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day14/input")), 95_736);
    }
}
