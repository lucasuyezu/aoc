use std::collections::HashMap;

#[derive(Debug)]
struct BoardingGroup {
    lines: Vec<String>,
}

impl BoardingGroup {
    fn answers_map(&self) -> HashMap<char, usize> {
        let mut result = HashMap::<char, usize>::new();

        for line in &self.lines {
            for c in line.chars() {
                if result.contains_key(&c) {
                    let _x = result.insert(c, *result.get(&c).unwrap() + 1);
                } else {
                    result.insert(c, 1);
                }
            }
        }

        // dbg!(&result);
        return result;
    }

    fn one_answer_count(&self) -> usize {
        return self.answers_map().len();
    }

    fn all_answer_count(&self) -> usize {
        let answers_map: HashMap<char, usize> = self.answers_map();

        let all_answers: HashMap<&char, &usize> = answers_map
            .iter()
            .filter(|&(_k, v)| *v == self.lines.len())
            .collect();

        return all_answers.len();
    }
}

pub fn solve_part_1(lines: &Vec<String>) -> usize {
    return boarding_groups(lines)
        .iter()
        .map(|group| group.one_answer_count())
        .sum();
}

pub fn solve_part_2(lines: &Vec<String>) -> usize {
    return boarding_groups(lines)
        .iter()
        .map(|group| group.all_answer_count())
        .sum();
}

fn boarding_groups(lines: &Vec<String>) -> Vec<BoardingGroup> {
    let mut boarding_groups = Vec::<BoardingGroup>::new();
    let mut current_group_lines = Vec::<String>::new();

    for line in lines {
        if !line.is_empty() {
            current_group_lines.push(line.to_string());
            continue;
        }

        let boarding_group = BoardingGroup {
            lines: current_group_lines,
        };

        current_group_lines = Vec::<String>::new();
        boarding_groups.push(boarding_group);
    }

    let boarding_group = BoardingGroup {
        lines: current_group_lines,
    };

    boarding_groups.push(boarding_group);

    return boarding_groups;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day6/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 11);
    }

    #[test]
    fn part2_test_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day6/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 6);
    }

    #[test]
    fn part1_real_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day6/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 6911);
    }

    #[test]
    fn part2_real_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day6/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 3473);
    }
}
