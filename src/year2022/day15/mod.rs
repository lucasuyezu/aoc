use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Hash, PartialEq, Eq, Copy)]
struct Point {
    x: isize,
    y: isize,
}

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
    sensors: HashSet<Point>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    target_y: isize,
}

fn dist(point_a: &Point, point_b: &Point) -> usize {
    (point_a.x - point_b.x).abs() as usize + (point_a.y - point_b.y).abs() as usize
}
impl Grid {
    fn from(input: &str, target_y: isize) -> Grid {
        let mut min_x = isize::MAX;
        let mut min_y = isize::MAX;
        let mut max_x = isize::MIN;
        let mut max_y = isize::MIN;

        let mut sensors: HashSet<Point> = HashSet::new();
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

                let distance = dist(&sensor, &beacon);

                sensors.insert(sensor);
                beacons.insert(beacon);

                Pair {
                    sensor,
                    beacon,
                    distance,
                }
            })
            .collect();

        pairs.iter().for_each(|pair| {
            if pair.sensor.x < min_x {
                min_x = pair.sensor.x;
            }

            if pair.sensor.x > max_x {
                max_x = pair.sensor.x;
            }

            if pair.sensor.y < min_y {
                min_y = pair.sensor.y;
            }

            if pair.sensor.y > max_y {
                max_y = pair.sensor.y;
            }

            if pair.beacon.x < min_x {
                min_x = pair.beacon.x;
            }

            if pair.beacon.x > max_x {
                max_x = pair.beacon.x;
            }

            if pair.beacon.y < min_y {
                min_y = pair.beacon.y;
            }

            if pair.beacon.y > max_y {
                max_y = pair.beacon.y;
            }
        });

        Grid {
            pairs,
            min_x,
            max_x,
            min_y,
            max_y,
            target_y,
            beacons,
            sensors,
        }
    }

    fn row_count(&self) -> usize {
        let mut point = Point {
            y: self.target_y,
            x: self.min_x,
        };
        let mut count = 0;
        for x in self.min_x..=self.max_x {
            point.x = x;

            // If I'm a beacon, skip me
            if self.beacons.contains(&point) {
                continue;
            }

            // If I'm within range of *any* sensor-beacon pair, count me
            if self
                .pairs
                .iter()
                .any(|pair| dist(&point, &pair.sensor) <= pair.distance)
            {
                count += 1;
            }
        }

        count
    }
}

pub fn solve_part_1(input: &str, target_y: isize) -> usize {
    let mut grid = Grid::from(input, target_y);

    dbg!(&grid.min_x);
    dbg!(&grid.max_x);
    // grid.populate();
    grid.row_count()
}

pub fn solve_part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input"), 10), 26);
    }

    #[test]
    fn part1_real_input() {
        // assert_eq!(super::solve_part_1(&include_str!("input"), 2000000), 4_299_549);
        // 4_299_549 is too low!
        assert_eq!(super::solve_part_1(&include_str!("input"), 2000000), 0);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 140);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 22184);
    }
}
