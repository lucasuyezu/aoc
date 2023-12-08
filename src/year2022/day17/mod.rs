use core::time;
use std::{collections::HashMap, thread};

use crate::utils::get_timer_millis;
use crate::utils::point::Point;

#[derive(Debug)]
enum RockShape {
    Minus,
    Plus,
    L,
    I,
    Square,
}

impl RockShape {
    fn from(n: usize) -> RockShape {
        match n % 5 {
            0 => RockShape::Minus,
            1 => RockShape::Plus,
            2 => RockShape::L,
            3 => RockShape::I,
            4 => RockShape::Square,
            _ => panic!("Invalid n for get_rock"),
        }
    }

    fn get_starting_points(&self, max_y: isize) -> Vec<Point> {
        match self {
            RockShape::Minus => vec![
                Point { x: 2, y: max_y + 4 },
                Point { x: 3, y: max_y + 4 },
                Point { x: 4, y: max_y + 4 },
                Point { x: 5, y: max_y + 4 },
            ],
            RockShape::Plus => vec![
                Point { x: 3, y: max_y + 6 },
                Point { x: 2, y: max_y + 5 },
                Point { x: 3, y: max_y + 5 },
                Point { x: 4, y: max_y + 5 },
                Point { x: 3, y: max_y + 4 },
            ],
            RockShape::L => vec![
                Point { x: 4, y: max_y + 6 },
                Point { x: 4, y: max_y + 5 },
                Point { x: 4, y: max_y + 4 },
                Point { x: 3, y: max_y + 4 },
                Point { x: 2, y: max_y + 4 },
            ],
            RockShape::I => vec![
                Point { x: 2, y: max_y + 7 },
                Point { x: 2, y: max_y + 6 },
                Point { x: 2, y: max_y + 5 },
                Point { x: 2, y: max_y + 4 },
            ],
            RockShape::Square => vec![
                Point { x: 2, y: max_y + 5 },
                Point { x: 3, y: max_y + 5 },
                Point { x: 2, y: max_y + 4 },
                Point { x: 3, y: max_y + 4 },
            ],
        }
    }
}

