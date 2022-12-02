pub fn solve_part_1(lines: &[String]) -> usize {
    let mut acc = 0;
    let mut max = 0;
    for line in lines.iter() {
        if line != "" {
            acc += line.parse::<usize>().unwrap();
        }
        else {
            if acc > max {
                max = acc;
            }
            acc = 0;
        }
    }
    
    if acc > max {
        max = acc;
    }

    return max;
}

pub fn solve_part_2(lines: &[String]) -> usize {
    let mut vec: Vec<usize> = Vec::new();
    let mut acc = 0;
    for line in lines.iter() {
        if line != "" {
            acc += line.parse::<usize>().unwrap();
        }
        else {
            vec.push(acc);
            acc = 0;
        }
    }

    vec.push(acc);
    
    vec.sort_unstable();
    vec.reverse();
    return vec[0] + vec[1] + vec[2];
}

#[cfg(test)]
mod tests {
    use super::super::super::utils;

    #[test]
    fn part1_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day1/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 24_000);
    }

    #[test]
    fn part1_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day1/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 69_883);
    }

    #[test]
    fn part2_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day1/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 45_000);
    }

    #[test]
    fn part2_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day1/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 207_576);
    }
}
