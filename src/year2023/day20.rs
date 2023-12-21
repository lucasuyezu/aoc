use std::collections::{HashMap, VecDeque};

use crate::utils::lcm;

const FLIP_FLOP: &'static str = "%";
const CONJUNCTION: &'static str = "&";

fn parse_input(
    input: &str,
) -> (
    HashMap<&str, (&str, Vec<&str>)>,
    HashMap<&str, bool>,
    HashMap<&str, HashMap<&str, bool>>,
) {
    let module_configuration: HashMap<&str, (&str, Vec<&str>)> = input
        .lines()
        .map(|line| {
            let (module_name_str, dest_str) = line.split_once("->").map(|(s1, s2)| (s1.trim(), s2.trim())).unwrap();

            let module_name = match &module_name_str[0..1] {
                "b" => module_name_str,
                FLIP_FLOP | CONJUNCTION => &module_name_str[1..],
                _ => panic!("Invalid module"),
            };

            let module_kind = match &module_name_str[0..1] {
                "broadcaster" => ".",
                x => x,
            };

            (
                module_name.trim(),
                (
                    module_kind.trim(),
                    dest_str.trim().split(",").map(|s| s.trim()).collect(),
                ),
            )
        })
        .collect();

    let mut conjunction_states: HashMap<&str, HashMap<&str, bool>> = HashMap::new();

    for (name, (kind, _)) in module_configuration.iter() {
        if *kind == CONJUNCTION {
            conjunction_states.insert(name, HashMap::new());
        }
    }

    for name in conjunction_states.clone().keys() {
        for (from_name, (_, destinations)) in module_configuration.iter() {
            if destinations.contains(name) {
                conjunction_states.get_mut(name).unwrap().insert(from_name, false);
            }
        }
    }

    (module_configuration, HashMap::new(), conjunction_states)
}

fn press_button<'a>(
    configuration: &HashMap<&str, (&str, Vec<&'a str>)>,
    flip_flop_states: &mut HashMap<&'a str, bool>,
    conjunction_states: &mut HashMap<&str, HashMap<&'a str, bool>>,
) -> HashMap<(&'a str, &'a str), (usize, usize)> {
    let mut pulses: HashMap<(&str, &str), (usize, usize)> = HashMap::new();
    pulses.insert(("input", "broadcaster"), (1, 0));

    let (_kind, destinations) = configuration.get("broadcaster").unwrap();
    let mut queue: VecDeque<(&str, &str, bool)> = VecDeque::new();

    for destination in destinations {
        queue.push_back((destination, "broadcaster", false));
    }

    while let Some((module_name, module_from, high_pulse)) = queue.pop_front() {
        if high_pulse {
            pulses
                .entry((module_from, module_name))
                .and_modify(|(_, high)| *high += 1)
                .or_insert((0, 1));
        } else {
            pulses
                .entry((module_from, module_name))
                .and_modify(|(low, _)| *low += 1)
                .or_insert((1, 0));
        }

        if let Some((kind, destinations)) = configuration.get(module_name) {
            let low_pulse = !high_pulse;

            if *kind == FLIP_FLOP && low_pulse {
                let state = *flip_flop_states.get(module_name).unwrap_or(&false);
                flip_flop_states.insert(module_name, !state);

                for destination in destinations {
                    queue.push_back((destination, module_name, !state));
                }
            }

            if *kind == CONJUNCTION {
                let x = conjunction_states.get_mut(module_name).unwrap();
                x.insert(module_from, high_pulse);

                let all_high_pulses = x.values().all(|state| *state);

                for destination in destinations {
                    queue.push_back((destination, module_name, !all_high_pulses));
                }
            }
        }
    }

    pulses
}

pub fn solve_part_1(input: &str) -> usize {
    let (module_configuration, mut flip_flop_states, mut conjunction_states) = parse_input(input);

    let mut total_lo = 0;
    let mut total_hi = 0;

    for _ in 0..1_000 {
        for (lo, hi) in press_button(&module_configuration, &mut flip_flop_states, &mut conjunction_states).values() {
            total_lo += *lo;
            total_hi += *hi;
        }
    }

    total_lo * total_hi
}

pub fn solve_part_2(input: &str) -> usize {
    let (module_configuration, mut flip_flop_states, mut conjunction_states) = parse_input(input);

    // gf is the only one sending pulses to rx
    // These are the modules sending pulses to gf
    // "gf": {
    //     "zs": false,
    //     "kf": false,
    //     "qk": false,
    //     "kr": false,
    // },
    let mut cycle_modules: HashMap<&str, usize> = conjunction_states
        .get("gf")
        .unwrap()
        .iter()
        .map(|(name, _)| (*name, 0))
        .collect();

    for i in 1..usize::MAX {
        let pulses = press_button(&module_configuration, &mut flip_flop_states, &mut conjunction_states);

        pulses
            .iter()
            .filter(|((_, module_to), (_, high))| *module_to == "gf" && *high > 0)
            .for_each(|((module_from, _), _)| {
                cycle_modules.entry(module_from).and_modify(|e| {
                    if *e == 0 {
                        *e = i
                    }
                });
            });

        if cycle_modules.values().all(|v| *v != 0) {
            let cycles: Vec<usize> = cycle_modules.values().map(|v| *v).collect();
            return lcm(&cycles);
        }
    }

    panic!("Oh no :(");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input_1() {
        assert_eq!(super::solve_part_1(&include_str!("day20/test_input")), 32_000_000);
    }

    #[test]
    fn part1_test_input_2() {
        assert_eq!(super::solve_part_1(&include_str!("day20/test_input_2")), 11_687_500);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day20/input")), 739_960_225);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day20/input")), 231_897_990_075_517);
    }
}
