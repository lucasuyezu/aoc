pub fn solve_part_1(input: &str) -> usize {
    0
}

pub fn solve_part_2(_: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day/test_input")), 1);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day/input")), 1);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day/test_input")), 1);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day/input")), 1);
    }
}
