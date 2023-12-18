use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::{
    parse_input_into_usize_grid,
    point::{Point, EAST, NORTH, SOUTH, WEST},
    Grid,
};

fn neighbours(
    point: Point,
    direction: Option<Point>,
    max_steps: usize,
    steps_north: usize,
    steps_east: usize,
    steps_south: usize,
    steps_west: usize,
) -> Vec<(Point, usize, usize, usize, usize)> {
    // dbg!(&direction);
    match direction {
        None => vec![
            (point + NORTH, 1, 0, 0, 0),
            (point + EAST, 0, 1, 0, 0),
            (point + SOUTH, 0, 0, 1, 0),
            (point + WEST, 0, 0, 0, 1),
        ],
        Some(NORTH) => {
            let mut neighbours = vec![(point + EAST, 0, 1, 0, 0), (point + WEST, 0, 0, 0, 1)];

            if steps_north < max_steps {
                neighbours.push((point + NORTH, steps_north + 1, steps_east, steps_south, steps_west));
            }

            return neighbours;
        }
        Some(EAST) => {
            let mut neighbours = vec![(point + SOUTH, 0, 0, 1, 0), (point + NORTH, 1, 0, 0, 0)];

            if steps_east < max_steps {
                neighbours.push((point + EAST, steps_north, steps_east + 1, steps_south, steps_west));
            }

            return neighbours;
        }
        Some(SOUTH) => {
            let mut neighbours = vec![(point + WEST, 0, 0, 0, 1), (point + EAST, 0, 1, 0, 0)];

            if steps_south < max_steps {
                neighbours.push((point + SOUTH, steps_north, steps_east, steps_south + 1, steps_west));
            }

            return neighbours;
        }
        Some(WEST) => {
            let mut neighbours = vec![(point + NORTH, 1, 0, 0, 0), (point + SOUTH, 0, 0, 1, 0)];

            if steps_west < max_steps {
                neighbours.push((point + WEST, steps_north, steps_east, steps_south, steps_west + 1));
            }

            return neighbours;
        }
        Some(_) => panic!("Invalid direction"),
    }
}

fn dijkstra(grid: &Grid<usize>, start: Point, end: Point, max_steps: usize) -> usize {
    let mut min_distances: HashMap<Point, usize> = HashMap::new();
    min_distances.insert(start, 0);

    let mut queue: VecDeque<(Point, Option<Point>, usize, usize, usize, usize, usize)> = VecDeque::new();
    let mut visited = HashSet::<(Point, Option<Point>, usize, usize, usize, usize)>::new();

    queue.push_back((start, None, 0, 0, 0, 0, 0));
    visited.insert((start, None, 0, 0, 0, 0));

    while let Some(tuple) = queue.pop_front() {
        println!("Popped {:?}", tuple);

        let (cur_pos, direction, cur_dist, steps_north, steps_east, steps_south, steps_west) = tuple;

        for n_tuple in neighbours(
            cur_pos,
            direction,
            max_steps,
            steps_north,
            steps_east,
            steps_south,
            steps_west,
        )
        .into_iter()
        .filter(|(pos, _, _, _, _)| grid.is_inside(pos))
        {
            println!("\tPopped neighbour {:?}", n_tuple);
            let (n_pos, n_steps_north, n_steps_east, n_steps_south, n_steps_west) = n_tuple;
            let cur_min_dist = *min_distances.get(&n_pos).unwrap_or(&usize::MAX);

            println!(
                "\t\t{:?} has distance {}",
                n_pos, grid.data[n_pos.x as usize][n_pos.y as usize]
            );
            let cur_total_dist = cur_dist + grid.data[n_pos.x as usize][n_pos.y as usize];
            if cur_total_dist < cur_min_dist {
                min_distances.insert(n_pos, cur_total_dist);
                visited.insert((
                    n_pos,
                    direction,
                    n_steps_north,
                    n_steps_east,
                    n_steps_south,
                    n_steps_west,
                ));
                queue.push_back((
                    n_pos,
                    Some(cur_pos - n_pos),
                    cur_total_dist,
                    n_steps_north,
                    n_steps_east,
                    n_steps_south,
                    n_steps_west,
                ));
            }
        }
    }

    *min_distances.get(&end).unwrap()
}

pub fn solve_part_1(input: &str) -> usize {
    let grid = parse_input_into_usize_grid(input);
    let start = Point { x: 0, y: 0 };
    let end = Point {
        x: grid.x_len as isize - 1,
        y: grid.y_len as isize - 1,
    };

    let max_steps: usize = 3;

    dijkstra(&grid, start, end, max_steps)
}

pub fn solve_part_2(_: &str) -> usize {
    todo!();
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
        assert_eq!(super::solve_part_2(&include_str!("day17/test_input")), 1);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day17/input")), 1);
    }
}
