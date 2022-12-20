use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Valve {
    name: String,
    flow_rate: usize,
}

lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
}

fn parse_input(input: &str) -> (HashMap<String, Valve>, HashMap<String, Vec<String>>) {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let cap = LINE_RE.captures_iter(line).next().unwrap();

        let valve = Valve {
            name: cap[1].to_string(),
            flow_rate: cap[2].parse().unwrap(),
        };

        edges.insert(
            valve.name.clone(),
            cap[3].split(',').map(|s| s.trim().to_string()).collect(),
        );

        valves.insert(valve.name.to_string(), valve);
    }

    (valves, edges)
}

fn solve(
    valves: &HashMap<String, Valve>,
    edges: &HashMap<String, Vec<String>>,
    open_valves: &HashSet<Valve>,
    valve_name: &String,
    visited: &mut HashMap<String, usize>,
    minutes_left: usize,
) -> usize {
    let open_valves_names = open_valves
        .iter()
        .map(|open_valve| open_valve.name.clone())
        .collect::<Vec<String>>();

    println!(
        "{} minutes left. on valve {}. open valves {:?}",
        minutes_left, valve_name, open_valves_names
    );
    if minutes_left == 0 {
        println!("\tTime's up. Returning zero");
        return 0;
    }

    let cache_key = format!("{}-{}-{:?}", minutes_left, valve_name, open_valves_names);
    println!("Cache key is {}", cache_key);
    if let Some(cached_result) = visited.get(&cache_key) {
        println!("cached_result: {}", cached_result);
        return *cached_result;
    }

    let valve = valves.get(valve_name).unwrap();
    let mut max_pressure = 0;

    // Open valve if flow rate > 0
    if !open_valves.contains(&valve) && valve.flow_rate > 0 {
        let mut new_open_valves = open_valves.clone();
        new_open_valves.insert(valve.clone());
        let result = solve(
            valves,
            edges,
            &new_open_valves,
            valve_name,
            visited,
            minutes_left - 1,
        );
        max_pressure = max_pressure.max(((minutes_left - 1) * valve.flow_rate) + result);
    }

    // walk to all edges
    for edge_name in edges.get(valve_name).unwrap() {
        let mut new_open_valves = open_valves.clone();
        new_open_valves.insert(valve.clone());
        let result = solve(
            valves,
            edges,
            open_valves,
            edge_name,
            visited,
            minutes_left - 1,
        );
        max_pressure = max_pressure.max(result);
    }

    visited.insert(cache_key, max_pressure);
    return max_pressure;
}

pub fn solve_part_1(input: &str) -> usize {
    let (valves, edges) = parse_input(input);
    solve(
        &valves,
        &edges,
        &mut HashSet::new(),
        &String::from("AA"),
        &mut HashMap::new(),
        30,
    )
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
