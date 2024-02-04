use itertools::Itertools;
use regex::Regex;
use serde_json::Value;

pub fn solve_part_1(input: &str) -> isize {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(input).map(|x| x.as_str().parse::<isize>().unwrap()).sum()
}

pub fn solve_part_2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| extract_value(&serde_json::from_str(line).unwrap()))
        .sum()
}

fn extract_value(value: &Value) -> i64 {
    match value {
        Value::Bool(_) | Value::String(_) | Value::Null => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|v| extract_value(v)).sum(),
        Value::Object(obj) => {
            if obj.values().contains(&serde_json::json!("red")) {
                0
            } else {
                obj.values().map(|v| extract_value(v)).sum()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day12/test_input")), 18);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day12/input")), 191_164);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day12/test_input_part_2")), 16);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day12/input")), 87_842);
    }
}
