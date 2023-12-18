use std::collections::{HashMap, VecDeque};

use crate::utils::{
    parse_input_into_usize_grid,
    point::{Point, EAST, NORTH, ORIGIN, SOUTH, WEST},
    Grid,
};

type Node = (Point, Point, usize);

fn dijkstra(grid: &Grid<usize>, start: Point, end: Point, min_steps: usize, max_steps: usize) -> usize {
    let mut min_distances: HashMap<Node, usize> = HashMap::new();
    let mut queue: VecDeque<(Node, usize)> = VecDeque::new();

    queue.push_back(((start, start, 0), 0));

    // TODO: Replace this with a proper priority queue
    let mut results = vec![];

    while let Some(((cur_pos, cur_dir, cur_steps), cur_dist)) = queue.pop_front() {
        if cur_pos == end {
            results.push(cur_dist);
        }

        let cur_min_dist = *min_distances.get(&(cur_pos, cur_dir, cur_steps)).unwrap_or(&usize::MAX);
        if cur_dist > cur_min_dist {
            continue;
        }

        for (neighbour, n_dist) in neighbours((cur_pos, cur_dir, cur_steps), min_steps, max_steps, grid) {
            let tentative_distance = cur_dist + n_dist;
            let cur_min_dist = *min_distances.get(&neighbour).unwrap_or(&usize::MAX);
            if tentative_distance < cur_min_dist {
                queue.push_back((neighbour, tentative_distance));
                min_distances.insert(neighbour, tentative_distance);
            }
        }
    }

    results.into_iter().min().unwrap()
}

fn origin_neighbours(grid: &Grid<usize>, (cur_pos, cur_dir, cur_steps): Node, steps: usize) -> Option<(Node, usize)> {
    let mut pos = cur_pos;
    let mut distance = 0;

    for _ in 0..steps {
        pos = pos + cur_dir;
        if !grid.is_inside(&pos) {
            return None;
        }
        distance += grid.data[pos.x as usize][pos.y as usize];
    }

    Some(((pos, cur_dir, cur_steps + steps), distance))
}

fn neighbours(
    (cur_pos, cur_dir, cur_steps): Node,
    min_steps: usize,
    max_steps: usize,
    grid: &Grid<usize>,
) -> Vec<(Node, usize)> {
    match cur_dir {
        ORIGIN => {
            let mut results = vec![];
            if let Some(n) = origin_neighbours(grid, (cur_pos, EAST, cur_steps), min_steps) {
                results.push(n);
            }
            if let Some(n) = origin_neighbours(grid, (cur_pos, SOUTH, cur_steps), min_steps) {
                results.push(n);
            }
            return results;
        }
        NORTH => {
            let mut results = vec![];
            if cur_steps < max_steps {
                if let Some(n) = origin_neighbours(grid, (cur_pos, cur_dir, cur_steps), 1) {
                    results.push(n);
                }
            }
            if let Some(n) = origin_neighbours(grid, (cur_pos, EAST, 0), min_steps) {
                results.push(n);
            }
            if let Some(n) = origin_neighbours(grid, (cur_pos, WEST, 0), min_steps) {
                results.push(n);
            }

            results
        }
        EAST => {
            let mut results = vec![];
            if cur_steps < max_steps {
                if let Some(n) = origin_neighbours(grid, (cur_pos, cur_dir, cur_steps), 1) {
                    results.push(n);
                }
            }
            if let Some(n) = origin_neighbours(grid, (cur_pos, SOUTH, 0), min_steps) {
                results.push(n);
            }
            if let Some(n) = origin_neighbours(grid, (cur_pos, NORTH, 0), min_steps) {
                results.push(n);
            }

            results
        }
        SOUTH => {
            let mut results = vec![];
            if cur_steps < max_steps {
                if let Some(n) = origin_neighbours(grid, (cur_pos, cur_dir, cur_steps), 1) {
                    results.push(n);
                }
            }
            if let Some(n) = origin_neighbours(grid, (cur_pos, WEST, 0), min_steps) {
                results.push(n);
            }
            if let Some(n) = origin_neighbours(grid, (cur_pos, EAST, 0), min_steps) {
                results.push(n);
            }

            results
        }
        WEST => {
            let mut results = vec![];
            if cur_steps < max_steps {
                if let Some(n) = origin_neighbours(grid, (cur_pos, cur_dir, cur_steps), 1) {
                    results.push(n);
                }
            }
            if let Some(n) = origin_neighbours(grid, (cur_pos, NORTH, 0), min_steps) {
                results.push(n);
            }
            if let Some(n) = origin_neighbours(grid, (cur_pos, SOUTH, 0), min_steps) {
                results.push(n);
            }

            results
        }
        _ => panic!(),
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let grid = parse_input_into_usize_grid(input);
    let start = Point::origin();
    let end = Point {
        x: grid.x_len as isize - 1,
        y: grid.y_len as isize - 1,
    };

    dijkstra(&grid, start, end, 1, 3)
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = parse_input_into_usize_grid(input);
    let start = Point::origin();
    let end = Point {
        x: grid.x_len as isize - 1,
        y: grid.y_len as isize - 1,
    };

    dijkstra(&grid, start, end, 4, 10)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day17/test_input")), 102);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day17/input")), 638);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day17/test_input")), 94);
    }

    #[test]
    fn part2_test_input_ultra() {
        assert_eq!(super::solve_part_2(&include_str!("day17/test_input_ultra")), 71);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day17/input")), 748);
    }
}
