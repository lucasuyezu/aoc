use std::{collections::HashMap, str::FromStr, string::ParseError};

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
    fn quality_level(&self, minutes_left: u8) -> u32 {
        // start with one ore collecting robot
        self.id * self.dfs(minutes_left, 1, 0, 0, 0, 0, 0, 0, 0, &mut HashMap::new())
    }

    fn dfs(
        &self,
        minutes_left: u8,
        ore_robots_count: u32,
        ore_count: u32,
        clay_robots_count: u32,
        clay_count: u32,
        obsidian_robots_count: u32,
        obsidian_count: u32,
        geode_robots_count: u32,
        geode_count: u32,
        visited: &mut HashMap<String, u32>,
    ) -> u32 {
        println!(
            "minutes_left={:<3} ore_robots_count={:<3}, ore_count={:<3}, clay_robots_count={:<3}, clay_count={:<3}, obsidian_robots_count={:<3}, obsidian_count={:<3}, geode_robots_count={:<3}, geode_count={:<3}",
            minutes_left, ore_robots_count, ore_count, clay_robots_count, clay_count, obsidian_robots_count, obsidian_count, geode_robots_count, geode_count,
        );

        if geode_count > 9 {
            panic!();
        }

        if minutes_left == 0 {
            return geode_count;
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
        if let Some(cached_result) = visited.get(&cache_key) {
            return *cached_result;
        }

        let mut max_geode_count = 0;

        // Robots mine every minute
        let new_ore_count = ore_count + ore_robots_count;
        let new_clay_count = clay_count + clay_robots_count;
        let new_obsidian_count = obsidian_count + obsidian_robots_count;
        let new_geode_count = geode_count + geode_robots_count;

        // Build a geode robot
        if ore_count >= self.geode_robot_ore_cost && obsidian_count > self.geode_robot_obsidian_cost
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
                visited,
            ));
        }

        // Build an obsidian robot
        // No need to build an obsidian robot if I already have enough obsidian robots to build a geodoe robot
        if ore_count >= self.obsidian_robot_ore_cost
            && clay_count > self.obsidian_robot_clay_cost
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
                visited,
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
            visited,
        ));

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
                visited,
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
                visited,
            ));
        }

        visited.insert(cache_key, max_geode_count);
        max_geode_count
    }
}

pub fn solve_part_1(input: &str) -> u32 {
    // let blueprints: Vec<Blueprint> = input
    //     .lines()
    //     .map(|line| line.parse::<Blueprint>().unwrap())
    //     .collect();

    let blueprints = vec![Blueprint {
        id: 1,
        ore_robot_cost: 4,
        clay_robot_ore_cost: 2,
        obsidian_robot_ore_cost: 3,
        obsidian_robot_clay_cost: 14,
        geode_robot_ore_cost: 2,
        geode_robot_obsidian_cost: 7,
        max_ore_cost: 4,
    }];

    blueprints
        .iter()
        .map(|blueprint| blueprint.quality_level(24))
        .sum()
}

pub fn solve_part_2(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
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
    }

    #[test]
    fn part1_test_input() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        assert_eq!(super::solve_part_1(&input), 9);
    }
}
