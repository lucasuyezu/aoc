mod maze;

use crate::year2023::day10::maze::Maze;

pub fn solve_part_1(input: &str) -> usize {
    let visited_points = input.parse::<Maze>().unwrap().walk();
    visited_points.len() / 2
}

pub fn solve_part_2(input: &str) -> usize {
    let maze = input.parse::<Maze>().unwrap();
    let visited_points = maze.walk();
    maze.enclosed_points_count(visited_points)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_square_loop() {
        assert_eq!(super::solve_part_1(&include_str!("test_input_square_loop")), 4);
    }

    #[test]
    fn part1_test_complex_loop() {
        assert_eq!(super::solve_part_1(&include_str!("test_input_complex_loop")), 8);
    }
    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 6754);
    }

    #[test]
    fn part2_test_input_small() {
        assert_eq!(super::solve_part_2(&include_str!("test_input_part_2_small")), 4);
    }

    #[test]
    fn part2_test_input_medium() {
        assert_eq!(super::solve_part_2(&include_str!("test_input_part_2_medium")), 8);
    }

    #[test]
    fn part2_test_input_large() {
        assert_eq!(super::solve_part_2(&include_str!("test_input_part_2_large")), 10);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 567);
    }
}
