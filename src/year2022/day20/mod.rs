fn solve(input: &str, decryption_key: isize, num_rounds: u8) -> isize {
    let mut numbers: Vec<(usize, isize)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| (i, decryption_key * line.parse::<isize>().unwrap()))
        .collect();

    for _ in 0..num_rounds {
        for i in 0..numbers.len() {
            let index = numbers.iter().position(|(j, _)| i == *j).unwrap();
            let tuple = numbers[index];

            numbers.remove(index);
            let new_index: usize =
                (tuple.1 + index as isize).rem_euclid(numbers.len() as isize) as usize;

            numbers.insert(new_index, tuple);
        }
    }

    let zero_index = numbers.iter().position(|(_, n)| *n == 0).unwrap();

    numbers[(zero_index + 1000) % numbers.len()].1
        + numbers[(zero_index + 2000) % numbers.len()].1
        + numbers[(zero_index + 3000) % numbers.len()].1
}

pub fn solve_part_1(input: &str) -> isize {
    solve(input, 1, 1)
}
pub fn solve_part_2(input: &str) -> isize {
    solve(input, 811589153, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part_1(&include_str!("test_input")), 3);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(solve_part_1(&include_str!("input")), 16533);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(solve_part_2(&include_str!("test_input")), 1623178306);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(solve_part_2(&include_str!("input")), 4789999181006);
    }
}
