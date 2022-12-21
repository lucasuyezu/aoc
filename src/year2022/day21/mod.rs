use std::{collections::HashMap, str::FromStr, string::ParseError};

#[derive(Debug, Clone)]
enum Job {
    Addition(String, String),
    Subtraction(String, String),
    Multiplication(String, String),
    Division(String, String),
    Number(isize),
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    job: Job,
}

impl FromStr for Monkey {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");
        let name = split.next().unwrap().to_string();

        let job_str = split.next().unwrap().trim();
        let job = if job_str.len() < 4 {
            Job::Number(job_str.parse().unwrap())
        } else {
            let mut job_split = job_str.split(" ");

            let monkey_a = job_split.next().unwrap().to_string();
            let operator = job_split.next().unwrap();
            let monkey_b = job_split.next().unwrap().to_string();

            match operator {
                "+" => Job::Addition(monkey_a, monkey_b),
                "-" => Job::Subtraction(monkey_a, monkey_b),
                "*" => Job::Multiplication(monkey_a, monkey_b),
                "/" => Job::Division(monkey_a, monkey_b),
                _ => panic!("Invalid operator!"),
            }
        };

        Ok(Monkey { name, job })
    }
}

fn parse_input(input: &str) -> HashMap<String, Monkey> {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    for line in input.lines() {
        let monkey = line.parse::<Monkey>().unwrap();
        monkeys.insert(monkey.name.clone(), monkey);
    }

    monkeys
}

fn solve(monkeys: &HashMap<String, Monkey>, start: String) -> isize {
    let monkey = monkeys.get(&start).unwrap();
    match monkey.job.clone() {
        Job::Number(n) => n,
        Job::Addition(monkey_a, monkey_b) => {
            solve(monkeys, monkey_a.clone()) + solve(monkeys, monkey_b.clone())
        }
        Job::Subtraction(monkey_a, monkey_b) => {
            solve(monkeys, monkey_a.clone()) - solve(monkeys, monkey_b.clone())
        }
        Job::Multiplication(monkey_a, monkey_b) => {
            solve(monkeys, monkey_a.clone()) * solve(monkeys, monkey_b.clone())
        }
        Job::Division(monkey_a, monkey_b) => {
            solve(monkeys, monkey_a.clone()) / solve(monkeys, monkey_b.clone())
        }
    }
}

pub fn solve_part_1(input: &str) -> isize {
    let monkeys = parse_input(input);

    let root = monkeys.get(&"root".to_string()).unwrap();

    if let Job::Addition(monkey_a, monkey_b) = root.job.clone() {
        return solve(&monkeys, monkey_a) + solve(&monkeys, monkey_b);
    }

    panic!();
}

pub fn solve_part_2(input: &str) -> isize {
    let mut monkeys = parse_input(input);

    let root = monkeys.get(&"root".to_string()).unwrap();

    if let Job::Addition(monkey_a, monkey_b) = root.job.clone() {
        let mut lower_bound: usize = 0;
        let mut upper_bound: usize = 1;
        let mut x: usize = 0;
        let mut lhs;
        let mut rhs;
        let humn_left;

        let mut humn = monkeys.get_mut(&"humn".to_string()).unwrap();
        humn.job = Job::Number(0);

        lhs = solve(&monkeys, monkey_a.clone());
        rhs = solve(&monkeys, monkey_b.clone());

        humn_left = lhs < rhs;
        if humn_left {
            lhs = 0;
            rhs = 1;
        } else {
            lhs = 1;
            rhs = 0;
        }

        while humn_left && lhs < rhs || !humn_left && lhs > rhs {
            upper_bound *= 2;

            let mut humn = monkeys.get_mut(&"humn".to_string()).unwrap();
            humn.job = Job::Number(upper_bound as isize);

            lhs = solve(&monkeys, monkey_a.clone());
            rhs = solve(&monkeys, monkey_b.clone());
        }

        while lhs != rhs {
            x = (lower_bound + upper_bound) / 2;
            let mut humn = monkeys.get_mut(&"humn".to_string()).unwrap();
            humn.job = Job::Number(x as isize);

            lhs = solve(&monkeys, monkey_a.clone());
            rhs = solve(&monkeys, monkey_b.clone());

            if humn_left && lhs < rhs || !humn_left && lhs > rhs {
                lower_bound = x + 1;
            } else {
                upper_bound = x.checked_sub(1).unwrap();
            }
        }

        return x as isize;
    }

    panic!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 152);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 152479825094094);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 302);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 3360561285172);
    }
}
