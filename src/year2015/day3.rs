use std::collections::HashSet;

use crate::utils::point::Point;

pub fn solve_part_1(input: &str) -> usize {
    let mut houses: HashSet<Point> = HashSet::new();
    houses.insert(Point::origin());

    let mut current_point = Point::origin();

    for c in input.trim().chars() {
        current_point = current_point + Point::from_direction(c);
        houses.insert(current_point);
    }

    houses.len()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut houses: HashSet<Point> = HashSet::new();
    houses.insert(Point::origin());

    let mut santa_position = Point::origin();
    let mut robot_position = Point::origin();

    let mut santa_moves = true;

    for c in input.trim().chars() {
        let direction = Point::from_direction(c);

        if santa_moves {
            santa_position = santa_position + direction;
            houses.insert(santa_position);
        } else {
            robot_position = robot_position + direction;
            houses.insert(robot_position);
        }

        santa_moves = !santa_moves;
    }

    houses.len()
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
        assert_eq!(super::solve_part_2(&"^v"), 3);
        assert_eq!(super::solve_part_2(&"^>v<"), 3);
        assert_eq!(super::solve_part_2(&"^v^v^v^v^v"), 11);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day3/input")), 2_341);
    }
}
