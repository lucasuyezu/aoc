fn build_stacks(input: &str) -> (Vec<Vec<String>>, Vec<String>, Vec<String>) {
    let lines = input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    let mut lines_split = lines.split(|line| line == "");
    let (stack_count_line, stack_lines) = lines_split.next().unwrap().split_last().unwrap();
    let command_lines = lines_split.next().unwrap();

    let stack_count = stack_count_line
        .trim()
        .split(" ")
        .last()
        .unwrap()
        .parse()
        .unwrap();

    return (
        vec![Vec::new(); stack_count],
        stack_lines.to_owned(),
        command_lines.to_owned(),
    );
}

fn populate_stacks(stacks: &mut Vec<Vec<String>>, stack_lines: Vec<String>) {
    for stack_line in stack_lines {
        let mut str_index = 1;
        for stack in stacks.iter_mut() {
            let crate_str = stack_line.get(str_index..str_index + 1).unwrap();
            if crate_str != " " {
                stack.insert(0, crate_str.to_string());
            }
            str_index += 4;
        }
    }
}

pub fn solve_part_1(input: &str) -> String {
    let (mut stacks, stack_lines, command_lines) = build_stacks(input);

    populate_stacks(&mut stacks, stack_lines);

    // move crates around
    for command_line in command_lines {
        let mut line_tokens = command_line.split(" ");
        let move_count: usize = line_tokens.nth(1).unwrap().parse().unwrap();
        let stack_from_index: usize = line_tokens.nth(1).unwrap().parse().unwrap();
        let stack_to_index: usize = line_tokens.nth(1).unwrap().parse().unwrap();

        for _ in 0..move_count {
            let value = stacks[stack_from_index - 1].pop().unwrap();
            stacks[stack_to_index - 1].push(value);
        }
    }

    // Grab top crate from each stack
    let mut result = String::new();
    for i in 0..stacks.len() {
        result.push_str(stacks[i].pop().unwrap().as_str());
    }

    return result;
}

pub fn solve_part_2(input: &str) -> String {
    let (mut stacks, stack_lines, command_lines) = build_stacks(input);

    populate_stacks(&mut stacks, stack_lines);

    // move crates around
    for command_line in command_lines {
        let mut line_tokens = command_line.split(" ");
        let move_count: usize = line_tokens.nth(1).unwrap().parse().unwrap();
        let stack_from_index: usize = line_tokens.nth(1).unwrap().parse().unwrap();
        let stack_to_index: usize = line_tokens.nth(1).unwrap().parse().unwrap();

        let mut temp_list = Vec::new();

        for _ in 0..move_count {
            let value = stacks[stack_from_index - 1].pop().unwrap();
            temp_list.insert(0, value);
        }

        stacks[stack_to_index - 1].append(&mut temp_list);
    }

    // Grab top crate from each stack
    let mut result = String::new();
    for i in 0..stacks.len() {
        result.push_str(stacks[i].pop().unwrap().as_str());
    }

    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), "CMZ");
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), "CVCWCRTVQ");
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), "MCD");
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), "CNSCZWLVT");
    }
}
