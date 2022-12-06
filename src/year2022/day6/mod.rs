use std::collections::HashSet;

fn solve(lines: &[String], streak_size: usize) -> Vec<usize> {
    return lines
        .iter()
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

pub fn solve_part_1(lines: &[String]) -> Vec<usize> {
    return solve(lines, 4);
}

pub fn solve_part_2(lines: &[String]) -> Vec<usize> {
    return solve(lines, 14);
}

#[cfg(test)]
mod tests {
    use super::super::super::utils;

    #[test]
    fn part1_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day6/test_input".to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), vec![7, 5, 6, 10, 11]);
    }

    #[test]
    fn part1_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day6/input".to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), vec![1779]);
    }

    #[test]
    fn part2_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day6/test_input".to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), vec![19, 23, 23, 29, 26]);
    }

    #[test]
    fn part2_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day6/input".to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), vec![2635]);
    }
}
