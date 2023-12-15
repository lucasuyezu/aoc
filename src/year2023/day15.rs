fn hash(s: &str) -> usize {
    let mut result = 0;

    for c in s.trim().chars() {
        let current_value = c as usize;
        result += current_value;
        result *= 17;
        result = result.rem_euclid(256);
    }

    println!("{} becomes {}", s, result);
    result
}
pub fn solve_part_1(input: &str) -> usize {
    input.split(",").map(hash).sum()
}

pub fn solve_part_2(_: &str) -> usize {
    1
}

#[cfg(test)]
mod tests {
    #[test]
    fn hash() {
        assert_eq!(super::hash("HASH"), 52);
        assert_eq!(super::hash("rn=1"), 30);
        assert_eq!(super::hash("cm-"), 253);
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day15/test_input")), 1_320);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day15/input")), 512_283);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day15/test_input")), 0);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day15/input")), 0);
    }
}
