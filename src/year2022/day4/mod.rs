pub fn solve_part_1(lines: &[String]) -> usize {
    return lines
        .iter()
        .filter(|line| {
            let mut elf_strings = line.split(",");

            let elf_1_str = elf_strings.next().unwrap();
            let mut elf_1_range_str = elf_1_str.split("-");
            let range_1_start: usize = elf_1_range_str.next().unwrap().parse().unwrap();
            let range_1_end: usize = elf_1_range_str.next().unwrap().parse().unwrap();

            let elf_2_str = elf_strings.next().unwrap();
            let mut elf_2_range_str = elf_2_str.split("-");
            let range_2_start: usize = elf_2_range_str.next().unwrap().parse().unwrap();
            let range_2_end: usize = elf_2_range_str.next().unwrap().parse().unwrap();

            (range_1_start >= range_2_start && range_1_end <= range_2_end)
                || (range_2_start >= range_1_start && range_2_end <= range_1_end)
        })
        .count();
}

pub fn solve_part_2(lines: &[String]) -> usize {
    return lines
        .iter()
        .filter(|line| {
            let mut elf_strings = line.split(",");

            let elf_1_str = elf_strings.next().unwrap();
            let mut elf_1_range_str = elf_1_str.split("-");
            let range_1_start: usize = elf_1_range_str.next().unwrap().parse().unwrap();
            let range_1_end: usize = elf_1_range_str.next().unwrap().parse().unwrap();

            let elf_2_str = elf_strings.next().unwrap();
            let mut elf_2_range_str = elf_2_str.split("-");
            let range_2_start: usize = elf_2_range_str.next().unwrap().parse().unwrap();
            let range_2_end: usize = elf_2_range_str.next().unwrap().parse().unwrap();

            range_1_start <= range_2_end && range_1_end >= range_2_start
        })
        .count();
}

#[cfg(test)]
mod tests {
    use super::super::super::utils;

    #[test]
    fn part1_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day4/test_input".to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 2);
    }

    #[test]
    fn part1_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day4/input".to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 450);
    }

    #[test]
    fn part2_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day4/test_input".to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 4);
    }

    #[test]
    fn part2_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day4/input".to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 837);
    }
}
