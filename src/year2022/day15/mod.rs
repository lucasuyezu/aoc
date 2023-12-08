use super::super::utils::point::Point;
use std::collections::HashSet;

#[derive(Debug)]
struct Pair {
    sensor: Point,
    beacon: Point,
    distance: usize,
}

#[derive(Debug)]
struct Grid {
    pairs: Vec<Pair>,
    beacons: HashSet<Point>,
    min_x: isize,
    max_x: isize,
}

impl Grid {
    fn from(input: &str) -> Grid {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;

        let mut beacons: HashSet<Point> = HashSet::new();

        let pairs: Vec<Pair> = input
            .lines()
            .map(|line| {
                let (sensor_str, beacon_str) = line.split_once(":").unwrap();
                let (sensor_x_str, sensor_y_str) = sensor_str.split_once(",").unwrap();
                let (beacon_x_str, beacon_y_str) = beacon_str.split_once(",").unwrap();

                let sensor_x: isize = sensor_x_str[12..].parse().unwrap();
                let sensor_y: isize = sensor_y_str[3..].parse().unwrap();
                let sensor = Point {
                    x: sensor_x,
                    y: sensor_y,
                };

                let beacon_x: isize = beacon_x_str[24..].parse().unwrap();
                let beacon_y: isize = beacon_y_str[3..].parse().unwrap();
                let beacon = Point {
                    x: beacon_x,
                    y: beacon_y,
                };

                let distance = sensor.manhattan_distance(&beacon);

                beacons.insert(beacon);

                Pair {
                    sensor,
                    beacon,
                    distance,
                }
            })
            .collect();

        pairs.iter().for_each(|pair| {
            if (pair.sensor.x - pair.distance as isize) < min_x {
                min_x = pair.sensor.x - pair.distance as isize;
            }

            if (pair.sensor.x + pair.distance as isize) > max_x {
                max_x = pair.sensor.x + pair.distance as isize;
            }

            if pair.beacon.x < min_x {
                min_x = pair.beacon.x;
            }

            if pair.beacon.x > max_x {
                max_x = pair.beacon.x;
            }
        });

        Grid {
            pairs,
            min_x,
            max_x,
            beacons,
        }
    }

    fn row_count(&self, y: isize) -> usize {
        let mut count = 0;
        let mut x = self.min_x;
        while x <= self.max_x {
            let point = Point { x, y };

            // If I'm a beacon, skip me
            if self.beacons.contains(&point) {
                x += 1;
                continue;
            }

            if let Some(pair) = self
                .pairs
                .iter()
                .find(|pair| point.manhattan_distance(&pair.sensor) <= pair.distance)
            {
                let point_to_sensor = point.manhattan_distance(&pair.sensor);
                let skip_steps: usize = pair.distance - point_to_sensor;

                count += skip_steps;
                x += 1 + skip_steps as isize;
                if x > self.max_x {
                    break;
                } else {
                    continue;
                }
            } else {
                return count;
            }
        }

        count
    }

    fn distress_beacon(&self, point_a: &Point, point_b: &Point) -> Option<Point> {
        let mut x = point_a.x;
        while x <= point_b.x {
            let mut y = point_a.y;
            while y <= point_b.y {
                let point = Point { x, y };

                // If I'm a beacon, skip me
                if self.beacons.contains(&point) {
                    y += 1;
                    if y > point_b.y {
                        break;
                    } else {
                        continue;
                    }
                }

                // If I'm within range of *any* sensor-beacon pair, count me
                if let Some(pair) = self
                    .pairs
                    .iter()
                    .find(|pair| point.manhattan_distance(&pair.sensor) <= pair.distance)
                {
                    let point_to_sensor = point.manhattan_distance(&pair.sensor);
                    let skip_steps: usize = pair.distance - point_to_sensor;

                    y += 1 + skip_steps as isize;
                    if y > point_b.y {
                        break;
                    } else {
                        continue;
                    }
                } else {
                    return Some(point);
                }
            }
            x += 1;
        }

        None
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let grid = Grid::from(input);
    grid.row_count(2000000)
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = Grid::from(input);

    let point_a = Point { x: 0, y: 0 };
    let point_b = Point { x: 4000000, y: 4000000 };

    match grid.distress_beacon(&point_a, &point_b) {
        Some(beacon_point) => beacon_point.x as usize * 4000000 + beacon_point.y as usize,
        None => panic!("Did not find beacon"),
    }
}

#[cfg(test)]
mod tests {
    use crate::year2022::day15::{Grid, Point};

    #[test]
    fn part1_test_input() {
        let grid = Grid::from(&include_str!("test_input"));
        assert_eq!(grid.row_count(10), 26);
    }

    #[test]
    fn part1_real_input() {
        let grid = Grid::from(&include_str!("input"));
        assert_eq!(grid.row_count(2000000), 4665948);
    }

    #[test]
    fn part2_test_input() {
        let point_a = Point { x: 0, y: 0 };
        let point_b = Point { x: 20, y: 20 };
        let grid = Grid::from(&include_str!("test_input"));
        let beacon_point: Point = grid.distress_beacon(&point_a, &point_b).unwrap();

        assert_eq!(beacon_point.x as usize * 4000000 + beacon_point.y as usize, 56000011,);
    }

    #[test]
    fn part2_real_input() {
        let point_a = Point { x: 0, y: 0 };
        let point_b = Point { x: 4000000, y: 4000000 };
        let grid = Grid::from(&include_str!("input"));
        let beacon_point: Point = grid.distress_beacon(&point_a, &point_b).unwrap();

        assert_eq!(
            beacon_point.x as usize * 4000000 + beacon_point.y as usize,
            13543690671045,
        );
    }
}
