pub fn solve_part_1(input: &str) -> usize {
    let mut acc = 0;
    let mut max = 0;
    for line in input.lines() {
        if line != "" {
            acc += line.parse::<usize>().unwrap();
        }
        else {
            if acc > max {
                max = acc;
            }
            acc = 0;
        }
    }
    
    if acc > max {
        max = acc;
    }

    return max;
}

pub fn solve_part_2(input: &str) -> usize {
    let mut vec: Vec<usize> = Vec::new();
    let mut acc = 0;
    for line in input.lines() {
        if line != "" {
            acc += line.parse::<usize>().unwrap();
        }
        else {
            vec.push(acc);
            acc = 0;
        }
    }

    vec.push(acc);
    
    vec.sort_unstable();
    vec.reverse();
    return vec[0] + vec[1] + vec[2];
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 24_000);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 69_883);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 45_000);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 207_576);
    }
}
