use std::{collections::HashSet, str::FromStr};

use crate::utils::{
    point::{Point, EAST, NORTH, SOUTH, WEST},
    ParseInputError,
};

#[derive(Debug)]
pub struct Maze {
    data: Vec<Vec<char>>,
    start: Point,
    x_len: usize,
    y_len: usize,
}

impl FromStr for Maze {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, rest) = s.split_once("\n").unwrap();
        let (start_x_str, start_y_str) = start_str.split_once(",").unwrap();
        let start = Point {
            x: start_x_str.parse().unwrap(),
            y: start_y_str.parse().unwrap(),
        };
        let data: Vec<Vec<char>> = rest.lines().map(|line| line.chars().collect()).collect();
        let x_len = data.len();
        let y_len = data.get(0).unwrap().len();

        Ok(Maze {
            data,
            start,
            x_len,
            y_len,
        })
    }
}

impl Maze {
    pub fn walk(&self) -> HashSet<Point> {
        let mut visited_points: HashSet<Point> = HashSet::new();
        visited_points.insert(self.start);

        let mut current_point = *self.neighbours(self.start).first().unwrap();
        while current_point != self.start {
            visited_points.insert(current_point);

            let neighbours = self.neighbours(current_point);
            current_point = *neighbours
                .iter()
                .find(|neighbour| !visited_points.contains(neighbour))
                .unwrap_or(&self.start);
        }

        visited_points
    }

    pub fn enclosed_points_count(&self, maze_points: HashSet<Point>) -> usize {
        let mut result = 0;
        for x in 0..self.x_len {
            for y in 0..self.y_len {
                let current_point = Point {
                    x: (x as isize),
                    y: (y as isize),
                };

                if maze_points.contains(&current_point) {
                    continue;
                }

                let mut enclosed = false;
                let mut current_line = '.';
                let mut xx_point = current_point;
                let direction = NORTH;
                for _ in (0..x).rev() {
                    xx_point = xx_point + direction;
                    let xx_char = self.get_char(xx_point);

                    let is_maze_point = maze_points.contains(&xx_point);
                    if !is_maze_point {
                        continue;
                    }

                    if xx_char == 'J' || xx_char == 'L' {
                        current_line = xx_char;
                    } else if xx_char == '7' || xx_char == 'F' {
                        if (current_line == 'J' && xx_char == 'F') || (current_line == 'L' && xx_char == '7') {
                            enclosed = !enclosed;
                        }
                        current_line = '.'
                    } else if xx_char == '-' {
                        enclosed = !enclosed;
                    }
                }

                if enclosed {
                    result += 1;
                }
            }
        }

        result
    }

    fn neighbours(&self, point: Point) -> Vec<Point> {
        let mut neighbours: Vec<Point> = vec![];

        let my_connections = self.connecting_points(point);

        let north = point + NORTH;
        if my_connections.contains(&north) && self.connecting_points(north).contains(&point) {
            neighbours.push(point + NORTH);
        }

        let east = point + EAST;
        if my_connections.contains(&east) && self.connecting_points(east).contains(&point) {
            neighbours.push(point + EAST);
        }

        let south = point + SOUTH;
        if my_connections.contains(&south) && self.connecting_points(south).contains(&point) {
            neighbours.push(point + SOUTH);
        }

        let west = point + WEST;
        if my_connections.contains(&west) && self.connecting_points(west).contains(&point) {
            neighbours.push(point + WEST);
        }

        neighbours
    }

    fn connecting_points(&self, point: Point) -> Vec<Point> {
        match self.get_char(point) {
            '|' => vec![point + NORTH, point + SOUTH],
            '-' => vec![point + EAST, point + WEST],
            'L' => vec![point + NORTH, point + EAST],
            'J' => vec![point + NORTH, point + WEST],
            '7' => vec![point + SOUTH, point + WEST],
            'F' => vec![point + SOUTH, point + EAST],
            _ => vec![],
        }
    }

    pub fn get_char(&self, point: Point) -> char {
        self.data[point.x as usize][point.y as usize]
    }
}
