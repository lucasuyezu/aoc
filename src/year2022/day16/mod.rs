use std::{
    collections::{HashMap},
};

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    id: usize,
    name: String,
    flow_rate: usize,
}

lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
}

fn parse_input(input: &str) -> (HashMap<usize, Valve>, HashMap<usize, Vec<usize>>) {
    let mut valves: HashMap<usize, Valve> = HashMap::new();
    let mut edges_names: HashMap<usize, Vec<String>> = HashMap::new();
    let mut edges_ids: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        let cap = LINE_RE.captures_iter(line).next().unwrap();

        let valve = Valve {
            id: i + 1,
            name: cap[1].to_string(),
            flow_rate: cap[2].parse().unwrap(),
        };

        edges_names.insert(valve.id, cap[3].split(',').map(|s| s.trim().to_string()).collect());

        valves.insert(valve.id, valve);
    }

    for valve in valves.values() {
        edges_ids.insert(
            valve.id,
            edges_names
                .get(&valve.id)
                .unwrap()
                .iter()
                .map(|edge_name| valves.values().find(|valve| valve.name == *edge_name).unwrap().id)
                .collect::<Vec<_>>(),
        );
    }

    (valves, edges_ids)
}

fn solve(
    valves: &HashMap<usize, Valve>,
    edges: &HashMap<usize, Vec<usize>>,
    open_valves: usize,
    valve_id: usize,
    visited: &mut HashMap<(usize, usize, usize), usize>,
    minutes_left: usize,
) -> usize {
    // println!(
    //     "{} minutes left. on valve {}. open valves {:?}",
    //     minutes_left, valve_name, open_valves_names
    // );
    if minutes_left == 0 {
        // println!("\tTime's up. Returning zero");
        return 0;
    }

    // let cache_key = format!("{}-{}-{:?}", minutes_left, valve_id, open_valves_names);
    let cache_key = (minutes_left, valve_id, open_valves);
    // println!("Cache key is {}", cache_key);
    if let Some(cached_result) = visited.get(&cache_key) {
        // println!("cached_result: {}", cached_result);
        return *cached_result;
    }

    let valve = valves.get(&valve_id).unwrap();
    let mut max_pressure = 0;

    // Open valve if flow rate > 0
    if (open_valves & (1 << valve_id) == 0) && valve.flow_rate > 0 {
         // dbg!(open_valves);
         // dbg!(valve);
         // dbg!(open_valves & (1 << valve_id));
         // dbg!(open_valves | (1 << valve_id));
         // panic!();

        let new_open_valves = open_valves | (1 << valve_id);
        assert!(new_open_valves > open_valves);
        let result = solve(valves, edges, new_open_valves, valve_id, visited, minutes_left - 1);
        max_pressure = max_pressure.max(((minutes_left - 1) * valve.flow_rate) + result);
    }

    // walk to all edges
    for edge_id in edges.get(&valve_id).unwrap() {
        let result = solve(valves, edges, open_valves, *edge_id, visited, minutes_left - 1);
        max_pressure = max_pressure.max(result);
    }

    visited.insert(cache_key, max_pressure);
    return max_pressure;
}

pub fn solve_part_1(input: &str) -> usize {
    let (valves, edges) = parse_input(input);
    let start_valve = valves.values().find(|valve| valve.name == String::from("AA")).unwrap();

    solve(&valves, &edges, 0, start_valve.id, &mut HashMap::new(), 30)
}

pub fn solve_part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 1651);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 1638);
    }
}
