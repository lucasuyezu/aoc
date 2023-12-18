use super::point::Point;

#[derive(Debug)]
pub struct Polygon {
    pub vertices: Vec<Point>,
}
impl Polygon {
    /*
     * Got this from https://en.wikipedia.org/wiki/Shoelace_formula
     * ATTENTION: This fn uses X as left/right and Y as up/down.
     */
    pub fn shoelace(&self) -> f64 {
        let mut result: f64 = 0.0;

        self.vertices.iter().enumerate().for_each(|(i, point_a)| {
            let point_b = self.vertices[(i + 1) % self.vertices.len()];

            // dbg!(point_a);
            // dbg!(point_b);

            result += (point_a.x as f64 * point_b.y as f64) - (point_b.x as f64 * point_a.y as f64);
        });

        result / 2.0
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{geometry::Polygon, point::Point};

    #[test]
    fn test_area() {
        let polygon = Polygon {
            vertices: vec![
                Point { x: 1, y: 6 },
                Point { x: 3, y: 1 },
                Point { x: 7, y: 2 },
                Point { x: 4, y: 4 },
                Point { x: 8, y: 5 },
            ],
        };

        assert_eq!(polygon.shoelace(), 16.5);
    }
}
