use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut in_memory_len = 0;

            let mut chars = line.chars().peekable();
            while let Some(_) = chars.peek() {
                let c = chars.next().unwrap();

                match c {
                    '"' => (),
                    '\\' => {
                        in_memory_len += 1;
                        if chars.peek() == Some(&'x') {
                            chars.next();
                            chars.next();
                            chars.next();
                        } else if chars.peek() == Some(&'"') || chars.peek() == Some(&'\\') {
                            chars.next();
                        }
                    }
                    _ => in_memory_len += 1,
                }
            }

            line.len() - in_memory_len
        })
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut new_line = String::new();
            new_line.push('"');

            let chars = line.chars().collect_vec();
            let mut idx = 0;

            while let Some(c) = chars.get(idx) {
                idx += 1;

                match c {
                    '"' => new_line.push_str("\\\""),
                    '\\' => {
                        new_line.push_str("\\\\");
                        if idx < chars.len()
                            && chars[idx] == 'x'
                            && idx + 1 < chars.len()
                            && chars[idx + 1].is_digit(16)
                            && idx + 2 < chars.len()
                            && chars[idx + 2].is_digit(16)
                        {
                            new_line.push(chars[idx]);
                            new_line.push(chars[idx + 1]);
                            new_line.push(chars[idx + 2]);
                            idx += 3;
                        } else if idx < chars.len() && chars[idx] == '"' {
                            new_line.push_str("\\\"");
                            idx += 1;
                        }
                    }
                    _ => {
                        new_line.push(*c);
                    }
                }
            }

            new_line.push('"');

            new_line.len() - line.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day8/test_input")), 12);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day8/input")), 1_350);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day8/test_input")), 19);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day8/input")), 2_085);
    }
}
