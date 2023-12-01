use regex::Regex;

pub fn solve_part_1(input: &str) -> usize {
    let re = Regex::new(r"\d").unwrap();

    return input
        .lines()
        .into_iter()
        .map(|line| {
            let digits: Vec<usize> = re
                .find_iter(line)
                .filter_map(|digit| digit.as_str().parse::<usize>().ok())
                .collect();

            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum();
}

pub fn solve_part_2(input: &str) -> usize {
    let first_re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();

    let tokens = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight",
        "nine",
    ];

    return input
        .lines()
        .into_iter()
        .map(|line| {
            let first_match = first_re.find(line).unwrap().as_str();

            let mut second_match = "oh_no";
            let mut idx = 0;

            for token in tokens.iter() {
                if let Some(current_idx) = line.rfind(token) {
                    if current_idx >= idx {
                        idx = current_idx;
                        second_match = token;
                    }
                }
            }

            string_to_digit(first_match) * 10 + string_to_digit(second_match)
        })
        .sum();
}

fn string_to_digit(input: &str) -> usize {
    match input {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        x => x.parse::<usize>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input_part_1")), 142);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 56_506);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input_part_2")), 292);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 56_017);
    }
}
