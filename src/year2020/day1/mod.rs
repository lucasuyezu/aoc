fn parse_input(input: &str) -> Vec<usize> {
    let numbers: Vec<usize> = input
        .lines()
        .map(|it| it.parse::<usize>().unwrap())
        .collect();

    numbers
}

pub fn solve_part_1(input: &str) -> usize {
    let mut numbers = parse_input(input);

    numbers.sort_unstable();

    let mut i = 0;
    let mut j = 1;

    while i < numbers.len() - 1 {
        while j < numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return numbers[i] * numbers[j];
            } else {
            }
            j += 1;
        }
        j = 1;
        i += 1;
    }

    panic!();
}

pub fn solve_part_2(input: &str) -> usize {
    let mut numbers = parse_input(input);

    numbers.sort_unstable();

    let mut i = 0;
    let mut j = 1;
    let mut k = 2;

    while i < numbers.len() - 2 {
        while j < numbers.len() - 1 {
            if numbers[i] + numbers[j] >= 2020 {
                k = 1;
                j += 1;
                continue;
            }

            while k < numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return numbers[i] * numbers[j] * numbers[k];
                }

                k += 1;
            }

            k = 1;
            j += 1;
        }

        k = 2;
        j = 1;
        i += 1;
    }

    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part_1(&include_str!("test_input")), 514579);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(solve_part_1(&include_str!("input")), 1014171);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(solve_part_2(&include_str!("test_input")), 241861950);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(solve_part_2(&include_str!("input")), 46584630);
    }
}
