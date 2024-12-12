use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut line_split = line.split_whitespace();
        let left_ele: usize = line_split.next().unwrap().parse().unwrap();
        let right_ele: usize = line_split.next().unwrap().parse().unwrap();

        left.push(left_ele);
        right.push(right_ele);
    }

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum::<usize>()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut line_split = line.split_whitespace();
        let left_ele: usize = line_split.next().unwrap().parse().unwrap();
        let right_ele: usize = line_split.next().unwrap().parse().unwrap();

        left.push(left_ele);
        right.push(right_ele);
    }

    let tally = right.into_iter().counts();

    left.iter().map(|l| l * tally.get(l).or(Some(&0)).unwrap()).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day1/test_input")), 11);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day1/input")), 1320851);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day1/test_input")), 31);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day1/input")), 26859182);
    }
}
