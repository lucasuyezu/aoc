mod maze;

use crate::year2023::day10::maze::Maze;

pub fn solve_part_1(input: &str) -> usize {
    input.parse::<Maze>().unwrap().walk()
}

pub fn solve_part_2(_input: &str) -> usize {
    0
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
        // 75 is wrong
        assert_eq!(super::solve_part_1(&include_str!("input")), 6754);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input_square_loop")), 71_503);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 34_655_848);
    }
}
