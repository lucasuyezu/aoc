pub fn solve_part_1(input: &str) -> usize {
    return input
        .lines()
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

pub fn solve_part_2(input: &str) -> usize {
    return input
        .lines()
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
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 2);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 450);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 4);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 837);
    }
}
