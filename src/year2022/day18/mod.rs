use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

fn parse_input(input: &str) -> Vec<Point3D> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(",");

            Point3D {
                x: split.next().unwrap().parse::<usize>().unwrap() + 1,
                y: split.next().unwrap().parse::<usize>().unwrap() + 1,
                z: split.next().unwrap().parse::<usize>().unwrap() + 1,
            }
        })
        .collect()
}

pub fn solve_part_1(input: &str) -> usize {
    let points: Vec<Point3D> = parse_input(input);
    let mut visited: HashSet<(&Point3D, &Point3D)> = HashSet::new();

    let mut surface_area = points.len() * 6;

    for (i, from) in points.iter().enumerate() {
        for (j, to) in points.iter().enumerate() {
            if i == j {
                continue;
            }

            if visited.contains(&(from, to)) || visited.contains(&(to, from)) {
                continue;
            }

            visited.insert((from, to));

            if (from.x as isize == to.x as isize - 1 || from.x as isize - 1 == to.x as isize)
                && from.y == to.y
                && from.z == to.z
            {
                surface_area -= 2;
            }
            if (from.y as isize == to.y as isize - 1 || from.y as isize - 1 == to.y as isize)
                && from.x == to.x
                && from.z == to.z
            {
                surface_area -= 2;
            }
            if (from.z as isize == to.z as isize - 1 || from.z as isize - 1 == to.z as isize)
                && from.x == to.x
                && from.y == to.y
            {
                surface_area -= 2;
            }
        }
    }

    surface_area
}

pub fn solve_part_2(input: &str) -> usize {
    let points: Vec<Point3D> = parse_input(input);

    floodfill(points, Point3D { x: 0, y: 0, z: 0 })
}

fn floodfill(points: Vec<Point3D>, start: Point3D) -> usize {
    let max_x: usize = points.iter().map(|point| point.x).max().unwrap();
    let max_y: usize = points.iter().map(|point| point.y).max().unwrap();
    let max_z: usize = points.iter().map(|point| point.z).max().unwrap();

    let grid_size = max_x.max(max_y).max(max_z) + 2;

    let mut grid = vec![vec![vec![false; grid_size]; grid_size]; grid_size];

    for point in points {
        grid[point.x][point.y][point.z] = true;
    }

    let mut result = 0;

    let mut queue: VecDeque<Point3D> = VecDeque::new();
    let mut visited: HashSet<Point3D> = HashSet::new();
    queue.push_back(start);
    visited.insert(start);

    while let Some(point) = queue.pop_front() {
        for neighbour in neighbours(&point, &grid, &visited) {
            if grid[neighbour.x][neighbour.y][neighbour.z] {
                result += 1;
            } else {
                queue.push_back(neighbour);
                visited.insert(neighbour);
            }
        }
    }

    result
}

fn neighbours(
    point: &Point3D,
    grid: &Vec<Vec<Vec<bool>>>,
    visited: &HashSet<Point3D>,
) -> Vec<Point3D> {
    let mut result = vec![];

    if point.x < grid.len() - 1 {
        let point = Point3D {
            x: point.x + 1,
            y: point.y,
            z: point.z,
        };
        if !visited.contains(&point) {
            result.push(point);
        }
    }

    if point.x > 0 {
        let point = Point3D {
            x: point.x - 1,
            y: point.y,
            z: point.z,
        };
        if !visited.contains(&point) {
            result.push(point);
        }
    }

    if point.y < grid[point.x].len() - 1 {
        let point = Point3D {
            x: point.x,
            y: point.y + 1,
            z: point.z,
        };
        if !visited.contains(&point) {
            result.push(point);
        }
    }

    if point.y > 0 {
        let point = Point3D {
            x: point.x,
            y: point.y - 1,
            z: point.z,
        };
        if !visited.contains(&point) {
            result.push(point);
        }
    }

    if point.z < grid[point.x][point.y].len() - 1 {
        let point = Point3D {
            x: point.x,
            y: point.y,
            z: point.z + 1,
        };
        if !visited.contains(&point) {
            result.push(point);
        }
    }

    if point.z > 0 {
        let point = Point3D {
            x: point.x,
            y: point.y,
            z: point.z - 1,
        };
        if !visited.contains(&point) {
            result.push(point);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&"1,1,1\n2,1,1"), 10);
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 64);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 4512);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 58);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 2554);
    }
}
