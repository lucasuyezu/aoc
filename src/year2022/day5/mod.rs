fn build_stacks(lines: &[String]) -> Vec<Vec<char>> {
    let line_length = lines.iter().next().unwrap().len();
    let stack_count = if line_length == 3 {
        1
    } else {
        1 + (line_length - 3) / 4
    };

    return vec![Vec::new(); stack_count];
}

pub fn solve_part_1(lines: &[String]) -> String {
    let mut stacks = build_stacks(lines);

    let mut iter = lines.iter();
    let mut line = iter.next().unwrap();

    while !line.starts_with(" 1") {
        let mut stack_index = 0;
        let crate_str = line.get(stack_index..stack_index + 3).unwrap();
        if crate_str.trim() != "" {
            let crate_char = crate_str.chars().nth(1).unwrap();
            stacks[stack_index].insert(0, crate_char);
        }

        stack_index = 1;
        let mut str_index = 3;
        while stack_index < stacks.len() {
            let crate_str = line.get(str_index..str_index + 4).unwrap();
            if crate_str.trim() != "" {
                let crate_char = crate_str.chars().nth(2).unwrap();
                stacks[stack_index].insert(0, crate_char);
            }

            stack_index += 1;
            str_index += 4;
        }

        line = iter.next().unwrap();
    }

    // throw away empty line
    iter.next();

    // move crates around
    for line in iter {
        let mut line_split = line.split(" ");
        let move_count: usize = line_split.nth(1).unwrap().parse().unwrap();
        let stack_from: usize = line_split.nth(1).unwrap().parse().unwrap();
        let stack_to: usize = line_split.nth(1).unwrap().parse().unwrap();

        for _ in 0..move_count {
            let value = stacks[stack_from - 1].pop().unwrap();
            stacks[stack_to - 1].push(value);
        }
    }

    // Grab top crate from each stack

    let mut result = String::new();
    for i in 0..stacks.len() {
        result.push(stacks[i].pop().unwrap());
    }

    return result;
}

pub fn solve_part_2(lines: &[String]) -> String {
    let mut stacks = build_stacks(lines);

    let mut iter = lines.iter();
    let mut line = iter.next().unwrap();

    // build_stacks
    while !line.starts_with(" 1") {
        let mut stack_index = 0;
        let crate_str = line.get(stack_index..stack_index + 3).unwrap();
        if crate_str.trim() != "" {
            let crate_char = crate_str.chars().nth(1).unwrap();
            stacks[stack_index].insert(0, crate_char);
        }

        stack_index = 1;
        let mut str_index = 3;
        while stack_index < stacks.len() {
            let crate_str = line.get(str_index..str_index + 4).unwrap();
            if crate_str.trim() != "" {
                let crate_char = crate_str.chars().nth(2).unwrap();
                stacks[stack_index].insert(0, crate_char);
            }

            stack_index += 1;
            str_index += 4;
        }

        line = iter.next().unwrap();
    }

    // throw away empty line
    iter.next();

    // move crates around
    for line in iter {
        let mut line_split = line.split(" ");
        let move_count: usize = line_split.nth(1).unwrap().parse().unwrap();
        let stack_from_index: usize = line_split.nth(1).unwrap().parse().unwrap();
        let stack_to_index: usize = line_split.nth(1).unwrap().parse().unwrap();

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
        result.push(stacks[i].pop().unwrap());
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::super::super::utils;

    #[test]
    fn part1_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day5/test_input".to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), "CMZ");
    }

    #[test]
    fn part1_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day5/input".to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), "CVCWCRTVQ");
    }

    #[test]
    fn part2_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day5/test_input".to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), "MCD");
    }

    #[test]
    fn part2_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2022/day5/input".to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), "CNSCZWLVT");
    }
}
