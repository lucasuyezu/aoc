use std::collections::{HashMap, HashSet};

use regex::Regex;

lazy_static! {
    static ref VAL_RE: Regex = Regex::new(r"\A(\d+) ->").unwrap();
    static ref AND_RE: Regex = Regex::new(r"\A(\w+) AND (\w+) ->").unwrap();
    static ref OR_RE: Regex = Regex::new(r"\A(\w+) OR (\w+) ->").unwrap();
    static ref LSHIFT_RE: Regex = Regex::new(r"\A(\w+) LSHIFT (\d+)").unwrap();
    static ref RSHIFT_RE: Regex = Regex::new(r"\A(\w+) RSHIFT (\d+)").unwrap();
    static ref NOT_RE: Regex = Regex::new(r"\ANOT (\w+) ->").unwrap();
    static ref WRITE_RE: Regex = Regex::new(r"\A([a-zA-Z]+) ->").unwrap();
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Operation {
    Value(usize),
    And(String, String),
    Or(String, String),
    LShift(String, usize),
    RShift(String, usize),
    Not(String),
    Write(String),
}

fn parse(line: &str) -> (Operation, String, HashSet<String>) {
    let operation;
    let mut deps = HashSet::new();

    if let Some(captures) = VAL_RE.captures(line) {
        operation = Operation::Value(captures[1].parse().unwrap());

        if captures[1].parse::<usize>().is_err() {
            deps.insert(captures[1].to_string());
        }
    } else if let Some(captures) = AND_RE.captures(line) {
        operation = Operation::And(captures[1].to_string(), captures[2].to_string());

        if captures[1].parse::<usize>().is_err() {
            deps.insert(captures[1].to_string());
        }
        if captures[2].parse::<usize>().is_err() {
            deps.insert(captures[2].to_string());
        }
    } else if let Some(captures) = OR_RE.captures(line) {
        operation = Operation::Or(captures[1].to_string(), captures[2].to_string());

        if captures[1].parse::<usize>().is_err() {
            deps.insert(captures[1].to_string());
        }
        if captures[2].parse::<usize>().is_err() {
            deps.insert(captures[2].to_string());
        }
    } else if let Some(captures) = LSHIFT_RE.captures(line) {
        operation = Operation::LShift(captures[1].to_string(), captures[2].parse().unwrap());
        deps.insert(captures[1].to_string());
    } else if let Some(captures) = RSHIFT_RE.captures(line) {
        operation = Operation::RShift(captures[1].to_string(), captures[2].parse().unwrap());
        deps.insert(captures[1].to_string());
    } else if let Some(captures) = NOT_RE.captures(line) {
        operation = Operation::Not(captures[1].to_string());
        deps.insert(captures[1].to_string());
    } else if let Some(captures) = WRITE_RE.captures(line) {
        operation = Operation::Write(captures[1].to_string());
        deps.insert(captures[1].to_string());
    } else {
        dbg!(&line);
        panic!();
    }

    (operation, line.split_once("->").unwrap().1.trim().to_string(), deps)
}

pub fn solve(input: &str, wire_values: &mut HashMap<String, usize>) {
    let mut dependencies: HashMap<(Operation, String), HashSet<String>> = HashMap::new();

    let mut s = vec![];

    for line in input.lines() {
        let (operation, wire_dest, deps) = parse(line);

        if deps.is_empty() {
            s.push((operation.clone(), wire_dest.clone()));
        }

        dependencies.insert((operation, wire_dest), deps);
    }

    // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
    while !s.is_empty() {
        let (command, writes_to) = s.pop().unwrap();

        wire_values.insert(writes_to.to_string(), execute_command(command, wire_values));

        for (edge, edge_deps) in dependencies
            .iter_mut()
            .filter(|(_, v)| v.contains(&writes_to.to_string()))
        {
            edge_deps.remove(&writes_to);

            if edge_deps.is_empty() {
                s.push(edge.clone());
            }
        }
    }
}

fn operand_or_wire_value(operand: String, wire_values: &HashMap<String, usize>) -> usize {
    operand.parse::<usize>().unwrap_or_else(|_| wire_values[&operand])
}

fn execute_command(operation: Operation, wire_values: &HashMap<String, usize>) -> usize {
    dbg!(&operation);
    dbg!(&wire_values);
    match operation {
        Operation::Value(operand) => operand,
        Operation::Write(operand) => wire_values[&operand],
        Operation::Not(operand) => !wire_values[&operand],
        Operation::LShift(left_operand, right_operand) => wire_values[&left_operand] << right_operand,
        Operation::RShift(left_operand, right_operand) => wire_values[&left_operand] >> right_operand,
        Operation::And(left_operand, right_operand) => {
            operand_or_wire_value(left_operand, wire_values) & operand_or_wire_value(right_operand, wire_values)
        }
        Operation::Or(left_operand, right_operand) => {
            operand_or_wire_value(left_operand, wire_values) | operand_or_wire_value(right_operand, wire_values)
        }
    }
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
