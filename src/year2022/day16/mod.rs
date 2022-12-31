use std::collections::HashMap;

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
            id: i,
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
    distances: &Vec<Vec<usize>>,
    open_valves: usize,
    valve_id: usize,
    visited: &mut HashMap<(usize, usize, usize, usize), usize>,
    minutes_left: usize,
    num_players: usize,
) -> usize {
    if minutes_left <= 1 {
        if num_players == 1 {
            return 0;
        } else {
            return solve(valves, distances, open_valves, 0, visited, 26, num_players - 1);
        }
    }

    let cache_key = (minutes_left, valve_id, open_valves, num_players);
    if let Some(cached_result) = visited.get(&cache_key) {
        return *cached_result;
    }

    let valve = valves.get(&valve_id).unwrap();
    let mut max_pressure = 0;

    if (minutes_left > 1 && open_valves & (1 << valve_id) == 0) && valve.flow_rate > 0 {
        let new_open_valves = open_valves | (1 << valve_id);
        let result = solve(
            valves,
            distances,
            new_open_valves,
            valve_id,
            visited,
            minutes_left - 1,
            num_players,
        );
        max_pressure = max_pressure.max(((minutes_left - 1) * valve.flow_rate) + result);
    }

    let closed_valves_with_flow: Vec<&Valve> = valves
        .values()
        .filter(|valve| valve.id != valve_id)
        .filter(|valve| open_valves & (1 << valve.id) == 0)
        .filter(|valve| valve.flow_rate > 0)
        .filter(|valve| minutes_left >= distances[valve_id][valve.id])
        .collect();

    for edge in closed_valves_with_flow {
        let result = solve(
            valves,
            distances,
            open_valves,
            edge.id,
            visited,
            minutes_left - distances[valve_id][edge.id],
            num_players,
        );
        max_pressure = max_pressure.max(result);
    }

    visited.insert(cache_key, max_pressure);
    return max_pressure;
}

fn floyd_warshall(valves: &HashMap<usize, Valve>, edges: &HashMap<usize, Vec<usize>>) -> Vec<Vec<usize>> {
    let mut distances: Vec<Vec<usize>> = vec![vec![valves.len() + 100; valves.len()]; valves.len()];

    for i in 0..valves.len() {
        distances[i][i] = 0;
    }

    for valve in valves.values() {
        for neighbour_name in edges.get(&valve.id).unwrap().iter() {
            let neighbour = valves.get(neighbour_name).unwrap();
            distances[valve.id][neighbour.id] = 1;
            distances[neighbour.id][valve.id] = 1;
        }
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j])
            }
        }
    }

    distances
}

pub fn solve_part_1(input: &str) -> usize {
    let (valves, edges) = parse_input(input);
    let distances = floyd_warshall(&valves, &edges);
    solve(&valves, &distances, 0, 0, &mut HashMap::new(), 30, 1)
}

pub fn solve_part_2(input: &str) -> usize {
    let (valves, edges) = parse_input(input);
    let distances = floyd_warshall(&valves, &edges);
    solve(&valves, &distances, 0, 0, &mut HashMap::new(), 26, 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part_1(&include_str!("test_input")), 1651);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(solve_part_1(&include_str!("input")), 1638);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(solve_part_2(&include_str!("test_input")), 1707);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(solve_part_2(&include_str!("input")), 2400);
    }
}
