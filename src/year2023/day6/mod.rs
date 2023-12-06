use regex::Regex;

#[derive(Debug)]
pub struct Race {
    time: usize,
    distance: usize,
}
impl Race {
    fn winning_settings_count(&self) -> usize {
        // find idx where we start winning
        let idx_start = (1..self.time + 1)
            .position(|speed| speed * (self.time - speed) > self.distance)
            .unwrap()
            + 1;

        // find idx where we stop winning
        let count = (idx_start..self.time + 1)
            .position(|speed| speed * (self.time - speed) <= self.distance)
            .unwrap();

        count
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    let re = Regex::new(r"\s+").unwrap();
    let (times_str, distances_str) = input.split_once("\n").unwrap();
    let times: Vec<usize> = re
        .replace_all(times_str.split_once(":").unwrap().1.trim(), " ")
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<usize> = re
        .replace_all(distances_str.split_once(":").unwrap().1.trim(), " ")
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    times
        .iter()
        .enumerate()
        .map(|(i, time)| Race {
            time: *time,
            distance: distances[i],
        })
        .collect()
}

pub fn solve_part_1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|race| race.winning_settings_count())
        .fold(1, |acc, x| acc * x)
}

pub fn solve_part_2(input: &str) -> usize {
    let re = Regex::new(r"\s").unwrap();
    let (time_str, distance_str) = input.split_once("\n").unwrap();
    let wtf = re.replace_all(time_str, "");
    let time: usize = wtf.split_once(":").unwrap().1.parse().unwrap();

    let wtf = re.replace_all(distance_str, "");
    let distance: usize = wtf.split_once(":").unwrap().1.parse().unwrap();

    let race = Race { time, distance };

    race.winning_settings_count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 288);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 588_588);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 71_503);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 34_655_848);
    }
}
