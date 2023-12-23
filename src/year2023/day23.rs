use std::collections::{BinaryHeap, HashMap};

use crate::utils::{
    grid::Grid,
    point::{Point, EAST, NORTH, SOUTH, WEST},
};

const PATH: char = '.';
const FOREST: char = '#';
const SLOPE_NORTH: char = '^';
const SLOPE_EAST: char = '>';
const SLOPE_SOUTH: char = 'v';
const SLOPE_WEST: char = '<';

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    position: Point,
    direction: Point,
    distance: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

fn longest_dijkstra(grid: Grid<char>, start: Point, end: Point) -> usize {
    let mut results = vec![];
    let mut queue = BinaryHeap::new();

    queue.push(Node {
        position: start,
        direction: start,
        distance: 0,
    });

    while let Some(current) = queue.pop() {
        println!("Popped {:?}", current);

        if current.position == end {
            results.push(current.distance);
            continue;
        }

        for neighbour_direction in vec![EAST, SOUTH, WEST, NORTH] {
            let forbidden_direction = match current.direction {
                EAST => WEST,
                SOUTH => NORTH,
                WEST => EAST,
                NORTH => SOUTH,
                _ => panic!("Invalid direction"),
            };

            if neighbour_direction == forbidden_direction {
                continue;
            }

            let neighbour_position = current.position + neighbour_direction;

            if let Some(value) = grid.get_value(&neighbour_position) {
                let forbidden_val = match neighbour_direction {
                    EAST => SLOPE_WEST,
                    SOUTH => SLOPE_NORTH,
                    WEST => SLOPE_EAST,
                    NORTH => SLOPE_SOUTH,
                    _ => panic!("Invalid direction"),
                };

                if value == FOREST || value == forbidden_val {
                    continue;
                }

                let neighbour_node = Node {
                    position: neighbour_position,
                    direction: neighbour_direction,
                    distance: current.distance + 1,
                };

                println!("\tPushing {:?}", neighbour_node);
                queue.push(neighbour_node);
            }
        }
    }

    dbg!(&results);
    *results.iter().max().unwrap()
}

pub fn solve_part_1(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();
    dbg!(&grid);

    let start = grid.find_in_row(0, &PATH).unwrap();
    dbg!(&start);

    let end = grid.find_in_row(grid.x_len - 1, &PATH).unwrap();
    dbg!(&end);

    longest_dijkstra(grid, start, end)
}

pub fn solve_part_2(_: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day23/test_input")), 94);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day23/input")), 2_194);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day23/test_input")), 1);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day23/input")), 1);
    }
}
