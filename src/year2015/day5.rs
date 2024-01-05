use fancy_regex::Regex;

pub fn solve_part_1(input: &str) -> usize {
    let x = Regex::new("ab|cd|pq|xy").unwrap();
    let y = Regex::new("([aeiou].*){3,}").unwrap();
    let z = Regex::new(r"(\w)(\1)").unwrap();

    input
        .lines()
        .filter(|line| !x.is_match(line).unwrap() && y.is_match(line).unwrap() && z.is_match(line).unwrap())
        .count()
}

pub fn solve_part_2(input: &str) -> usize {
    let x = fancy_regex::Regex::new(r"(\w\w).*(\1)").unwrap();
    let y = fancy_regex::Regex::new(r"(\w).(\1)").unwrap();

    input
        .lines()
        .filter(|line| x.is_match(line).unwrap() && y.is_match(line).unwrap())
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day5/test_input")), 2);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day5/input")), 236);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day5/test_input_part_2")), 2);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day5/input")), 51);
    }
}
