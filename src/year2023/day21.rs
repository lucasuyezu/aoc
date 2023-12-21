use std::collections::{HashSet, VecDeque};

use crate::utils::{
    parse_input_into_char_grid,
    point::{Point, EAST, NORTH, SOUTH, WEST},
    Grid,
};

const START: char = 'S';
const GARDEN_PLOT: char = '.';
// const ROCK: char = '#';

fn walk(grid: &Grid<char>, max_steps: usize, infinity: bool) -> usize {
    // dbg!(&max_steps);
    let mut results: HashSet<Point> = HashSet::new();
    let mut visited: HashSet<(Point, usize)> = HashSet::new();
    let start = grid.get_pos(&START).unwrap();

    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();

    visited.insert((start, 0));
    queue.push_back((start, 0));

    while let Some((pos, step)) = queue.pop_front() {
        // println!("Popped {:?} step {}", pos, step);

        if step == max_steps {
            results.insert(pos);
            continue;
        }

        for dir in vec![NORTH, EAST, SOUTH, WEST] {
            let n_pos = pos + dir;
            let n_node = (n_pos, step + 1);

            if visited.contains(&n_node) {
                continue;
            }

            visited.insert(n_node);

            if let Some(cell) = grid.get_value(&n_pos) {
                if cell == GARDEN_PLOT || cell == START {
                    // println!("\tPushing {:?}{}", neighbour, step + 1);
                    queue.push_back(n_node);
                }
            }
        }
    }

    // dbg!(&start);
    results.len()
}

pub fn solve_part_1(input: &str) -> usize {
    let grid = parse_input_into_char_grid(input);
    walk(&grid, 64, false)
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = parse_input_into_char_grid(input);
    walk(&grid, 64, true)
}

#[cfg(test)]
mod tests {
    use crate::utils::parse_input_into_char_grid;

    #[test]
    fn part1_test_input() {
        let input = &include_str!("day21/test_input");
        let grid = parse_input_into_char_grid(input);

        for (steps, result) in vec![(1, 2), (2, 4), (3, 6), (6, 16)] {
            assert_eq!(super::walk(&grid, steps, false), result);
        }
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day21/input")), 3_682);
    }

    #[test]
    fn part2_test_input() {
        let input = &include_str!("day21/test_input");
        let grid = parse_input_into_char_grid(input);

        for (steps, result) in vec![
            (6, 16),
            (10, 50),
            (50, 1_594),
            (100, 6_536),
            (500, 167_004),
            (1_000, 668_697),
            (5_000, 16_733_044),
        ] {
            assert_eq!(super::walk(&grid, steps, true), result);
        }
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day21/input")), 1);
    }
}
