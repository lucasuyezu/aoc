use std::{
    collections::HashMap,
    str::FromStr,
    string::ParseError,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use regex::Regex;

lazy_static! {
    static ref LINE_RE: Regex =
        Regex::new(r"\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)\D+(\d+)").unwrap();
}

#[derive(Debug, PartialEq, Eq)]
struct Blueprint {
    id: u32,
    ore_robot_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
    max_ore_cost: u32,
}

impl FromStr for Blueprint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cap = LINE_RE.captures_iter(s).next().unwrap();

        let id = cap[1].parse().unwrap();
        let ore_robot_cost = cap[2].parse().unwrap();
        let clay_robot_ore_cost = cap[3].parse().unwrap();
        let obsidian_robot_ore_cost = cap[4].parse().unwrap();
        let obsidian_robot_clay_cost = cap[5].parse().unwrap();
        let geode_robot_ore_cost = cap[6].parse().unwrap();
        let geode_robot_obsidian_cost = cap[7].parse().unwrap();

        Ok(Blueprint {
            id,
            ore_robot_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
            max_ore_cost: ore_robot_cost
                .max(clay_robot_ore_cost)
                .max(obsidian_robot_ore_cost)
                .max(geode_robot_ore_cost),
        })
    }
}

impl Blueprint {
    fn quality_level(&self, minutes_left: u32) -> u32 {
        self.id * self.max_geode_count(minutes_left)
    }

    fn max_geode_count(&self, minutes_left: u32) -> u32 {
        let mut cache = HashMap::new();
        cache.insert("best".to_string(), 0);
        let result = self.dfs(minutes_left, 1, 0, 0, 0, 0, 0, 0, 0, &mut cache);

        result
    }

    fn dfs(
        &self,
        minutes_left: u32,
        ore_robots_count: u32,
        ore_count: u32,
        clay_robots_count: u32,
        clay_count: u32,
        obsidian_robots_count: u32,
        obsidian_count: u32,
        geode_robots_count: u32,
        geode_count: u32,
        cache: &mut HashMap<String, u32>,
    ) -> u32 {
        if minutes_left == 0 {
            return geode_count;
        }

        let upper_bound = geode_count
            + (geode_robots_count * minutes_left)
            + (minutes_left * (minutes_left + 1) / 2);

        if upper_bound < *cache.get("best").unwrap() {
            return 0;
        }

        let cache_key = format!(
            "{}-{}-{}-{}-{}-{}-{}-{}-{}",
            minutes_left,
            ore_robots_count,
            ore_count,
            clay_robots_count,
            clay_count,
            obsidian_robots_count,
            obsidian_count,
            geode_robots_count,
            geode_count,
        );
        if let Some(cached_result) = cache.get(&cache_key) {
            return *cached_result;
        }

        let mut max_geode_count = 0;

        // Robots mine every minute
        let new_ore_count = ore_count + ore_robots_count;
        let new_clay_count = clay_count + clay_robots_count;
        let new_obsidian_count = obsidian_count + obsidian_robots_count;
        let new_geode_count = geode_count + geode_robots_count;

        // Build a geode robot
        if ore_count >= self.geode_robot_ore_cost
            && obsidian_count >= self.geode_robot_obsidian_cost
        {
            max_geode_count = max_geode_count.max(self.dfs(
                minutes_left - 1,
                ore_robots_count,
                new_ore_count - self.geode_robot_ore_cost,
                clay_robots_count,
                new_clay_count,
                obsidian_robots_count,
                new_obsidian_count - self.geode_robot_obsidian_cost,
                geode_robots_count + 1,
                new_geode_count,
                cache,
            ));
        }

        // Build an obsidian robot
        // No need to build an obsidian robot if I already have enough obsidian robots to build a geodoe robot
        if ore_count >= self.obsidian_robot_ore_cost
            && clay_count >= self.obsidian_robot_clay_cost
            && obsidian_robots_count < self.geode_robot_obsidian_cost
        {
            max_geode_count = max_geode_count.max(self.dfs(
                minutes_left - 1,
                ore_robots_count,
                new_ore_count - self.obsidian_robot_ore_cost,
                clay_robots_count,
                new_clay_count - self.obsidian_robot_clay_cost,
                obsidian_robots_count + 1,
                new_obsidian_count,
                geode_robots_count,
                new_geode_count,
                cache,
            ));
        }

        // Build a clay robot
        // No need to build a clay robot if I already have enough clay robots to build an obsidian robot
        if ore_count >= self.clay_robot_ore_cost
            && clay_robots_count < self.obsidian_robot_clay_cost
        {
            max_geode_count = max_geode_count.max(self.dfs(
                minutes_left - 1,
                ore_robots_count,
                new_ore_count - self.clay_robot_ore_cost,
                clay_robots_count + 1,
                new_clay_count,
                obsidian_robots_count,
                new_obsidian_count,
                geode_robots_count,
                new_geode_count,
                cache,
            ));
        }

        // Build an ore robot
        // No need to build an ore robot if I already have more ore robots than self.max_ore_count
        if ore_count >= self.ore_robot_cost && ore_robots_count < self.max_ore_cost {
            max_geode_count = max_geode_count.max(self.dfs(
                minutes_left - 1,
                ore_robots_count + 1,
                new_ore_count - self.ore_robot_cost,
                clay_robots_count,
                new_clay_count,
                obsidian_robots_count,
                new_obsidian_count,
                geode_robots_count,
                new_geode_count,
                cache,
            ));
        }

        // Do nothing and accumulate resources
        max_geode_count = max_geode_count.max(self.dfs(
            minutes_left - 1,
            ore_robots_count,
            new_ore_count,
            clay_robots_count,
            new_clay_count,
            obsidian_robots_count,
            new_obsidian_count,
            geode_robots_count,
            new_geode_count,
            cache,
        ));

        cache.insert(cache_key, max_geode_count);

        if max_geode_count > *cache.get("best").unwrap() {
            cache.insert("best".to_string(), max_geode_count);
        }

        max_geode_count
    }
}

