use std::collections::HashSet;

fn solve(input: &str, streak_size: usize) -> usize {
    let mut result = 0;

    for i in 0..input.trim().len() - streak_size {
        let char_set: HashSet<char> = input[i..i + streak_size].chars().collect();
        if char_set.len() == streak_size {
            result = i + streak_size;
            break;
        }
    }

    result
}

pub fn solve_part_1(input: &str) -> usize {
    return solve(input, 4);
}

pub fn solve_part_2(input: &str) -> usize {
    return solve(input, 14);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        let results: Vec<usize> = include_str!("test_input")
            .lines()
            .map(|line| solve_part_1(line))
            .collect();
        assert_eq!(results, vec![7, 5, 6, 10, 11]);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(solve_part_1(&include_str!("input")), 1779);
    }

    #[test]
    fn part2_test_input() {
        let results: Vec<usize> = include_str!("test_input")
            .lines()
            .map(|line| solve_part_2(line))
            .collect();
        assert_eq!(results, vec![19, 23, 23, 29, 26]);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(solve_part_2(&include_str!("input")), 2635);
    }
}
