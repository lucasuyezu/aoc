use std::collections::HashSet;

use rayon::prelude::*;

use crate::utils::{
    grid::Grid,
    point::{Point, EAST, NORTH, SOUTH, WEST},
};

#[derive(Debug)]
struct Beam {
    pos: Point,
    direction: Point,
}

fn should_split(direction: Point, tile: char) -> bool {
    match tile {
        '.' | '/' | '\\' => false,
        '|' => match direction {
            EAST | WEST => true,
            _ => false,
        },
        '-' => match direction {
            NORTH | SOUTH => true,
            _ => false,
        },
        _ => panic!("Invalid direction"),
    }
}

fn new_direction(direction: Point, tile: char) -> Point {
    match tile {
        '.' => direction,
        '/' => match direction {
            NORTH => EAST,
            EAST => NORTH,
            SOUTH => WEST,
            WEST => SOUTH,
            _ => panic!("Invalid tile"),
        },
        '\\' => match direction {
            NORTH => WEST,
            EAST => SOUTH,
            SOUTH => EAST,
            WEST => NORTH,
            _ => panic!("Invalid tile"),
        },
        '|' => match direction {
            NORTH | SOUTH => direction,
            _ => panic!("Invalid tile"),
        },
        '-' => match direction {
            EAST | WEST => direction,
            _ => panic!("Invalid tile"),
        },
        _ => panic!("Invalid tile"),
    }
}

fn energize(grid: &Grid<char>, start: Point, direction: Point) -> usize {
    let mut energized_tiles: HashSet<Point> = HashSet::new();
    let mut splitter_cache: HashSet<Point> = HashSet::new();
    let mut beams = vec![Beam {
        pos: start,
        direction: direction,
    }];

    while beams.iter().any(|beam| grid.is_inside(&beam.pos)) {
        let mut new_beams = vec![];
        for beam in beams.iter_mut().filter(|beam| grid.is_inside(&beam.pos)) {
            let tile = grid.data[beam.pos.x as usize][beam.pos.y as usize];
            energized_tiles.insert(beam.pos);

            if should_split(beam.direction, tile) {
                if !splitter_cache.contains(&beam.pos) {
                    splitter_cache.insert(beam.pos);
                    let mut beam_a = Beam {
                        pos: beam.pos,
                        direction: beam.direction,
                    };
                    let mut beam_b = Beam {
                        pos: beam.pos,
                        direction: beam.direction,
                    };
                    if tile == '|' {
                        beam_a.direction = NORTH;
                        beam_b.direction = SOUTH;
                    } else {
                        beam_a.direction = WEST;
                        beam_b.direction = EAST;
                    }
                    beam_a.pos = beam_a.pos + beam_a.direction;
                    beam_b.pos = beam_b.pos + beam_b.direction;

                    if grid.is_inside(&beam_a.pos) {
                        new_beams.push(beam_a);
                    }

                    if grid.is_inside(&beam_b.pos) {
                        new_beams.push(beam_b);
                    }
                }

                beam.pos = Point { x: -1, y: -1 };
            } else {
                beam.direction = new_direction(beam.direction, tile);
                beam.pos = beam.pos + beam.direction;
            }
        }

        beams.append(&mut new_beams);
    }

    energized_tiles.len()
}

pub fn solve_part_1(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();
    energize(&grid, Point { x: 0, y: 0 }, EAST)
}

pub fn solve_part_2(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();

    let mut grid_starting_positions = vec![];

    for x in 0..grid.x_len {
        grid_starting_positions.push((Point { x: x as isize, y: 0 }, EAST));
        grid_starting_positions.push((
            Point {
                x: x as isize,
                y: grid.y_len as isize - 1,
            },
            WEST,
        ));
    }

    for y in 0..grid.y_len {
        grid_starting_positions.push((Point { x: 0, y: y as isize }, SOUTH));
        grid_starting_positions.push((
            Point {
                x: grid.x_len as isize - 1,
                y: y as isize,
            },
            NORTH,
        ));
    }

    grid_starting_positions
        .par_iter()
        .map(|(starting_pos, direction)| energize(&grid, *starting_pos, *direction))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day16/test_input")), 46);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day16/input")), 6_883);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day16/test_input")), 51);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day16/input")), 7_228);
    }
}
