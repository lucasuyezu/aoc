pub fn solve_part_1(input: &str) -> usize {
    return input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|values| values[1] > values[0])
        .count();
}

pub fn solve_part_2(input: &str) -> usize {
    return input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .windows(3)
        .map(|values| values[0] + values[1] + values[2])
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|values| values[1] > values[0])
        .count();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 7);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 1462);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 5);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 1497);
    }
}
