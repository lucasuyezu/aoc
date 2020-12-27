use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct LoginPolicy {
    min: usize,
    max: usize,
    req_char: char,
    password: String,
}

impl LoginPolicy {
    fn is_valid_part_1(&self) -> bool {
        let matches: Vec<&str> = self.password.matches(self.req_char).collect();
        let matches_count = matches.len();

        return matches_count >= self.min && matches_count <= self.max
    }

    fn is_valid_part_2(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();

        // println!("login={:?} chars[{}]={} chars[{}]", self, self.min-1, chars[self.min-1], self.max-1);
        return (chars[self.min-1] == self.req_char) ^ (chars[self.max-1] == self.req_char);
    }
}

pub fn solve_part_1(filename: &str) -> usize {
    let valid_login_policies: Vec<LoginPolicy> = login_policies(filename)
        .into_iter()
        .filter(|it| it.is_valid_part_1())
        .collect();

    return valid_login_policies.len();
}

pub fn solve_part_2(filename: &str) -> usize {
    let valid_login_policies: Vec<LoginPolicy> = login_policies(filename)
        .into_iter()
        .filter(|it| it.is_valid_part_2())
        .collect();

    return valid_login_policies.len();
}

fn login_policies(filename: &str) -> Vec<LoginPolicy> {
    let passwords_policy_lines = get_lines(filename);
    let mut login_policies: Vec<LoginPolicy> = Vec::new();

    // Build login structs
    for password_policy_line in passwords_policy_lines {
        let idx_low = 0;
        let idx_high = password_policy_line.find('-').unwrap();

        let min = password_policy_line[idx_low..idx_high].to_string().parse::<usize>().unwrap();
        // println!("idx_low={} idx_max={} min={}", idx_low, idx_high, min);

        let idx_low = idx_high + 1;
        let idx_high = password_policy_line.find(' ').unwrap();

        let max = password_policy_line[idx_low..idx_high].to_string().parse::<usize>().unwrap();
        // println!("idx_low={} idx_max={} max={}", idx_low, idx_high, max);

        let idx_low = idx_high + 1;
        let idx_high = password_policy_line.find(':').unwrap();

        let req_char = password_policy_line[idx_low..idx_low+1].to_string().chars().next().unwrap();
        // println!("idx_low={} idx_max={} req_char={}", idx_low, idx_high, req_char);

        let idx_low = idx_high + 2;

        let password = password_policy_line[idx_low..].to_string();
        // println!("idx_low={} idx_max={} pasword={}", idx_low, idx_high, password);

        login_policies.push(LoginPolicy { min: min, max: max, req_char: req_char, password: password });
    }

    // println!("login_policies={:?}", login_policies);
    return login_policies;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_lines(filename: &str) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    if let Ok(file_lines) = read_lines(filename) {
        for file_line in file_lines {
            if let Ok(line_string) = file_line {
                lines.push(line_string);
            }
        }
    } else {
        println!("Error reading file {}", filename);
    }

    return lines;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1("/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day2/test_input"), 3);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1("/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day2/input"), 393);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2("/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day2/test_input"), 2);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2("/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day2/input"), 690);
    }
}
