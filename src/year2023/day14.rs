use itertools::Itertools;

use crate::utils::print_2d_vec;

type Grid = Vec<Vec<char>>;

const ROUNDED_ROCK: char = 'O';
const EMPTY_SPACE: char = '.';

fn parse_input(input: &str) -> Option<Grid> {
    Some(
        input
            .lines()
            .map(|table_line| table_line.chars().collect_vec())
            .collect_vec(),
    )
}

fn tilt(mut grid: Grid) -> Option<Grid> {
    print_2d_vec(&grid);

    let x_len = grid.len();
    let y_len = grid[0].len();

    // for each column
    for y in 0..y_len {
        for x in 0..x_len {
            // for each ROUNDED_ROCK
            if grid[x][y] == ROUNDED_ROCK {
                let mut xx = x;
                // Keep bumping it up until you reach the top OR anything other than an EMPTY_SPACE
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

fn calculate_total_load(grid: Grid) -> Option<usize> {
    print_2d_vec(&grid);
    let x_len = grid.len();

    Some(
        grid.iter()
            .enumerate()
            .map(|(i, row)| row.iter().filter(|c| **c == ROUNDED_ROCK).count() * (x_len - i))
            .sum(),
    )
}

pub fn solve_part_1(input: &str) -> usize {
    parse_input(input)
        .and_then(tilt)
        .and_then(calculate_total_load)
        .unwrap()
}

pub fn solve_part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day14/test_input")), 136);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day14/input")), 30_575);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day14/test_input")), 400);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day14/input")), 37_478);
    }
}
