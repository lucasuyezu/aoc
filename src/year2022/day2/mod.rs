pub fn solve_part_1(lines: &[String]) -> usize {
    let mut result = 0;
    
    for line in lines {
        match line.as_str() {
            "A X" => result += 1 + 3,
            "A Y" => result += 2 + 6,
            "A Z" => result += 3 + 0,
            "B X" => result += 1 + 0,
            "B Y" => result += 2 + 3,
            "B Z" => result += 3 + 6,
            "C X" => result += 1 + 6,
            "C Y" => result += 2 + 0,
            "C Z" => result += 3 + 3,
            _ => panic!("Invalid line {}", line),
        }
    }

    return result;
}

pub fn solve_part_2(lines: &[String]) -> usize {
    let mut result = 0;
    
    for line in lines {
        match line.as_str() {
            "A X" => result += 3 + 0,
            "A Y" => result += 1 + 3,
            "A Z" => result += 2 + 6,
            "B X" => result += 1 + 0,
            "B Y" => result += 2 + 3,
            "B Z" => result += 3 + 6,
            "C X" => result += 2 + 0,
            "C Y" => result += 3 + 3,
            "C Z" => result += 1 + 6,
            _ => panic!("Invalid line {}", line),
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::super::super::utils;

    #[test]
    fn part1_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day2/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 15);
    }

    #[test]
    fn part1_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day2/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 11_603);
    }

    #[test]
    fn part2_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day2/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 12);
    }

    #[test]
    fn part2_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day2/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 12_725);
    }
}
