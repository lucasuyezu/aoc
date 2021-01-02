pub fn solve_part_1(lines: &[String]) -> usize {
    let numbers = get_structs(lines);

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

    0
}

pub fn solve_part_2(lines: &[String]) -> usize {
    let mut numbers = get_structs(lines);

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

    0
}

fn get_structs(lines: &[String]) -> Vec<usize> {
    let numbers: Vec<usize> = lines
        .iter()
        .map(|it| it.parse::<usize>().unwrap())
        .collect();

    numbers
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day1/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 514579);
    }

    #[test]
    fn part1_real_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day1/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 1014171);
    }

    #[test]
    fn part2_test_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day1/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 241861950);
    }

    #[test]
    fn part2_real_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day1/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 46584630);
    }
}
