use super::point::Point;

#[allow(dead_code)]
pub struct Polygon {
    vertices: Vec<Point>,
}
#[allow(dead_code)]
impl Polygon {
    /*
    Got this from https://en.wikipedia.org/wiki/Shoelace_formula
    */
    pub fn area(&self) -> f32 {
        let mut result = 0.0;

        for i in 0..self.vertices.len() {
            let point_a = self.vertices[i];
            let point_b = self.vertices[(i + 1) % self.vertices.len()];

            result += (point_a.x * point_b.y) as f32 - (point_a.y * point_b.x) as f32;

            dbg!(point_a);
            dbg!(point_b);
        }

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

        assert_eq!(polygon.area(), 16.5);
    }
}
