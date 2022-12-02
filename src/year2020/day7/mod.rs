use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq)]
struct BagRule {
    handle: String,
    quantity: usize,
}

pub fn solve_part_1(lines: &[String]) -> usize {
    // build hash map of bag handle => rules
    let mut inverted_tree_map = HashMap::<String, HashSet<String>>::new();

    for line in lines {
        // parse line
        let index = line.find("bags contain").unwrap();
        let (bag_handle, rules_string) = line.split_at(index);

        if let Some(_x) = rules_string.find("no other bags") {
            continue;
        }

        for rule_string in rules_string[12..].split(',') {
            let mut token_iter = rule_string.trim().split(' ');
            let _quantity = token_iter.next();
            let rule_handle_string = format!(
                "{} {}",
                token_iter.next().unwrap(),
                token_iter.next().unwrap().to_string(),
            )
            .to_string();

            // upsert bag_handle to its children
            let rules_set = inverted_tree_map
                .entry(rule_handle_string)
                .or_insert_with(HashSet::<String>::new);
            rules_set.insert(bag_handle.trim_end().to_string());
        }
    }

    let mut rules_set = HashSet::<String>::new();

    let mut current_node = String::from("shiny gold");
    let mut nodes_to_visit = Vec::<String>::new();

    // dbg!(&inverted_tree_map);
    loop {
        if let Some(children) = inverted_tree_map.get(&current_node) {
            for child in children {
                rules_set.insert(child.to_string());
                nodes_to_visit.push(child.to_string());
            }
        }

        if nodes_to_visit.is_empty() {
            return rules_set.len();
        } else {
            current_node = nodes_to_visit.pop().unwrap();
        }
        // dbg!(&rules_set);
        // dbg!(&nodes_to_visit);
        // dbg!(&current_node);
    }
}

pub fn solve_part_2(lines: &[String]) -> usize {
    // build hash map of bag handle => rules
    let mut tree_map = HashMap::<String, HashSet<BagRule>>::new();

    for line in lines {
        // parse line
        let index = line.find("bags contain").unwrap();
        let (bag_handle, rules_string) = line.split_at(index);

        if let Some(_x) = rules_string.find("no other bags") {
            continue;
        }

        for rule_string in rules_string[12..].split(',') {
            let mut token_iter = rule_string.trim().split(' ');
            let quantity = token_iter.next().unwrap().parse::<usize>().unwrap();
            let rule_handle_string = format!(
                "{} {}",
                token_iter.next().unwrap(),
                token_iter.next().unwrap().to_string(),
            )
            .to_string();

            // upsert bag_handle to its children
            let rules_set = tree_map
                .entry(bag_handle.trim_end().to_string())
                .or_insert_with(HashSet::<BagRule>::new);
            rules_set.insert(BagRule {
                handle: rule_handle_string,
                quantity,
            });
        }
    }

    let current_node = String::from("shiny gold");

    // dbg!(&tree_map);

    traverse_tree_map(&tree_map, current_node)
}

fn traverse_tree_map(tree_map: &HashMap<String, HashSet<BagRule>>, current_node: String) -> usize {
    let mut result = 0;

    if let Some(children) = tree_map.get(&current_node) {
        for child in children {
            // dbg!(&child);
            result += child.quantity
                + child.quantity * traverse_tree_map(&tree_map, child.handle.to_string());
        }
    }

    // dbg!(&result);
    result
}

#[cfg(test)]
mod tests {
    use super::super::super::utils;

    #[test]
    fn part1_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day7/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 4);
    }

    #[test]
    fn part2_test_inputs() {
        let mut lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day7/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 32);

        lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day7/test_input_part_2"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 126);
    }

    #[test]
    fn part1_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day7/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 124);
    }

    #[test]
    fn part2_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day7/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 34862);
    }
}
