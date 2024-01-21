use std::collections::{HashMap, HashSet};

use regex::Regex;

lazy_static! {
    static ref VAL_RE: Regex = Regex::new(r"\A(\w+) ->").unwrap();
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Operation<'a> {
    Value(usize),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    LShift(&'a str, usize),
    RShift(&'a str, usize),
    Not(&'a str),
    Write(&'a str),
}

fn parse(line: &str) -> (Operation, &str, HashSet<&str>) {
    let operation;
    let mut deps: HashSet<&str> = HashSet::new();

    let (command, writes_to) = line.split_once("->").unwrap();

    if VAL_RE.is_match(line) {
        match command.trim().parse::<usize>() {
            Ok(val) => operation = Operation::Value(val),
            Err(_) => {
                operation = Operation::Write(command.trim());
                deps.insert(command.trim());
            }
        }
    } else if let Some((left_operand, right_operand)) = command.split_once("AND") {
        operation = Operation::And(left_operand.trim(), right_operand.trim());

        if left_operand.trim().parse::<usize>().is_err() {
            deps.insert(left_operand.trim());
        }
        if right_operand.trim().parse::<usize>().is_err() {
            deps.insert(right_operand.trim());
        }
    } else if let Some((left_operand, right_operand)) = command.split_once("OR") {
        operation = Operation::Or(left_operand.trim(), right_operand.trim());

        if left_operand.trim().parse::<usize>().is_err() {
            deps.insert(left_operand.trim());
        }
        if right_operand.trim().parse::<usize>().is_err() {
            deps.insert(right_operand.trim());
        }
    } else if let Some((left_operand, right_operand)) = command.split_once("LSHIFT") {
        operation = Operation::LShift(left_operand.trim(), right_operand.trim().parse().unwrap());
        deps.insert(left_operand.trim());
    } else if let Some((left_operand, right_operand)) = command.split_once("RSHIFT") {
        operation = Operation::RShift(left_operand.trim(), right_operand.trim().parse().unwrap());
        deps.insert(left_operand.trim());
    } else if let Some((_, operand)) = command.split_once("NOT") {
        operation = Operation::Not(operand.trim());
        deps.insert(operand.trim());
    } else {
        panic!();
    }

    (operation, writes_to.trim(), deps)
}

pub fn solve<'a>(input: &'a str, wire_values: &mut HashMap<&'a str, usize>) {
    let mut dependencies: HashMap<(Operation, &str), HashSet<&str>> = HashMap::new();

    let mut s = vec![];

    for line in input.lines() {
        let (operation, wire_dest, deps) = parse(line);

        if deps.is_empty() {
            s.push((operation.clone(), wire_dest));
        }

        dependencies.insert((operation, wire_dest), deps);
    }

    // https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
    while !s.is_empty() {
        let (command, writes_to) = s.pop().unwrap();

        wire_values.insert(writes_to, execute_command(command, wire_values));

        for (edge, edge_deps) in dependencies.iter_mut().filter(|(_, v)| v.contains(writes_to)) {
            edge_deps.remove(writes_to);

            if edge_deps.is_empty() {
                s.push(edge.clone());
            }
        }
    }
}

fn operand_or_wire_value(operand: &str, wire_values: &HashMap<&str, usize>) -> usize {
    operand.parse::<usize>().unwrap_or_else(|_| wire_values[operand])
}

fn execute_command(operation: Operation, wire_values: &HashMap<&str, usize>) -> usize {
    match operation {
        Operation::Value(operand) => operand,
        Operation::Write(operand) => wire_values[operand],
        Operation::Not(operand) => !wire_values[operand],
        Operation::LShift(left_operand, right_operand) => wire_values[left_operand] << right_operand,
        Operation::RShift(left_operand, right_operand) => wire_values[left_operand] >> right_operand,
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
    wires_second_run.insert("b", wires_first_run["a"]);
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
