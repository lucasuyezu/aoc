pub fn solve_part_1(input: &str) -> isize {
    let mut open_parens_count = 0;
    let mut close_parens_count = 0;

    for c in input.chars() {
        match c {
            '(' => open_parens_count += 1,
            ')' => close_parens_count += 1,
            _ => panic!("Invalid char {}", c),
        }
    }

    open_parens_count - close_parens_count
}

pub fn solve_part_2(input: &str) -> usize {
    let mut current_floor = 0;
    let result = input.chars().enumerate().find(|(_, c)| {
        current_floor += if *c == '(' { 1 } else { -1 };
        current_floor < 0
    });

    match result {
        Some((index, _)) => index + 1,
        None => panic!("Did not reach basement"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_inputs() {
        assert_eq!(super::solve_part_1("(())"), 0);
        assert_eq!(super::solve_part_1("()()"), 0);
        assert_eq!(super::solve_part_1("((("), 3);
        assert_eq!(super::solve_part_1("(()(()("), 3);
        assert_eq!(super::solve_part_1("))((((("), 3);
        assert_eq!(super::solve_part_1("())"), -1);
        assert_eq!(super::solve_part_1("))("), -1);
        assert_eq!(super::solve_part_1(")))"), -3);
        assert_eq!(super::solve_part_1(")())())"), -3);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 74);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(")"), 1);
        assert_eq!(super::solve_part_2("()())"), 5);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 1_795);
    }
}
