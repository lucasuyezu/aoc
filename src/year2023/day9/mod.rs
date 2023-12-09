fn calculate_sequences(history: Vec<isize>) -> Vec<Vec<isize>> {
    let mut levels: Vec<Vec<isize>> = vec![];
    levels.push(history.clone());

    let mut last_level = levels.last().unwrap().clone();

    while !last_level.iter().all(|level| *level == 0) {
        let mut current_level: Vec<isize> = vec![];

        for i in 1..last_level.len() {
            current_level.push(last_level.get(i).unwrap() - last_level.get(i - 1).unwrap());
        }

        levels.push(current_level.clone());
        last_level = current_level;
    }

    levels
}

pub fn solve_part_1(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.split(" ").map(|n| n.parse().unwrap()).collect())
        .map(calculate_sequences)
        .map(|levels| levels.iter().map(|level| *level.last().unwrap()).sum::<isize>())
        .sum()
}

pub fn solve_part_2(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.split(" ").map(|n| n.parse().unwrap()).collect())
        .map(calculate_sequences)
        .map(|levels| {
            let firsts: Vec<isize> = levels.iter().rev().map(|level| *level.first().unwrap()).collect();

            let mut result = 0;
            for i in 1..levels.len() {
                result = firsts.get(i).unwrap() - result;
            }

            result
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 114);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 1955513104);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 2);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 1_131);
    }
}
