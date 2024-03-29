pub const NORTH: Point = Point { x: -1, y: 0 };
pub const NE: Point = Point { x: -1, y: 1 };
pub const EAST: Point = Point { x: 0, y: 1 };
pub const SE: Point = Point { x: 1, y: 1 };
pub const SOUTH: Point = Point { x: 1, y: 0 };
pub const SW: Point = Point { x: 1, y: -1 };
pub const WEST: Point = Point { x: 0, y: -1 };
pub const NW: Point = Point { x: -1, y: -1 };
pub const ORIGIN: Point = Point { x: 0, y: 0 };

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn origin() -> Point {
        Point {
            x: 0 as isize,
            y: 0 as isize,
        }
    }

    pub fn from_direction(c: char) -> Self {
        match c {
            '>' => EAST,
            'v' => SOUTH,
            '<' => WEST,
            '^' => NORTH,
            x => panic!("Invalid char {}", x),
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: rhs.x - self.x,
            y: rhs.y - self.y,
        }
    }
}

impl Point {
    pub fn manhattan_distance(&self, other_point: &Point) -> usize {
        (self.x - other_point.x).abs() as usize + (self.y - other_point.y).abs() as usize
    }

    pub fn is_touching(&self, other: Point) -> bool {
        (self.x == other.x + 1 && self.y == other.y + 1)
            || (self.x == other.x + 1 && self.y == other.y)
            || (self.x == other.x + 1 && self.y == other.y - 1)
            || (self.x == other.x && self.y == other.y + 1)
            || (self.x == other.x && self.y == other.y - 1)
            || (self.x == other.x - 1 && self.y == other.y + 1)
            || (self.x == other.x - 1 && self.y == other.y)
            || (self.x == other.x - 1 && self.y == other.y - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn test_manhattan_distance() {
        let point_a = Point { x: 1, y: -2 };
        let point_b = Point { x: -3, y: 5 };

        assert_eq!(point_a.manhattan_distance(&point_b), 11);
        assert_eq!(point_b.manhattan_distance(&point_a), 11);
    }

    #[test]
    fn test_add() {
        let point_a = Point { x: 1, y: -2 };
        let point_b = Point { x: -3, y: 5 };

        assert_eq!(Point { x: -2, y: 3 }, point_a + point_b);
    }
}
