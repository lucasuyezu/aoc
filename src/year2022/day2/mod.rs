pub fn solve_part_1(input: &str) -> usize {
    let mut result = 0;
    
    for line in input.lines() {
        match line {
            "A X" => result += 1 + 3,
            "A Y" => result += 2 + 6,
            "A Z" => result += 3 + 0,
            "B X" => result += 1 + 0,
            "B Y" => result += 2 + 3,
            "B Z" => result += 3 + 6,
            "C X" => result += 1 + 6,
            "C Y" => result += 2 + 0,
            "C Z" => result += 3 + 3,
            _ => panic!("Invalid line {}", line),
        }
    }

    return result;
}

pub fn solve_part_2(input: &str) -> usize {
    let mut result = 0;
    
    for line in input.lines() {
        match line {
            "A X" => result += 3 + 0,
            "A Y" => result += 1 + 3,
            "A Z" => result += 2 + 6,
            "B X" => result += 1 + 0,
            "B Y" => result += 2 + 3,
            "B Z" => result += 3 + 6,
            "C X" => result += 2 + 0,
            "C Y" => result += 3 + 3,
            "C Z" => result += 1 + 6,
            _ => panic!("Invalid line {}", line),
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 15);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 11_603);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 12);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 12_725);
    }
}
