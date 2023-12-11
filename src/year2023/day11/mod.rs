use std::collections::{HashSet, VecDeque};

use crate::utils::point::{Point, EAST, NORTH, SOUTH, WEST};
use itertools::Itertools;

pub struct Universe {
    data: Vec<Vec<char>>,
}
impl Universe {
    fn expand(&mut self) {
        let x_len = self.data.len();
        let y_len = self.data.first().unwrap().len();

        let empty_lines: Vec<usize> = (0..x_len)
            .filter(|x| self.data.get(*x).unwrap().iter().all(|c| *c == '.'))
            .collect();

        // dbg!(&empty_lines);

        let empty_columns: Vec<usize> = (0..y_len)
            .filter(|y| (0..x_len).all(|x| *self.data.get(x).unwrap().get(*y).unwrap() == '.'))
            .collect();

        // dbg!(&empty_columns);

        // print(self);

        for (offset, x) in empty_lines.iter().enumerate() {
            // println!("Inserting an empty line at x {}", x + offset);
            let line = self.data.get(x + offset).unwrap().clone();
            self.data.insert(x + offset, line);
        }

        // print(self);

        for (offset, y) in empty_columns.iter().enumerate() {
            for x in 0..(x_len + empty_lines.len()) {
                // println!("Inserting a dot at {}-{}", x, y + offset);
                self.data.get_mut(x).unwrap().insert(y + offset, '.');
            }
        }

        // print(self);
    }

    fn galaxies(&self) -> Vec<Point> {
        let mut result: Vec<Point> = vec![];
        for (x, line) in self.data.iter().enumerate() {
            for (y, c) in line.iter().enumerate() {
                if *c == '#' {
                    result.push(Point {
                        x: x as isize,
                        y: y as isize,
                    });
                }
            }
        }

        result
    }

    fn shortest_distance(&self, pair: Vec<&Point>) -> usize {
        let x_len = self.data.len();
        let y_len = self.data.get(0).unwrap().len();

        let start = **pair.get(0).unwrap();
        let end = **pair.get(1).unwrap();
        println!("Calculating shortest distance between {:?} and {:?}", start, end);

        let mut visited: HashSet<Point> = HashSet::new();
        let mut queue: VecDeque<(Point, usize)> = VecDeque::new();

        queue.push_back((start, 0));
        visited.insert(start);

        while let Some((current, current_distance)) = queue.pop_front() {
            if current == end {
                return current_distance;
            }

            let north = current + NORTH;
            if north.x >= 0 && !visited.contains(&north) {
                queue.push_back((north, current_distance + 1));
                visited.insert(north);
            }

            let east = current + EAST;
            if east.y < y_len as isize && !visited.contains(&east) {
                queue.push_back((east, current_distance + 1));
                visited.insert(east);
            }

            let south = current + SOUTH;
            if south.y < x_len as isize && !visited.contains(&south) {
                queue.push_back((south, current_distance + 1));
                visited.insert(south);
            }

            let west = current + WEST;
            if west.y >= 0 && !visited.contains(&west) {
                queue.push_back((west, current_distance + 1));
                visited.insert(west);
            }
        }

        0
    }
}

fn print(universe: &Universe) {
    for line in universe.data.iter() {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
}

fn parse_part_1_input(input: &str) -> Universe {
    Universe {
        data: input.lines().map(|line| line.chars().collect()).collect(),
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let mut universe = parse_part_1_input(input);
    universe.expand();

    let galaxies = universe.galaxies();
    galaxies
        .iter()
        .combinations(2)
        .unique()
        .map(|pair| universe.shortest_distance(pair))
        .sum()
}

pub fn solve_part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 374);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 10_292_708);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 71_503);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 34_655_848);
    }
}