#[derive(Debug)]
struct Grid {
    stuff_hash: HashMap<Point, String>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            stuff_hash: HashMap::new(),
            min_x: 0,
            max_x: 7,
            min_y: 0,
            max_y: 0,
        }
    }

    fn animate(&mut self, input: &str, rock_count: usize) {
        let mut jet_id = 0;
        let mut jet_direction;
        let mut chars = input.chars().enumerate().cycle();
        let mut cache: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        for piece_id in 0..rock_count {
            if piece_id == 200 {
                dbg!(piece_id);
                dbg!()
            }
            let cache_key = (piece_id % 5, jet_id % input.len());
            if cache.contains_key(&cache_key) {
                let (s, t) = cache.get(&cache_key).unwrap();
                let d = (1_000_000_000_000 - s) / (piece_id - s);
                let m = (1_000_000_000_000 - s) % (piece_id - s);
                if m == 0 {
                    dbg!(self.max_y as usize + (self.max_y as usize - t) * d);
                    panic!();
                }
            } else {
                cache.insert(cache_key, (piece_id, self.max_y as usize));
            }

            let current_rock = RockShape::from(piece_id);
            let rock_starting_points = current_rock.get_starting_points(if piece_id == 0 { -1 } else { self.max_y });

            let mut rock_top_left = rock_starting_points[0];

            self.max_y = rock_top_left.y;

            for starting_point in rock_starting_points.iter() {
                self.stuff_hash.insert(starting_point.clone(), "@".to_string());
            }

            if let Some(timer_millis) = get_timer_millis() {
                self.print_frame();

                let timer = time::Duration::from_millis(timer_millis);
                thread::sleep(timer);
            }

            let mut rock_points = rock_starting_points.clone();
            loop {
                (jet_id, jet_direction) = chars.next().unwrap();
                if jet_id == 0 {
                    dbg!(piece_id);
                    dbg!(piece_id % 5);
                    dbg!(jet_id);
                    dbg!(jet_direction);
                }
                if self.can_move_horizontally(&current_rock, &rock_top_left, jet_direction) {
                    for old_point in &rock_points {
                        self.stuff_hash.remove(&old_point);
                    }

                    for point in rock_points.iter_mut() {
                        if jet_direction == '<' {
                            point.x -= 1;
                        } else {
                            point.x += 1;
                        }
                    }
                    rock_top_left = rock_points[0];

                    for new_point in &rock_points {
                        self.stuff_hash.insert(new_point.clone(), "@".to_string());
                    }
                }

                if let Some(timer_millis) = get_timer_millis() {
                    self.print_frame();

                    let timer = time::Duration::from_millis(timer_millis);
                    thread::sleep(timer);
                }

                // move down 1 line
                if self.can_move_down(&current_rock, &rock_top_left) {
                    for old_point in &rock_points {
                        self.stuff_hash.remove(&old_point);
                    }

                    for point in rock_points.iter_mut() {
                        point.y -= 1;
                    }
                    rock_top_left = rock_points[0];

                    for new_point in &rock_points {
                        self.stuff_hash.insert(new_point.clone(), "@".to_string());
                    }
                } else {
                    break;
                }

                if let Some(timer_millis) = get_timer_millis() {
                    self.print_frame();

                    let timer = time::Duration::from_millis(timer_millis);
                    thread::sleep(timer);
                }
            }

            for point in &rock_points {
                self.stuff_hash.insert(point.clone(), "#".to_string());
            }

            self.max_y = self.find_new_max_y();
        }
    }

    fn print_frame(&self) {
        // Clear screen
        // println!("{}[2J", 27 as char);
        let mut frame = String::new();

        for y in (-1..=self.max_y).rev() {
            if y == -1 {
                frame.push_str("    +");
            } else {
                frame.push_str(format!("{:>3} |", y).as_str());
            }

            for x in self.min_x..self.max_x {
                let point = Point { x, y };
                if let Some(stuff) = self.stuff_hash.get(&point) {
                    frame.push_str(stuff);
                } else {
                    if y == -1 {
                        frame.push_str("-");
                    } else {
                        frame.push_str(".");
                    }
                }
            }
            if y == -1 {
                frame.push_str("+\n");
            } else {
                frame.push_str("|\n");
            }
        }

        println!("{}", frame);
    }

    fn can_move_horizontally(&self, current_rock: &RockShape, rock_top_left: &Point, c: char) -> bool {
        match current_rock {
            RockShape::Minus => {
                return (c == '<'
                    && rock_top_left.x > self.min_x
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 1,
                        y: rock_top_left.y,
                    }))
                    || (c == '>'
                        && rock_top_left.x + 4 < self.max_x
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 4,
                            y: rock_top_left.y,
                        }));
            }
            RockShape::Plus => {
                return (c == '<'
                    && rock_top_left.x - 1 > self.min_x
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 2,
                        y: rock_top_left.y - 1,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 1,
                        y: rock_top_left.y - 2,
                    }))
                    || (c == '>'
                        && rock_top_left.x + 2 < self.max_x
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 2,
                            y: rock_top_left.y - 1,
                        })
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 1,
                            y: rock_top_left.y - 2,
                        }));
            }
            RockShape::L => {
                return (c == '<'
                    && rock_top_left.x - 2 > self.min_x
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 3,
                        y: rock_top_left.y - 2,
                    }))
                    || (c == '>'
                        && rock_top_left.x + 1 < self.max_x
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 1,
                            y: rock_top_left.y,
                        })
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 1,
                            y: rock_top_left.y - 1,
                        })
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 1,
                            y: rock_top_left.y - 2,
                        }))
            }
            RockShape::I => {
                return (c == '<'
                    && rock_top_left.x > self.min_x
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 1,
                        y: rock_top_left.y,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 1,
                        y: rock_top_left.y - 1,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 1,
                        y: rock_top_left.y - 2,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 1,
                        y: rock_top_left.y - 3,
                    }))
                    || (c == '>'
                        && rock_top_left.x + 1 < self.max_x
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 1,
                            y: rock_top_left.y,
                        })
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 1,
                            y: rock_top_left.y - 1,
                        })
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 1,
                            y: rock_top_left.y - 2,
                        })
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 1,
                            y: rock_top_left.y - 3,
                        }))
            }
            RockShape::Square => {
                return (c == '<'
                    && rock_top_left.x > self.min_x
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 1,
                        y: rock_top_left.y,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 1,
                        y: rock_top_left.y - 1,
                    }))
                    || (c == '>'
                        && rock_top_left.x + 2 < self.max_x
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 2,
                            y: rock_top_left.y,
                        })
                        && !self.stuff_hash.contains_key(&Point {
                            x: rock_top_left.x + 2,
                            y: rock_top_left.y - 1,
                        }))
            }
        }
    }

    fn can_move_down(&self, current_rock: &RockShape, rock_top_left: &Point) -> bool {
        match current_rock {
            RockShape::Minus => {
                return rock_top_left.y > self.min_y
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x,
                        y: rock_top_left.y - 1,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x + 1,
                        y: rock_top_left.y - 1,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x + 2,
                        y: rock_top_left.y - 1,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x + 3,
                        y: rock_top_left.y - 1,
                    })
            }
            RockShape::Plus => {
                return rock_top_left.y > 2
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 1,
                        y: rock_top_left.y - 2,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x,
                        y: rock_top_left.y - 3,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x + 1,
                        y: rock_top_left.y - 2,
                    })
            }
            RockShape::L => {
                return rock_top_left.y > 2
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 2,
                        y: rock_top_left.y - 3,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x - 1,
                        y: rock_top_left.y - 3,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x,
                        y: rock_top_left.y - 3,
                    })
            }
            RockShape::I => {
                return rock_top_left.y > 3
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x,
                        y: rock_top_left.y - 4,
                    })
            }
            RockShape::Square => {
                return rock_top_left.y > 1
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x,
                        y: rock_top_left.y - 2,
                    })
                    && !self.stuff_hash.contains_key(&Point {
                        x: rock_top_left.x + 1,
                        y: rock_top_left.y - 2,
                    })
            }
        }
    }

    fn find_new_max_y(&self) -> isize {
        for y in (self.min_y..=self.max_y).rev() {
            for x in self.min_x..self.max_x {
                let point = Point { x, y: y as isize };
                if let Some(_value) = self.stuff_hash.get(&point) {
                    return y;
                }
            }
        }

        panic!();
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let mut grid = Grid::new();
    grid.animate(input, 2022);
    grid.max_y as usize + 1
}

pub fn solve_part_2(input: &str) -> usize {
    let mut grid = Grid::new();
    grid.animate(input, 1_000_000_000_000);
    grid.max_y as usize + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 3068);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 3065);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 1514285714288);
    }
}
