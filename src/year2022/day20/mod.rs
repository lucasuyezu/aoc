pub fn solve_part_1(input: &str) -> isize {
    let mut numbers: Vec<(usize, isize)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| (i, line.parse::<isize>().unwrap()))
        .collect();

    for i in 0..numbers.len() {
        // find index of i
        let mut index = numbers.iter().position(|(j, _)| i == *j).unwrap();

        let tuple = numbers[index];

        for _ in 0..tuple.1.abs() {
            let new_index = if tuple.1 > 0 {
                if index == numbers.len() - 1 {
                    1
                } else {
                    index + 1
                }
            } else if index == 1 {
                numbers.len() - 1
            } else if index == 0 {
                numbers.len() - 2
            } else {
                index - 1
            };
            numbers.remove(index);
            numbers.insert(new_index, tuple);
            index = new_index;
        }
    }

    let zero_index = numbers.iter().position(|(_, n)| *n == 0).unwrap();

    numbers[(zero_index + 1000) % numbers.len()].1
        + numbers[(zero_index + 2000) % numbers.len()].1
        + numbers[(zero_index + 3000) % numbers.len()].1
}

pub fn solve_part_2(_input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 3);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 16533);
    }
}
