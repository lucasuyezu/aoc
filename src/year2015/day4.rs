fn solve(input: &str, prefix: &str) -> usize {
    for i in 1..usize::MAX {
        let digest = md5::compute(format!("{}{}", input, i));

        if format!("{:x}", digest).starts_with(prefix) {
            return i;
        }
    }

    panic!()
}

pub fn solve_part_1(input: &str) -> usize {
    solve(input, "00000")
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input, "000000")
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1("abcdef"), 609_043);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1("bgvyzdsv"), 254_575);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2("bgvyzdsv"), 1_038_736);
    }
}
