pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            dbg!(&line);
            let code_len = line.len();
            dbg!(&code_len);

            let mut in_memory_len = 0;

            let mut chars = line.chars().peekable();

            while let Some(_) = chars.peek() {
                let c = chars.next().unwrap();
                dbg!(&c);

                match c {
                    '"' => (),
                    '\\' => {
                        in_memory_len += 1;
                        println!("pah");
                        if chars.peek() == Some(&'x') {
                            chars.next();
                            chars.next();
                            chars.next();
                        } else if chars.peek() == Some(&'"') || chars.peek() == Some(&'\\') {
                            chars.next();
                        }
                    }
                    _ => {
                        println!("pah");
                        in_memory_len += 1
                    }
                }
            }

            dbg!(&in_memory_len);
            let result = code_len - in_memory_len;
            dbg!(&result);

            result
        })
        .sum()
}

pub fn solve_part_2(_: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day8/test_input")), 30);
    }

    #[test]
    fn part1_real_input() {
        // 1251 is too low
        assert_eq!(super::solve_part_1(&include_str!("day8/input")), 1);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day8/test_input")), 1);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day8/input")), 1);
    }
}