pub fn solve_part_1(input: &str) -> u32 {
    let blueprints: Vec<Blueprint> = input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect();

    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();
    let mut threads = vec![];

    for blueprint in blueprints {
        let thread_tx = tx.clone();
        threads.push(thread::spawn(move || {
            thread_tx.send(blueprint.quality_level(24)).unwrap();
        }));
    }

    let mut results: Vec<u32> = vec![];

    for _ in 0..threads.len() {
        results.push(rx.recv().unwrap());
    }

    for thread in threads {
        let _ = thread.join();
    }

    results.iter().sum()
}

pub fn solve_part_2(input: &str) -> u32 {
    let blueprints: Vec<Blueprint> = input
        .lines()
        .take(3)
        .map(|line| line.parse::<Blueprint>().unwrap())
        .collect();

    let (tx, rx): (Sender<u32>, Receiver<u32>) = mpsc::channel();
    let mut threads = vec![];

    for blueprint in blueprints {
        let thread_tx = tx.clone();
        threads.push(thread::spawn(move || {
            thread_tx.send(blueprint.max_geode_count(32)).unwrap();
        }));
    }

    let mut results: Vec<u32> = vec![];

    for _ in 0..threads.len() {
        results.push(rx.recv().unwrap());
    }

    for thread in threads {
        let _ = thread.join();
    }

    results.iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year2022::day19::Blueprint;

    #[test]
    fn part1_test_from_str() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        assert_eq!(
            Blueprint {
                id: 1,
                ore_robot_cost: 4,
                clay_robot_ore_cost: 2,
                obsidian_robot_ore_cost: 3,
                obsidian_robot_clay_cost: 14,
                geode_robot_ore_cost: 2,
                geode_robot_obsidian_cost: 7,
                max_ore_cost: 4,
            },
            input.parse::<Blueprint>().unwrap()
        );
        let input = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(
            Blueprint {
                id: 2,
                ore_robot_cost: 2,
                clay_robot_ore_cost: 3,
                obsidian_robot_ore_cost: 3,
                obsidian_robot_clay_cost: 8,
                geode_robot_ore_cost: 3,
                geode_robot_obsidian_cost: 12,
                max_ore_cost: 3,
            },
            input.parse::<Blueprint>().unwrap()
        );
        let input = "Blueprint 7: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 20 obsidian.";
        assert_eq!(
            Blueprint {
                id: 7,
                ore_robot_cost: 2,
                clay_robot_ore_cost: 3,
                obsidian_robot_ore_cost: 3,
                obsidian_robot_clay_cost: 8,
                geode_robot_ore_cost: 3,
                geode_robot_obsidian_cost: 20,
                max_ore_cost: 3,
            },
            input.parse::<Blueprint>().unwrap()
        );
    }

    #[test]
    fn part1_test_bp1() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        assert_eq!(solve_part_1(&input), 9);
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part_1(&include_str!("test_input")), 33);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(solve_part_1(&include_str!("input")), 1413);
    }

    #[test]
    fn part2_test_bp1() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        assert_eq!(solve_part_2(&input), 56);
    }

    #[test]
    fn part2_test_bp2() {
        let input = "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        assert_eq!(solve_part_2(&input), 62);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(solve_part_2(&include_str!("input")), 21080);
    }
}
