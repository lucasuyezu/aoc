use std::collections::HashSet;

use itertools::Itertools;

fn increment(s: &mut [char]) {
    let mut done = false;

    for x in (0..s.len()).rev() {
        if done {
            break;
        } else if s[x] == 'z' {
            s[x] = 'a';
        } else {
            s[x] = (s[x] as u8 + 1) as char;
            done = true;
        }
    }
}

fn is_pwd_valid(s: &[char]) -> bool {
    let mut pairs = HashSet::new();
    let mut consecutive_chars = false;

    for i in 0..s.len() {
        if s[i] == 'i' || s[i] == 'o' || s[i] == 'l' {
            return false;
        }

        if i < s.len() - 1 && s[i] == s[i + 1] {
            pairs.insert(s[i]);
        }

        if !consecutive_chars && i < s.len() - 2 {
            let a = s[i] as u8 + 2;
            let b = s[i + 1] as u8 + 1;
            let c = s[i + 2] as u8;

            consecutive_chars = a == b && b == c;
        }
    }

    consecutive_chars && pairs.len() >= 2
}

pub fn solve(s: &str) -> String {
    let mut cur_pwd = s.chars().collect_vec();
    loop {
        increment(&mut cur_pwd[..]);

        if is_pwd_valid(&cur_pwd) {
            return cur_pwd.iter().collect::<String>();
        }
    }
}

pub fn solve_part_1(input: &str) -> usize {
    solve(input);
    0
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input);
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::solve(&"cqjxjnds"), String::from("cqjxxyzz"));
    }

    #[test]
    fn part2() {
        assert_eq!(super::solve(&"cqjxxyzz"), String::from("cqkaabcc"));
    }
}
