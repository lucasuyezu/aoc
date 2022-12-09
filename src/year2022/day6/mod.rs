use std::collections::HashSet;

fn solve(input: &str, streak_size: usize) -> Vec<usize> {
    return input
        .lines()
        .map(|line| {
            let mut result = 0;

            for i in 0..line.len() - streak_size {
                let char_set: HashSet<char> = line[i..i + streak_size].chars().collect();
                if char_set.len() == streak_size {
                    result = i + streak_size;
                    break;
                }
            }

            result
        })
        .collect();
}

pub fn solve_part_1(input: &str) -> Vec<usize> {
    return solve(input, 4);
}

pub fn solve_part_2(input: &str) -> Vec<usize> {
    return solve(input, 14);
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(
            super::solve_part_1(&include_str!("test_input")),
            vec![7, 5, 6, 10, 11]
        );
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), vec![1779]);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            super::solve_part_2(&include_str!("test_input")),
            vec![19, 23, 23, 29, 26]
        );
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), vec![2635]);
    }
}
