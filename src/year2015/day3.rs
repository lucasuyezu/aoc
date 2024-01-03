use std::collections::HashSet;

use crate::utils::point::{Point, EAST, NORTH, SOUTH, WEST};

pub fn solve_part_1(input: &str) -> usize {
    let mut houses: HashSet<Point> = HashSet::new();

    let mut current_point = Point::origin();

    houses.insert(current_point);

    for c in input.trim().chars() {
        let direction = match c {
            '>' => EAST,
            'v' => SOUTH,
            '<' => WEST,
            '^' => NORTH,
            x => panic!("Invalid char {}", x),
        };

        current_point = current_point + direction;
        houses.insert(current_point);
    }

    houses.len()
}

pub fn solve_part_2(_: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&">"), 2);
        assert_eq!(super::solve_part_1(&"^>v<"), 4);
        assert_eq!(super::solve_part_1(&"^v^v^v^v^v"), 2);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day3/input")), 2_081);
    }

    #[test]
    fn part2_test_input() {
        // assert_eq!(super::solve_part_2(&include_str!("day3/test_input")), 1);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day3/input")), 1);
    }
}
