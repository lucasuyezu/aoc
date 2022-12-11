#[derive(Clone, Debug)]
enum Operation {
    Addition,
    Multiplication,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    operand: Option<usize>,
    divisible_test: usize,
    destination_monkey_true_index: usize,
    destination_monkey_false_index: usize,
    items_inspected: usize,
}

fn parse_input(input: &str) -> Vec<Monkey> {
    return input
        .split("\n\n")
        .map(|monkey_lines| {
            let mut monkey_lines_iter = monkey_lines.lines();

            let items: Vec<usize> = monkey_lines_iter.nth(1).unwrap()[18..]
                .split(",")
                .map(|number| number.trim().parse().unwrap())
                .collect();

            let operation_line = monkey_lines_iter.next().unwrap();
            let operation = match &operation_line[23..24] {
                "+" => Operation::Addition,
                "*" => Operation::Multiplication,
                _ => panic!(),
            };
            let operand = if operation_line[25..] == *"old" {
                None
            } else {
                Some(operation_line[25..].parse::<usize>().unwrap())
            };

            let divisible_test: usize = monkey_lines_iter.next().unwrap()[21..].parse().unwrap();

            let destination_monkey_true_index: usize =
                monkey_lines_iter.next().unwrap()[29..].parse().unwrap();

            let destination_monkey_false_index: usize =
                monkey_lines_iter.next().unwrap()[30..].parse().unwrap();

            Monkey {
                items,
                operation,
                operand,
                destination_monkey_true_index,
                destination_monkey_false_index,
                divisible_test,
                items_inspected: 0,
            }
        })
        .collect();
}

fn solve(input: &str, num_rounds: usize, divide_by_3: bool) -> usize {
    let mut monkeys: Vec<Monkey> = parse_input(input);

    let lcm: usize = monkeys.iter().map(|monkey| monkey.divisible_test).product();

    for _ in 0..num_rounds {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let mut item = monkeys[i].items[j];
                monkeys[i].items_inspected += 1;
                let operand = match monkeys[i].operand {
                    Some(value) => value,
                    None => item,
                };
                match monkeys[i].operation {
                    Operation::Addition => item = item.wrapping_add(operand) % lcm,
                    Operation::Multiplication => item = item.wrapping_mul(operand) % lcm,
                }
                if divide_by_3 {
                    item /= 3;
                }
                let new_index = if item % monkeys[i].divisible_test == 0 {
                    monkeys[i].destination_monkey_true_index
                } else {
                    monkeys[i].destination_monkey_false_index
                };
                monkeys[new_index].items.push(item);
            }

            monkeys[i].items = vec![];
        }
    }

    let mut items_inspected = monkeys
        .iter()
        .map(|monkey| monkey.items_inspected)
        .collect::<Vec<usize>>();

    items_inspected.sort_unstable();
    return items_inspected.iter().rev().take(2).product::<usize>();
}

pub fn solve_part_1(input: &str) -> usize {
    return solve(input, 20, true);
}

pub fn solve_part_2(input: &str) -> usize {
    return solve(input, 10_000, false);
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        let lines = &include_str!("test_input");
        assert_eq!(super::solve_part_1(lines), 10_605);
    }

    #[test]
    fn part1_real_input() {
        let lines = &include_str!("input");
        assert_eq!(super::solve_part_1(lines), 66_124);
    }

    #[test]
    fn part2_test_input() {
        let lines = &include_str!("test_input");
        assert_eq!(super::solve_part_2(lines), 2_713_310_158);
    }

    #[test]
    fn part2_real_input() {
        let lines = &include_str!("input");
        assert_eq!(super::solve_part_2(lines), 19_309_892_877);
    }
}
