use self::hand::Hand;

mod camel_cards;
mod hand;

pub fn solve_part_1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .lines()
        .enumerate()
        .map(|(i, line)| format!("{i} {line}").parse::<Hand>().unwrap())
        .collect();
    hands.sort_by(camel_cards::cmp);
    hands.iter().enumerate().map(|(i, hand)| hand.bid * (i + 1)).sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut hands: Vec<Hand> = input
        .replace("J", "X")
        .lines()
        .enumerate()
        .map(|(i, line)| format!("{i} {line}").parse::<Hand>().unwrap())
        .collect();
    hands.sort_by(camel_cards::cmp);
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
