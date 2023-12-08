use std::collections::HashMap;
use crate::utils::lcm;

fn parse_input(input: &str) -> (String, HashMap<String, (String, String)>) {
    let (turns_str, rest) = input.split_once("\n\n").unwrap();
    let mut nodes_hash: HashMap<String, (String, String)> = HashMap::new();

    for line in rest.lines() {
        let (name_str, rest) = line.split_once("=").unwrap();
        let (left_str, right_str) = rest.trim().split_once(",").unwrap();

        let name = String::from(name_str.trim());
        let left = String::from(left_str.replace("(", ""));
        let right = String::from(right_str.trim().replace(")", ""));

        nodes_hash.insert(name, (left, right));
    }

    (String::from(turns_str), nodes_hash)
}

pub fn solve_part_1(input: &str) -> usize {
    let (turns_str, node_hash) = parse_input(input);

    let mut result = 0;
    let mut current_node = String::from("AAA");
    let target_node = String::from("ZZZ");

    let mut turn_chars = turns_str.chars().cycle();
    while current_node != target_node {
        current_node = match turn_chars.next().unwrap() {
            'L' => node_hash.get(&current_node).unwrap().0.clone(),
            'R' => node_hash.get(&current_node).unwrap().1.clone(),
            _ => panic!("Invalid turn"),
        };
        result += 1;
    }

    result
}

pub fn solve_part_2(input: &str) -> usize {
    let (turns_str, node_hash) = parse_input(input);

    let start_nodes: Vec<String> = node_hash
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.clone())
        .collect();

    let steps: Vec<usize> = start_nodes
        .into_iter()
        .map(|start_node| {
            let mut current_node = start_node;
            let mut steps_count = 0;
            let mut turn_chars = turns_str.chars().cycle();
            while !current_node.ends_with("Z") {
                let current_turn = turn_chars.next().unwrap();
                current_node = match current_turn {
                    'L' => node_hash.get(&current_node).unwrap().0.clone(),
                    'R' => node_hash.get(&current_node).unwrap().1.clone(),
                    _ => panic!("Invalid turn"),
                };

                steps_count += 1;
            }

            steps_count
        })
        .collect();

    lcm(&steps)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 2);
    }

    #[test]
    fn part1_test_input_2() {
        assert_eq!(super::solve_part_1(&include_str!("test_input_2")), 6);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 16_531);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input_3")), 6);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 24_035_773_251_517);
    }
}
