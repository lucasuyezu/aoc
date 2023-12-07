use self::camel_poker::Hand;

mod camel_poker;

pub fn solve_part_1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .replace("A", "F")
        .replace("K", "E")
        .replace("Q", "D")
        .replace("J", "C")
        .replace("T", "B")
        .lines()
        .map(|line| line.parse::<Hand>().unwrap())
        .collect();

    hands.sort();
    hands.iter().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .replace("A", "F")
        .replace("K", "E")
        .replace("Q", "D")
        .replace("T", "B")
        .replace("J", "0")
        .lines()
        .map(|line| line.parse::<Hand>().unwrap())
        .collect();

    hands.sort();
    hands.iter().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 6_440);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 251_029_473);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 5_905);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 251_003_917);
    }
}
