fn parse(input: &str) -> Vec<(&str, usize)> {
    return input
        .lines()
        .map(|line| {
            let (movement, value) = line.split_once(" ").unwrap();
            (movement, value.parse::<usize>().unwrap())
        })
        .collect();
}

pub fn solve_part_1(input: &str) -> usize {
    let final_position =
        parse(input)
            .iter()
            .fold((0, 0), |acc, (movement, value)| match *movement {
                "forward" => (acc.0 + value, acc.1),
                "down" => (acc.0, acc.1 + value),
                "up" => (acc.0, acc.1 - value),
                _ => panic!(),
            });

    return final_position.0 * final_position.1;
}

pub fn solve_part_2(input: &str) -> usize {
    let final_position = parse(input)
        .iter()
        .fold((0, 0, 0), |acc, (movement, value)| match *movement {
            "forward" => (acc.0 + value, acc.1 + acc.2 * value, acc.2),
            "down" => (acc.0, acc.1, acc.2 + value),
            "up" => (acc.0, acc.1, acc.2 - value),
            _ => panic!(),
        });

    return final_position.0 * final_position.1;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 150);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 1_990_000);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 900);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 1_975_421_260);
    }
}
