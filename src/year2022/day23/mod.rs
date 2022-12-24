use std::{collections::HashSet, str::FromStr, string::ParseError};

use crate::utils::point::*;

const POSITION_DELTAS: [Point; 8] = [NORTH, NE, EAST, SE, SOUTH, SW, WEST, NW];

#[derive(Debug)]
struct Grid {
    elves: HashSet<Point>,
    directions: Vec<Point>,
}

impl FromStr for Grid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut elves: HashSet<Point> = HashSet::new();

        for (x, line) in s.lines().enumerate() {
            for (y, c) in line.chars().enumerate() {
                if c == '#' {
                    let point = Point {
                        x: x as isize,
                        y: y as isize,
                    };

                    elves.insert(point);
                }
            }
        }

        Ok(Self {
            elves,
            directions: vec![NORTH, SOUTH, WEST, EAST],
        })
    }
}

impl Grid {
    fn get_min_max_coords(&self) -> (isize, isize, isize, isize) {
        let mut min_x: isize = isize::MAX;
        let mut max_x: isize = isize::MIN;
        let mut min_y: isize = isize::MAX;
        let mut max_y: isize = isize::MIN;

        for elf in self.elves.iter() {
            min_x = min_x.min(elf.x);
            max_x = max_x.max(elf.x);
            min_y = min_y.min(elf.y);
            max_y = max_y.max(elf.y);
        }

        (min_x, max_x, min_y, max_y)
    }

    fn run_rounds(&mut self, rounds: i32) {
        println!("== Start ==");
        self.print_frame();

        for i in 1..=rounds {
            println!("== Round {:?} ==", i);
            self.run_round();
        }
    }

    fn run_round(&mut self) -> usize {
        // First half: Every elf may propose a new position
        let mut proposed_positions: Vec<(Point, Point)> = vec![];
        for (elf, positions) in self.adjacent_positions() {
            if positions
                .iter()
                .all(|position| !self.elves.contains(position))
            {
                continue;
            }

            if let Some(proposed_position) =
                self.directions.iter().find(|direction| match **direction {
                    NORTH => {
                        !self.elves.contains(&(*elf + NORTH))
                            && !self.elves.contains(&(*elf + NE))
                            && !self.elves.contains(&(*elf + NW))
                    }
                    SOUTH => {
                        !self.elves.contains(&(*elf + SOUTH))
                            && !self.elves.contains(&(*elf + SE))
                            && !self.elves.contains(&(*elf + SW))
                    }
                    WEST => {
                        !self.elves.contains(&(*elf + WEST))
                            && !self.elves.contains(&(*elf + NW))
                            && !self.elves.contains(&(*elf + SW))
                    }
                    EAST => {
                        !self.elves.contains(&(*elf + EAST))
                            && !self.elves.contains(&(*elf + NE))
                            && !self.elves.contains(&(*elf + SE))
                    }
                    _ => panic!(),
                })
            {
                proposed_positions.push((*elf, *elf + *proposed_position));
            }
        }

        // Second half: Elves who were the only ones to propose a move to point X move
        let mut elves_moved = 0;
        for (elf, new_position) in proposed_positions.iter() {
            if proposed_positions
                .iter()
                .filter(|(_elf, pos)| *pos == *new_position)
                .count()
                == 1
            {
                self.elves.remove(&elf);
                self.elves.insert(*new_position);
                elves_moved += 1;
            }
        }

        self.print_frame();

        let dir = self.directions.remove(0);
        self.directions.push(dir);

        elves_moved
    }

    fn adjacent_positions(&self) -> Vec<(&Point, Vec<Point>)> {
        self.elves
            .iter()
            .map(|elf| {
                let proposed_positions = POSITION_DELTAS
                    .iter()
                    .map(|pos| *elf + *pos)
                    .collect::<Vec<_>>();
                (elf, proposed_positions)
            })
            .collect::<Vec<_>>()
    }

    fn print_frame(&self) {
        let (min_x, max_x, min_y, max_y) = self.get_min_max_coords();

        let mut frame = String::new();
        for x in min_x..=max_x {
            frame.push_str(format!("{:>2} ", x).as_str());
            for y in min_y..=max_y {
                if self.elves.contains(&Point { x, y }) {
                    frame.push('#');
                } else {
                    frame.push('.');
                }
            }
            frame.push('\n');
        }

        println!("{}", frame);
    }

    fn empty_tile_count(&self) -> usize {
        let (min_x, max_x, min_y, max_y) = self.get_min_max_coords();

        ((max_x - min_x + 1) * (max_y - min_y + 1) - self.elves.len() as isize) as usize
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let mut grid = input.parse::<Grid>().unwrap();
    grid.run_rounds(10);
    grid.empty_tile_count()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut grid = input.parse::<Grid>().unwrap();
    let mut num_rounds = 0;
    let mut elves_moved = 1;

    while elves_moved != 0 {
        elves_moved = grid.run_round();
        num_rounds += 1;
    }

    num_rounds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part_1(&include_str!("test_input")), 110);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(solve_part_1(&include_str!("input")), 3925);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(solve_part_2(&include_str!("test_input")), 20);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(solve_part_2(&include_str!("input")), 903);
    }
}
