use core::time;
use std::{collections::HashMap, thread};

use crate::utils::get_timer_millis;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    stuff_hash: HashMap<Point, String>,
    min_x: usize,
    max_x: usize,
    max_y: usize,
}

const SAND_UNIT_ORIGIN: Point = Point { x: 500, y: 0 };

impl Grid {
    fn from(input: &str, has_floor: bool) -> Grid {
        let mut stuff_hash: HashMap<Point, String> = HashMap::new();
        let mut min_x = usize::MAX;
        let mut min_y = usize::MAX;
        let mut max_x = usize::MIN;
        let mut max_y = usize::MIN;

        let shapes: Vec<Vec<Point>> = input
            .lines()
            .map(|line| {
                line.split(" -> ")
                    .map(|point_str| {
                        let (x_str, y_str) = point_str.split_once(",").unwrap();
                        let x = x_str.parse().unwrap();
                        let y = y_str.parse().unwrap();

                        if x < min_x {
                            min_x = x;
                        }

                        if x > max_x {
                            max_x = x;
                        }

                        if y < min_y {
                            min_y = y;
                        }

                        if y > max_y {
                            max_y = y;
                        }

                        Point { x, y }
                    })
                    .collect::<Vec<Point>>()
            })
            .collect();

        shapes.iter().for_each(|shape| {
            shape.windows(2).for_each(|points| {
                let mut x_points = [points[0].x, points[1].x];
                let mut y_points = [points[0].y, points[1].y];

                x_points.sort();
                y_points.sort();

                let [start_x, end_x] = x_points;
                let [start_y, end_y] = y_points;

                for x in start_x..=end_x {
                    for y in start_y..=end_y {
                        stuff_hash.insert(Point { x, y }, "#".to_string());
                    }
                }
            })
        });

        if has_floor {
            max_y += 2;

            for x in 0..(max_x + 10_000) {
                stuff_hash.insert(Point { x, y: max_y }, "#".to_string());
            }
        }

        Grid {
            stuff_hash,
            min_x,
            max_x,
            max_y,
        }
    }

    fn animate(&mut self) {
        let mut cur_point = SAND_UNIT_ORIGIN;
        self.stuff_hash.insert(cur_point, "o".to_string());

        loop {
            if let Some(timer_millis) = get_timer_millis() {
                self.print_frame();

                let timer = time::Duration::from_millis(timer_millis);
                thread::sleep(timer);
            }

            match self.move_sand(&cur_point) {
                Some(new_point) => {
                    if new_point.y > self.max_y {
                        self.stuff_hash.remove(&new_point);
                        return;
                    } else {
                        cur_point = new_point;
                        if new_point.x < self.min_x {
                            self.min_x = new_point.x;
                        }
                        if new_point.x > self.max_x {
                            self.max_x = new_point.x;
                        }
                    }
                }
                None => {
                    if cur_point == SAND_UNIT_ORIGIN {
                        return;
                    } else {
                        cur_point = SAND_UNIT_ORIGIN;
                        self.stuff_hash.insert(cur_point.clone(), "o".to_string());
                    }
                }
            }
        }
    }

    fn move_sand(&mut self, cur_point: &Point) -> Option<Point> {
        let mut new_point = cur_point.clone();

        // can move straight down?
        new_point.y += 1;
        if !self.stuff_hash.contains_key(&new_point) {
            self.stuff_hash.remove(&cur_point);
            self.stuff_hash.insert(new_point.clone(), "o".to_string());

            return Some(new_point);
        }

        // can move down-left?
        new_point.x -= 1;
        if !self.stuff_hash.contains_key(&new_point) {
            self.stuff_hash.remove(&cur_point);
            self.stuff_hash.insert(new_point.clone(), "o".to_string());

            return Some(new_point);
        }

        // can move down-right?
        new_point.x += 2;
        if !self.stuff_hash.contains_key(&new_point) {
            self.stuff_hash.remove(&cur_point);
            self.stuff_hash.insert(new_point.clone(), "o".to_string());

            return Some(new_point);
        }

        None
    }

    fn print_frame(&self) {
        // Clear screen
        println!("{}[2J", 27 as char);
        let mut frame = String::new();

        for y in 0..=self.max_y {
            frame.push_str(&format!(" {:>3}", y));

            if y == self.max_y {
                frame.push('~');
            } else {
                frame.push(' ');
            }

            for x in (self.min_x - 1)..=(self.max_x + 1) {
                let point = Point { x, y };
                if let Some(stuff) = self.stuff_hash.get(&point) {
                    frame.push_str(stuff);
                } else if point == SAND_UNIT_ORIGIN {
                    frame.push('+');
                } else {
                    frame.push('.');
                }
            }
            frame.push('\n');
        }

        println!("{}", frame);
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let mut grid = Grid::from(input, false);

    grid.animate();

    grid.stuff_hash
        .values()
        .filter(|value| **value == "o".to_string())
        .count()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut grid = Grid::from(input, true);

    grid.animate();

    grid.stuff_hash
        .values()
        .filter(|value| **value == "o".to_string())
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 24);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 1072);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 93);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 24_659);
    }
}
