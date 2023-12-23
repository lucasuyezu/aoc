use std::str::FromStr;

use super::{point::Point, ParseInputError};

#[derive(Debug)]
pub struct Grid<T: Copy> {
    pub data: Vec<Vec<T>>,
    pub x_len: usize,
    pub y_len: usize,
}
impl<T: Copy + PartialEq> Grid<T> {
    pub fn is_inside(&self, point: &Point) -> bool {
        point.x >= 0 && (point.x as usize) < self.x_len && point.y >= 0 && (point.y as usize) < self.y_len
    }

    pub fn get_value(&self, point: &Point) -> Option<T> {
        if self.is_inside(point) {
            return Some(self.data[point.x as usize][point.y as usize]);
        }

        None
    }

    pub fn get_pos(&self, val: &T) -> Option<Point> {
        for x in 0..self.x_len {
            if let Some(found) = self.find_in_row(x, val) {
                return Some(found);
            }
        }

        None
    }

    pub fn find_in_row(&self, x: usize, val: &T) -> Option<Point> {
        for y in 0..self.y_len {
            if self.data[x][y] == *val {
                return Some(Point {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }

        None
    }
}

impl FromStr for Grid<char> {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let x_len = data.len();
        let y_len = data[0].len();

        Ok(Grid { data, x_len, y_len })
    }
}

impl FromStr for Grid<usize> {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();

        let x_len = data.len();
        let y_len = data[0].len();

        Ok(Grid { data, x_len, y_len })
    }
}
