pub fn solve_part_1(input: &str) -> Result<isize, String> {
    let mut open_parens_count = 0;
    let mut close_parens_count = 0;

    for c in input.chars() {
        match c {
            '(' => open_parens_count += 1,
            ')' => close_parens_count += 1,
            _ => return Err(format!("Invalid char {}", c)),
        }
    }

    return Ok(open_parens_count - close_parens_count);
}

pub fn solve_part_2(input: &str) -> Result<usize, String> {
    let mut current_floor = 0;
    let result = input.chars().enumerate().find(|(_, c)| {
        current_floor += if *c == '(' { 1 } else { -1 };
        current_floor < 0
    });

    return match result {
        Some((index, _)) => Ok(index + 1),
        None => Err(format!("Did not reach basement")),
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_inputs() {
        assert_eq!(super::solve_part_1("(())"), Ok(0));
        assert_eq!(super::solve_part_1("()()"), Ok(0));
        assert_eq!(super::solve_part_1("((("), Ok(3));
        assert_eq!(super::solve_part_1("(()(()("), Ok(3));
        assert_eq!(super::solve_part_1("))((((("), Ok(3));
        assert_eq!(super::solve_part_1("())"), Ok(-1));
        assert_eq!(super::solve_part_1("))("), Ok(-1));
        assert_eq!(super::solve_part_1(")))"), Ok(-3));
        assert_eq!(super::solve_part_1(")())())"), Ok(-3));
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), Ok(74));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(")"), Ok(1));
        assert_eq!(super::solve_part_2("()())"), Ok(5));
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), Ok(1_795));
    }
}
