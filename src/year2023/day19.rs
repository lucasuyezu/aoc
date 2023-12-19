use std::{
    collections::{HashMap, VecDeque},
    ops::RangeInclusive,
};

type Workflow = HashMap<String, Vec<String>>;
type Part = HashMap<String, usize>;
type PartRanges = HashMap<String, RangeInclusive<usize>>;

fn parse_input(input: &str) -> (Workflow, Vec<Part>) {
    let (workflows_str, parts_str) = input.split_once("\n\n").unwrap();

    let workflows: Workflow = workflows_str
        .lines()
        .map(|workflow_str| {
            let (name, code_str) = workflow_str.split_once("{").unwrap();
            let code: Vec<String> = code_str
                .split_once("}")
                .unwrap()
                .0
                .split(",")
                .map(|s| s.to_string())
                .collect();

            (name.to_string(), code)
        })
        .collect();

    let parts: Vec<Part> = parts_str
        .lines()
        .map(|parts_str| {
            let (_, s) = parts_str.split_once("{").unwrap();
            let (s, _) = s.split_once("}").unwrap();

            s.split(",")
                .map(|part_str| {
                    let split = part_str.split_once("=").unwrap();

                    (split.0.to_string(), split.1.parse().unwrap())
                })
                .collect()
        })
        .collect();

    (workflows, parts)
}

fn is_accepted(part: &Part, workflow: &Workflow) -> bool {
    let mut current_workflow = workflow.get("in").unwrap();

    loop {
        for instruction in current_workflow {
            if instruction == "A" {
                return true;
            }

            if instruction == "R" {
                return false;
            }

            if instruction.contains("<") || instruction.contains(">") {
                let (test, result) = instruction.split_once(":").unwrap();

                let split_char = if test.contains("<") { "<" } else { ">" };

                let (k, v) = test.split_once(split_char).unwrap();

                let lhs = part[k];
                let rhs = v.parse().unwrap();

                if (split_char == "<" && lhs < rhs) || (split_char == ">" && lhs > rhs) {
                    match result {
                        "A" => return true,
                        "R" => return false,
                        redirect => {
                            current_workflow = workflow.get(redirect).unwrap();
                            break;
                        }
                    }
                } else {
                    continue;
                }
            }

            current_workflow = workflow.get(instruction).unwrap();
            break;
        }
    }
}

fn build_tree(workflow: &Workflow, part_ranges: &PartRanges) -> Vec<PartRanges> {
    let mut result: Vec<PartRanges> = vec![];
    let mut queue: VecDeque<(String, usize, PartRanges)> = VecDeque::new();

    queue.push_back(("in".to_string(), 0, part_ranges.clone()));

    while let Some(tuple) = queue.pop_front() {
        let (workflow_k, instruction_id, part) = tuple;

        let instruction = &workflow.get(&workflow_k).unwrap()[instruction_id];

        let (if_str, then_str) = instruction.split_once(":").unwrap();

        let k = if_str[0..1].to_string();
        let op = &if_str[1..2];
        let val: usize = if_str[2..].parse().unwrap();

        let mut lhs = part.clone();
        let mut rhs = part.clone();

        if op == "<" {
            lhs.insert(k.clone(), *part.get(&k).unwrap().start()..=val - 1);
            rhs.insert(k.clone(), val..=*part.get(&k).unwrap().end());
        } else {
            lhs.insert(k.clone(), (val + 1)..=*part.get(&k).unwrap().end());
            rhs.insert(k.clone(), *part.get(&k).unwrap().start()..=val);
        }

        match then_str {
            "R" => (),
            "A" => result.push(lhs),
            redirect => queue.push_back((redirect.to_string(), 0, lhs)),
        }

        if let Some(next) = workflow.get(&workflow_k).unwrap().get(instruction_id + 1) {
            match next.as_str() {
                "R" => (),
                "A" => result.push(rhs.clone()),
                _ => {
                    if next.contains("<") || next.contains(">") {
                        queue.push_back((workflow_k, instruction_id + 1, rhs));
                    } else {
                        queue.push_back((next.clone(), 0, rhs));
                    }
                }
            }
        }
    }

    result
}

pub fn solve_part_1(input: &str) -> usize {
    let (workflow, parts) = parse_input(input);

    parts
        .into_iter()
        .filter(|part| is_accepted(part, &workflow))
        .map(|part| part.values().sum::<usize>())
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let master_part: PartRanges = HashMap::from([
        (String::from("x"), 1..=4_000),
        (String::from("m"), 1..=4_000),
        (String::from("a"), 1..=4_000),
        (String::from("s"), 1..=4_000),
    ]);

    let workflow = parse_input(input).0;
    let result = build_tree(&workflow, &master_part);

    result
        .iter()
        .map(|part| {
            part.values()
                .map(|range| (range.end() + 1 - range.start()))
                .product::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day19/test_input")), 19_114);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day19/input")), 432_427);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            super::solve_part_2(&include_str!("day19/test_input")),
            167_409_079_868_000
        );
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day19/input")), 143_760_172_569_135);
    }
}
