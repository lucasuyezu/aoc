use std::collections::{HashMap, VecDeque};

use crate::utils::lcm;

type ModuleConfiguration = HashMap<String, (String, Vec<String>)>;
type StateMap = HashMap<String, bool>;
type ConjunctionStateMap = HashMap<String, StateMap>;

const FLIP_FLOP: &'static str = "%";
const CONJUNCTION: &'static str = "&";

fn parse_input(input: &str) -> (ModuleConfiguration, StateMap, ConjunctionStateMap) {
    let module_configuration: ModuleConfiguration = input
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
                module_name.trim().to_string(),
                (
                    module_kind.trim().to_string(),
                    dest_str.trim().split(",").map(|s| s.trim().to_string()).collect(),
                ),
            )
        })
        .collect();

    let mut conjunction_states: HashMap<String, StateMap> = HashMap::new();

    for (name, (kind, _)) in module_configuration.iter() {
        if kind == CONJUNCTION {
            conjunction_states.insert(name.clone(), HashMap::new());
        }
    }

    for name in conjunction_states.clone().keys() {
        for (from_name, (_, destinations)) in module_configuration.iter() {
            if destinations.contains(name) {
                conjunction_states
                    .get_mut(name)
                    .unwrap()
                    .insert(from_name.clone(), false);
            }
        }
    }

    (module_configuration, HashMap::new(), conjunction_states)
}

fn press_button(
    configuration: &ModuleConfiguration,
    flip_flop_states: &mut StateMap,
    conjunction_states: &mut HashMap<String, StateMap>,
) -> HashMap<(String, String), (usize, usize)> {
    let mut pulses: HashMap<(String, String), (usize, usize)> = HashMap::new();
    pulses.insert(("input".to_string(), "broadcaster".to_string()), (1, 0));

    let (_kind, destinations) = configuration.get("broadcaster").unwrap();
    let mut queue: VecDeque<(String, String, bool)> = VecDeque::new();

    for destination in destinations {
        queue.push_back((destination.clone(), "broadcaster".to_string(), false));
    }

    while let Some((module_name, module_from, high_pulse)) = queue.pop_front() {
        if high_pulse {
            pulses
                .entry((module_from.to_string(), module_name.clone()))
                .and_modify(|(_, high)| *high += 1)
                .or_insert((0, 1));
        } else {
            pulses
                .entry((module_from.to_string(), module_name.clone()))
                .and_modify(|(low, _)| *low += 1)
                .or_insert((1, 0));
        }

        if let Some((kind, destinations)) = configuration.get(&module_name) {
            let low_pulse = !high_pulse;

            if kind == FLIP_FLOP && low_pulse {
                let state = *flip_flop_states.get(&module_name).unwrap_or(&false);
                flip_flop_states.insert(module_name.clone(), !state);

                for destination in destinations {
                    queue.push_back((destination.clone(), module_name.clone(), !state));
                }
            }

            if kind == CONJUNCTION {
                let x = conjunction_states.get_mut(&module_name).unwrap();
                x.insert(module_from.clone(), high_pulse);

                let all_high_pulses = x.values().all(|state| *state);

                for destination in destinations {
                    queue.push_back((destination.clone(), module_name.clone(), !all_high_pulses));
                }
            }
        }
    }

    pulses
}

pub fn solve_part_1(input: &str) -> usize {
    let (module_configuration, mut flip_flop_states, mut conjunction_states) = parse_input(input);

    let (total_low, total_high) = (0..1_000)
        .map(|_| {
            press_button(&module_configuration, &mut flip_flop_states, &mut conjunction_states)
                .values()
                .fold((0, 0), |(total_low, total_high), (cur_low, cur_high)| {
                    (total_low + cur_low, total_high + cur_high)
                })
        })
        .fold((0, 0), |(total_low, total_high), (cur_low, cur_high)| {
            (total_low + cur_low, total_high + cur_high)
        });

    total_low * total_high
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
    let mut cycle_modules: HashMap<String, usize> = conjunction_states
        .get("gf")
        .unwrap()
        .iter()
        .map(|(name, _)| (name.clone(), 0))
        .collect();

    for i in 1..usize::MAX {
        let pulses = press_button(&module_configuration, &mut flip_flop_states, &mut conjunction_states);

        pulses
            .iter()
            .filter(|((_, module_to), (_, high))| module_to == "gf" && *high > 0)
            .for_each(|((module_from, _), _)| {
                cycle_modules.entry(module_from.clone()).and_modify(|e| {
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
