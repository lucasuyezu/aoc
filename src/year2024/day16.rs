use itertools::Itertools;
use pathfinding::prelude::dijkstra;

use crate::utils::{
    grid::Grid,
    point::{Point, EAST, NORTH, SOUTH, WEST},
};

#[derive(Clone, Hash, PartialEq, Eq)]
struct Pos {
    point: Point,
    direction: isize,
}

const DIRECTIONS: [Point; 4] = [NORTH, EAST, SOUTH, WEST];

pub fn solve_part_1(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();
    let start_point = grid.get_pos(&'S').unwrap();
    let end_point = grid.get_pos(&'E').unwrap();

    let start_pos = Pos {
        point: start_point,
        direction: 1,
    };

    let result = dijkstra(
        &start_pos,
        |p| {
            [
                (p.direction, 1),
                ((p.direction + 1).rem_euclid(DIRECTIONS.len() as isize), 1001),
                ((p.direction - 1).rem_euclid(DIRECTIONS.len() as isize), 1001),
            ]
            .iter()
            .filter_map(|(dir, cost)| {
                let n = p.point + DIRECTIONS[*dir as usize];
                match grid.get_value(&n) {
                    Some('.') | Some('E') => Some((
                        Pos {
                            point: n,
                            direction: dir.clone(),
                        },
                        *cost,
                    )),
                    _ => None,
                }
            })
            .collect_vec()
        },
        |p| p.point == end_point,
    );

    result.unwrap().1
}

pub fn solve_part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input_small() {
        assert_eq!(super::solve_part_1(&include_str!("day16/test_input_small")), 7036);
    }

    #[test]
    fn part1_test_input_large() {
        assert_eq!(super::solve_part_1(&include_str!("day16/test_input")), 11048);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day16/input")), 75416);
    }

    #[test]
    fn part2_test_input_small() {
        assert_eq!(super::solve_part_2(&include_str!("day16/test_input_small")), 45);
    }

    #[test]
    fn part2_test_input_large() {
        assert_eq!(super::solve_part_2(&include_str!("day16/test_input")), 64);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day16/input")), 26859182);
    }
}
