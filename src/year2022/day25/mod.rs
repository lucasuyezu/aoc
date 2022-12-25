use crate::utils::snafu::Snafu;

pub fn solve_part_1(input: &str) -> String {
    let mut result = Snafu { value: 0 };

    for line in input.lines() {
        result += line.parse::<Snafu>().unwrap();
    }

    result.to_string()
}

pub fn solve_part_2(_input: &str) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part_1(&include_str!("test_input")), "2=-1=0");
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(solve_part_1(&include_str!("input")), "2=222-2---22=1=--1-2");
    }
}
