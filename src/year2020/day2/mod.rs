#[derive(Debug)]
pub struct LoginPolicy {
    min: usize,
    max: usize,
    req_char: char,
    password: String,
}

impl LoginPolicy {
    fn is_valid_part_1(&self) -> bool {
        let matches_count = self.password.matches(self.req_char).count();

        matches_count >= self.min && matches_count <= self.max
    }

    fn is_valid_part_2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();

        // println!("login={:?} chars[{}]={} chars[{}]", self, self.min-1, chars[self.min-1], self.max-1);
        (chars[self.min - 1] == self.req_char) ^ (chars[self.max - 1] == self.req_char)
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let numbers = parse_input(input);

    return numbers.iter().filter(|it| it.is_valid_part_1()).count();
}

pub fn solve_part_2(input: &str) -> usize {
    let numbers = parse_input(input);

    return numbers.iter().filter(|it| it.is_valid_part_2()).count();
}

fn parse_input(input: &str) -> Vec<LoginPolicy> {
    let mut login_policies: Vec<LoginPolicy> = Vec::new();

    // Build login structs
    for password_policy_line in input.lines() {
        let idx_low = 0;
        let idx_high = password_policy_line.find('-').unwrap();

        let min = password_policy_line[idx_low..idx_high]
            .to_string()
            .parse::<usize>()
            .unwrap();
        // println!("idx_low={} idx_max={} min={}", idx_low, idx_high, min);

        let idx_low = idx_high + 1;
        let idx_high = password_policy_line.find(' ').unwrap();

        let max = password_policy_line[idx_low..idx_high]
            .to_string()
            .parse::<usize>()
            .unwrap();
        // println!("idx_low={} idx_max={} max={}", idx_low, idx_high, max);

        let idx_low = idx_high + 1;
        let idx_high = password_policy_line.find(':').unwrap();

        let req_char = password_policy_line[idx_low..idx_low + 1]
            .to_string()
            .chars()
            .next()
            .unwrap();
        // println!("idx_low={} idx_max={} req_char={}", idx_low, idx_high, req_char);

        let idx_low = idx_high + 2;

        let password = password_policy_line[idx_low..].to_string();
        // println!("idx_low={} idx_max={} pasword={}", idx_low, idx_high, password);

        login_policies.push(LoginPolicy {
            min,
            max,
            req_char,
            password,
        });
    }

    // println!("login_policies={:?}", login_policies);
    login_policies
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 3);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 393);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 2);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 690);
    }
}
