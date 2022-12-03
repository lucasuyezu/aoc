use std::collections::HashSet;

pub fn solve_part_1(lines: &[String]) -> usize {
    return lines
        .iter()
        .map(|it| (it.split_at(it.len()/2)))
        .map(|it| {
            let a: HashSet<char> = it.0.chars().into_iter().collect();
            let b: HashSet<char> = it.1.chars().into_iter().collect();

            a.intersection(&b).copied().next().unwrap()
        })
        .map(|it| {
            let ascii = it as usize;
            if ascii >= 97 {
                ascii - 96
            }
            else {
                ascii - 38
            }
        })
        .sum();
}

pub fn solve_part_2(lines: &[String]) -> usize {
    return lines
        .chunks(3)
        .map(|chunk| {
            let a: HashSet<char> = chunk[0].chars().into_iter().collect();
            let b: HashSet<char> = chunk[1].chars().into_iter().collect();
            let c: HashSet<char> = chunk[2].chars().into_iter().collect();

            let temp: HashSet<char> = a.intersection(&b).copied().collect();
            temp.intersection(&c).copied().next().unwrap()
        })
        .map(|it| {
            let ascii = it as usize;
            if ascii >= 97 {
                ascii - 96
            }
            else {
                ascii - 38
            }
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use super::super::super::utils;

    #[test]
    fn part1_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day3/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 157);
    }

    #[test]
    fn part1_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day3/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 8_018);
    }

    #[test]
    fn part2_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day3/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 70);
    }

    #[test]
    fn part2_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day3/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 2_518);
    }
}
