#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn manhattan_distance(&self, other_point: &Point) -> usize {
        (self.x - other_point.x).abs() as usize + (self.y - other_point.y).abs() as usize
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
}
