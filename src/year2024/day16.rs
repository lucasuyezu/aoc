use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    usize,
};

use crate::utils::{
    grid::Grid,
    point::{Point, EAST, NORTH, SOUTH, WEST},
};

#[derive(Clone, Hash, PartialEq, Eq, Debug, Copy)]
struct Node {
    pos: Point,
    dir: i8,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct State {
    node: Node,
    cost: usize,
    path: Vec<Point>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const DIRECTIONS: [Point; 4] = [NORTH, EAST, SOUTH, WEST];

fn dijkstra(input: &str) -> (usize, usize) {
    let grid: Grid<char> = input.parse().unwrap();
    let start_point = grid.get_pos(&'S').unwrap();
    let end_point = grid.get_pos(&'E').unwrap();

    let mut queue = BinaryHeap::new();
    queue.push(State {
        node: Node {
            pos: start_point,
            dir: 1,
        },
        cost: 0,
        path: vec![start_point],
    });

    let mut dists = HashMap::new();
    dists.insert(
        Node {
            pos: start_point,
            dir: 1,
        },
        0,
    );

    let mut visited = HashSet::new();

    let mut min_dist = usize::MAX;

    while let Some(State { cost, node, path }) = queue.pop() {
        if cost > dists[&node] {
            continue;
        }

        if node.pos == end_point {
            if cost <= min_dist {
                min_dist = cost;
                for p in path.into_iter() {
                    visited.insert(p);
                }
            }
            continue;
        }

        for (n_dir, n_cost) in [
            (node.dir, 1),
            ((node.dir + 1).rem_euclid(DIRECTIONS.len() as i8), 1001),
            ((node.dir - 1).rem_euclid(DIRECTIONS.len() as i8), 1001),
        ] {
            let mut n_state = State {
                cost: cost + n_cost,
                node: Node {
                    pos: node.pos + DIRECTIONS[n_dir as usize],
                    dir: n_dir,
                },
                path: path.clone(),
            };
            n_state.path.push(n_state.node.pos);

            let n_char = grid.get_value(&n_state.node.pos).unwrap();
            if n_char != '#' && n_state.cost <= *dists.get(&n_state.node).or(Some(&usize::MAX)).unwrap() {
                dists.insert(n_state.node, n_state.cost);
                queue.push(n_state);
            }
        }
    }

    dbg!(&visited);

    (min_dist, visited.len())
}

pub fn solve_part_1(input: &str) -> usize {
    dijkstra(input).0
}
pub fn solve_part_2(input: &str) -> usize {
    dijkstra(input).1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_input_small() {
        let (min_dist, pos_count) = super::dijkstra(&include_str!("day16/test_input_small"));
        assert_eq!(min_dist, 7036);
        assert_eq!(pos_count, 45);
    }

    #[test]
    fn test_input_large() {
        let (min_dist, pos_count) = super::dijkstra(&include_str!("day16/test_input"));
        assert_eq!(min_dist, 11048);
        assert_eq!(pos_count, 64);
    }

    #[test]
    fn test_real_input() {
        let (min_dist, pos_count) = super::dijkstra(&include_str!("day16/input"));
        assert_eq!(min_dist, 75416);
        assert_eq!(pos_count, 476);
    }
}
