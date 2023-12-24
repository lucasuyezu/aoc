use std::collections::{HashSet, VecDeque};

use crate::utils::graph::Graph;
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

fn longest_dijkstra(graph: &Graph, start: String, end: String) -> usize {
    let mut result = usize::MIN;
    let mut queue = VecDeque::new();

    queue.push_back((start, 0, HashSet::new()));

    let mut counter = 0;

    while let Some(cur_node) = queue.pop_front() {
        counter += 1;
        let (cur_pos, cur_dist, mut cur_visited) = cur_node.clone();

        if counter % 100_000 == 0 {
            println!("{}", counter);
        }

        if cur_pos == end && cur_dist > result {
            println!("Possible result: {}", cur_dist);
            result = cur_dist;
            continue;
        }

        if cur_visited.contains(&cur_pos) {
            continue;
        }

        cur_visited.insert(cur_pos.clone());

        for (n_node, n_dist) in graph.neighbours(cur_pos) {
            let neighbour_node = (n_node, cur_dist + n_dist, cur_visited.clone());
            queue.push_back(neighbour_node);
        }
    }

    result
}

fn parse_graph(grid: &Grid<char>, ignore_slopes: bool) -> Graph {
    let mut graph = Graph::new();

    for (x, row) in grid.data.iter().enumerate() {
        for (y, value) in row.iter().enumerate() {
            let pos = Point {
                x: x as isize,
                y: y as isize,
            };

            if *value == FOREST {
                continue;
            }

            println!("In {:?}", pos);
            for n_dir in neighbours(value, ignore_slopes) {
                let n_pos = pos + n_dir;
                match grid.get_value(&n_pos) {
                    None => {}
                    Some(n_value) => {
                        let forbidden_val = match n_dir {
                            EAST => SLOPE_WEST,
                            SOUTH => SLOPE_NORTH,
                            WEST => SLOPE_EAST,
                            NORTH => SLOPE_SOUTH,
                            _ => panic!("Invalid direction"),
                        };

                        if n_value != FOREST && (ignore_slopes || n_value != forbidden_val) {
                            println!("\tAdding neighbour {:?}", n_pos);
                            graph.insert_edge_directed(
                                format!("{}-{}", pos.x, pos.y),
                                format!("{}-{}", n_pos.x, n_pos.y),
                                1,
                            );
                        }
                    }
                }
            }
        }
    }

    graph
}

fn neighbours(value: &char, ignore_slopes: bool) -> Vec<Point> {
    if ignore_slopes {
        vec![EAST, SOUTH, WEST, NORTH]
    } else {
        match value {
            &PATH => vec![EAST, SOUTH, WEST, NORTH],
            &SLOPE_EAST => vec![EAST],
            &SLOPE_SOUTH => vec![SOUTH],
            &SLOPE_WEST => vec![WEST],
            &SLOPE_NORTH => vec![NORTH],
            _ => panic!(),
        }
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();
    let start = grid.find_in_row(0, &PATH).unwrap();
    let end = grid.find_in_row(grid.x_len - 1, &PATH).unwrap();

    let mut graph = parse_graph(&grid, false);
    graph.compact();

    longest_dijkstra(
        &graph,
        format!("{}-{}", start.x, start.y),
        format!("{}-{}", end.x, end.y),
    )
}

pub fn solve_part_2(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();
    let start = grid.find_in_row(0, &PATH).unwrap();
    let end = grid.find_in_row(grid.x_len - 1, &PATH).unwrap();

    let mut graph = parse_graph(&grid, true);
    graph.compact();

    longest_dijkstra(
        &graph,
        format!("{}-{}", start.x, start.y),
        format!("{}-{}", end.x, end.y),
    )
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
        assert_eq!(super::solve_part_2(&include_str!("day23/test_input")), 154);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day23/input")), 6_410);
    }
}
