use std::collections::{HashMap, HashSet};

use regex::Regex;

lazy_static! {
    static ref VAL_RE: Regex = Regex::new(r"\A(\d+)\z").unwrap();
    static ref AND_RE: Regex = Regex::new(r"\A(\w+) AND (\w+)\z").unwrap();
    static ref OR_RE: Regex = Regex::new(r"\A(\w+) OR (\w+)\z").unwrap();
    static ref LSHIFT_RE: Regex = Regex::new(r"\A(\w+) LSHIFT (\d+)").unwrap();
    static ref RSHIFT_RE: Regex = Regex::new(r"\A(\w+) RSHIFT (\d+)").unwrap();
    static ref NOT_RE: Regex = Regex::new(r"\ANOT (\w+)\z").unwrap();
    static ref WRITE_RE: Regex = Regex::new(r"\A([a-zA-Z]+)\z").unwrap();
}

fn deps(line: &str) -> HashSet<String> {
    let mut result = HashSet::new();

    if let Some(captures) = AND_RE.captures(line) {
        if captures[1].parse::<usize>().is_err() {
            result.insert(captures[1].to_string());
        }

        if captures[2].parse::<usize>().is_err() {
            result.insert(captures[2].to_string());
        }
    } else if let Some(captures) = OR_RE.captures(line) {
        if captures[1].parse::<usize>().is_err() {
            result.insert(captures[1].to_string());
        }

        if captures[2].parse::<usize>().is_err() {
            result.insert(captures[2].to_string());
        }
    } else if let Some(captures) = LSHIFT_RE.captures(line) {
        result.insert(captures[1].to_string());
    } else if let Some(captures) = RSHIFT_RE.captures(line) {
        result.insert(captures[1].to_string());
    } else if let Some(captures) = NOT_RE.captures(line) {
        result.insert(captures[1].to_string());
    } else if let Some(captures) = WRITE_RE.captures(line) {
        result.insert(captures[1].to_string());
    }

    result
}

pub fn solve(input: &str, wire_values: &mut HashMap<String, usize>) {
    let mut dependencies: HashMap<(&str, &str), HashSet<String>> = HashMap::new();

    let mut s = vec![];

    for line in input.lines() {
        let (mut left, mut right) = line.split_once("->").unwrap();

        left = left.trim();
        right = right.trim();

        let deps = deps(left);

        if deps.is_empty() {
            s.push((left, right));
        }

        dependencies.insert((left, right), deps);
    }

    // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
    while !s.is_empty() {
        let (command, writes_to) = s.pop().unwrap();

        wire_values.insert(writes_to.to_string(), execute_command(command, wire_values));

        for (edge, edge_deps) in dependencies
            .iter_mut()
            .filter(|(_, v)| v.contains(&writes_to.to_string()))
        {
            edge_deps.remove(writes_to);

            if edge_deps.is_empty() {
                s.push(edge.clone());
            }
        }
    }
}

fn execute_command(line: &str, wire_values: &HashMap<String, usize>) -> usize {
    if let Some(captures) = VAL_RE.captures(line) {
        return captures[1].parse().unwrap();
    } else if let Some(captures) = AND_RE.captures(line) {
        let left = captures[1]
            .parse::<usize>()
            .unwrap_or_else(|_| *wire_values.get(&captures[1]).unwrap());

        let right = captures[2]
            .parse::<usize>()
            .unwrap_or_else(|_| *wire_values.get(&captures[2]).unwrap());

        return left & right;
    } else if let Some(captures) = OR_RE.captures(line) {
        let left = captures[1]
            .parse::<usize>()
            .unwrap_or_else(|_| *wire_values.get(&captures[1]).unwrap());

        let right = captures[2]
            .parse::<usize>()
            .unwrap_or_else(|_| *wire_values.get(&captures[2]).unwrap());

        return left | right;
    } else if let Some(captures) = LSHIFT_RE.captures(line) {
        let left = captures[1]
            .parse::<usize>()
            .unwrap_or_else(|_| *wire_values.get(&captures[1]).unwrap());

        let right = captures[2].parse::<usize>().unwrap();

        return left << right;
    } else if let Some(captures) = RSHIFT_RE.captures(line) {
        let left = captures[1]
            .parse::<usize>()
            .unwrap_or_else(|_| *wire_values.get(&captures[1]).unwrap());

        let right = captures[2].parse::<usize>().unwrap();

        return left >> right;
    } else if let Some(captures) = NOT_RE.captures(line) {
        let op = captures[1]
            .parse::<usize>()
            .unwrap_or_else(|_| *wire_values.get(&captures[1]).unwrap());

        return !op;
    } else if let Some(captures) = WRITE_RE.captures(line) {
        let wire_value = *wire_values.get(&captures[1]).unwrap();

        return wire_value;
    }

    panic!();
}

pub fn solve_part_1(input: &str) -> usize {
    let mut wires = HashMap::new();
    solve(input, &mut wires);
    wires["a"]
}

pub fn solve_part_2(input: &str) -> usize {
    let mut wires_first_run = HashMap::new();
    solve(input, &mut wires_first_run);

    let mut wires_second_run = HashMap::new();
    wires_second_run.insert("b".to_string(), wires_first_run["a"]);
    solve(input, &mut wires_second_run);

    wires_second_run["a"]
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn part1_test_input() {
        let mut wires = HashMap::new();
        super::solve(&include_str!("day7/test_input"), &mut wires);
        assert_eq!(wires["d"], 72);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day7/input")), 46_065);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day7/input")), 1);
    }
}
