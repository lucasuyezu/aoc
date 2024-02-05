use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

struct Reindeer<'a> {
    name: &'a str,
    speed: usize,
    fly_seconds: usize,
    distance_per_chunk: usize,
    seconds_per_chunk: usize,
}

impl<'a> Reindeer<'a> {
    fn distance_after(&self, seconds: usize) -> usize {
        let chunks = seconds / self.seconds_per_chunk;
        let last_chunk_seconds = seconds % self.seconds_per_chunk;
        let last_chunk_distance = if last_chunk_seconds >= self.fly_seconds {
            self.distance_per_chunk
        } else {
            self.speed * last_chunk_seconds
        };

        self.distance_per_chunk * chunks + last_chunk_distance
    }
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.").unwrap();
}

fn parse<'a>(input: &'a str) -> Vec<Reindeer<'a>> {
    let mut reindeers = vec![];

    for line in input.lines() {
        let captures = RE.captures(line).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let speed = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let fly_seconds = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let rest_seconds = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let distance_per_chunk = speed * fly_seconds;
        let seconds_per_chunk = fly_seconds + rest_seconds;

        reindeers.push(Reindeer {
            name,
            speed,
            fly_seconds,
            distance_per_chunk,
            seconds_per_chunk,
        });
    }

    reindeers
}

pub fn solve_distance(input: &str, seconds: usize) -> usize {
    parse(input)
        .iter()
        .map(|reindeer| reindeer.distance_after(seconds))
        .max()
        .unwrap()
}

pub fn solve_score(input: &str, seconds: usize) -> usize {
    let mut scores: HashMap<&str, usize> = HashMap::new();
    let hash = parse(input);

    for i in 1..seconds {
        let distances = hash
            .iter()
            .map(|reindeer| (reindeer.name, reindeer.distance_after(i)))
            .collect_vec();

        let max_distance = distances.iter().map(|(_name, dist)| dist).max().unwrap();

        let winners = distances
            .iter()
            .filter(|(_name, dist)| dist == max_distance)
            .map(|(name, _dist)| name)
            .collect_vec();

        for winner in winners {
            scores.entry(winner).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    *scores.values().max().unwrap()
}

pub fn solve_part_1(input: &str) -> usize {
    solve_distance(input, 2_503)
}

pub fn solve_part_2(input: &str) -> usize {
    solve_score(input, 2_503)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_distance(&include_str!("day14/test_input"), 1_000), 1_120);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day14/input")), 2_660);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_score(&include_str!("day14/test_input"), 1_000), 689);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day14/input")), 1_256);
    }
}
