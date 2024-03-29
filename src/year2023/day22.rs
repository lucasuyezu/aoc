use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}
impl Point3D {
    fn is_on_ground(&self) -> bool {
        self.z == 1
    }

    fn is_on_top_of(&self, other: &Point3D) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z + 1
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Brick {
    id: usize,
    cubes: Vec<Point3D>,
}

impl Brick {
    fn is_on_ground(&self) -> bool {
        self.cubes.iter().any(|cube| cube.is_on_ground())
    }

    fn is_on_top_of(&self, other: &Brick) -> bool {
        self.cubes
            .iter()
            .any(|cube| other.cubes.iter().any(|other_cube| cube.is_on_top_of(other_cube)))
    }
}

fn parse_input(input: &str) -> (Vec<Brick>, Vec<Vec<Vec<Option<usize>>>>) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    let bricks = input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let (cube_a_str, cube_b_str) = line.split_once("~").unwrap();

            let mut cube_a_split = cube_a_str.split(",");
            let mut cube_b_split = cube_b_str.split(",");

            let cube_a = Point3D {
                x: cube_a_split.next().unwrap().parse().unwrap(),
                y: cube_a_split.next().unwrap().parse().unwrap(),
                z: cube_a_split.next().unwrap().parse().unwrap(),
            };
            let cube_b = Point3D {
                x: cube_b_split.next().unwrap().parse().unwrap(),
                y: cube_b_split.next().unwrap().parse().unwrap(),
                z: cube_b_split.next().unwrap().parse().unwrap(),
            };

            max_x = max_x.max(cube_a.x).max(cube_b.x);
            max_y = max_y.max(cube_a.y).max(cube_b.y);
            max_z = max_z.max(cube_a.z).max(cube_b.z);

            let mut cubes: Vec<Point3D> = vec![];

            if cube_a.x != cube_b.x {
                for x in cube_a.x.min(cube_b.x)..=cube_a.x.max(cube_b.x) {
                    cubes.push(Point3D {
                        x,
                        y: cube_a.y,
                        z: cube_a.z,
                    })
                }
            }

            if cube_a.y != cube_b.y {
                for y in cube_a.y.min(cube_b.y)..=cube_a.y.max(cube_b.y) {
                    cubes.push(Point3D {
                        x: cube_a.x,
                        y,
                        z: cube_a.z,
                    })
                }
            }

            if cube_a.z != cube_b.z {
                for z in cube_a.z.min(cube_b.z)..=cube_a.z.max(cube_b.z) {
                    cubes.push(Point3D {
                        x: cube_a.x,
                        y: cube_a.y,
                        z,
                    })
                }
            }

            if cubes.is_empty() {
                cubes.push(cube_a);
            }

            Brick { id, cubes }
        })
        .sorted_by_key(|brick| brick.cubes.iter().map(|cube| cube.z).min())
        .collect_vec();

    let mut grid = vec![vec![vec![None; max_z + 1]; max_y + 1]; max_x + 1];

    for brick in bricks.iter() {
        for cube in brick.cubes.iter() {
            grid[cube.x][cube.y][cube.z] = Some(brick.id);
        }
    }

    (bricks, grid)
}

pub fn simulate(input: &str) -> (HashMap<usize, HashSet<usize>>, HashSet<usize>) {
    let (mut bricks, mut grid) = parse_input(input);

    for brick in bricks.iter_mut() {
        let mut is_at_rest = false;

        while !is_at_rest {
            if brick.is_on_ground() {
                is_at_rest = true;
                continue;
            }

            if brick.cubes.iter().any(|cube| {
                let cube_below = grid[cube.x][cube.y][cube.z - 1];
                cube_below.is_some() && cube_below.unwrap() != brick.id
            }) {
                is_at_rest = true;
                continue;
            }

            for cube in brick.cubes.iter_mut() {
                grid[cube.x][cube.y][cube.z] = None;
                grid[cube.x][cube.y][cube.z - 1] = Some(brick.id);
                cube.z -= 1;
            }
        }
    }

    // All bricks are at rest now

    let mut dependency_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut safe_for_disintegration: HashSet<usize> = HashSet::new();

    for brick in bricks.iter() {
        dependency_map.insert(brick.id, HashSet::new());
        safe_for_disintegration.insert(brick.id);
    }

    for outer_brick in bricks.iter() {
        for inner_brick in bricks.iter() {
            if outer_brick != inner_brick && inner_brick.is_on_top_of(&outer_brick) {
                dependency_map.entry(inner_brick.id).and_modify(|v| {
                    v.insert(outer_brick.id);
                });
            }
        }
    }

    for v in dependency_map.values() {
        if v.len() == 1 {
            safe_for_disintegration.remove(&v.iter().next().unwrap());
        }
    }

    (dependency_map, safe_for_disintegration)
}

pub fn solve_part_1(input: &str) -> usize {
    simulate(input).1.len()
}

pub fn solve_part_2(input: &str) -> usize {
    let dependency_map = simulate(input).0;

    dependency_map
        .par_iter()
        .map(|(brick_id, _v)| {
            let mut bricks_dropped: HashSet<usize> = HashSet::new();
            bricks_dropped.insert(*brick_id);

            let mut x = dependency_map.clone();

            let mut stop = false;
            while !stop {
                let keys_empty_before: HashSet<usize> =
                    x.iter().filter(|(_, v)| v.is_empty()).map(|(k, _)| k.clone()).collect();

                for brick_dropped in bricks_dropped.iter() {
                    for v in x.values_mut() {
                        v.remove(&brick_dropped);
                    }
                }

                let keys_empty_after: HashSet<usize> =
                    x.iter().filter(|(_, v)| v.is_empty()).map(|(k, _)| k.clone()).collect();

                let keys_removed: HashSet<&usize> = keys_empty_after
                    .iter()
                    .filter(|key| !keys_empty_before.contains(key))
                    .collect();

                if keys_removed.is_empty() {
                    stop = true;
                }

                for k in keys_removed {
                    bricks_dropped.insert(*k);
                }
            }

            bricks_dropped.len() - 1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day22/test_input")), 5);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day22/input")), 488);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day22/test_input")), 7);
    }

    #[test]
    fn part2_real_input() {
        // This takes 11s to run in --release mode, doesn't finish in test mode.
        assert_eq!(super::solve_part_2(&include_str!("day22/input")), 79_465);
    }
}
