pub fn solve_part_1(numbers: &[usize]) -> usize {
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

    return 0;
}

pub fn solve_part_2(numbers: &[usize]) -> usize {
    let mut i = 0;
    let mut j = 1;
    let mut k = 2;

    while i < numbers.len() - 2 {
        while j < numbers.len() - 1 {
            while k < numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return numbers[i] * numbers[j] * numbers[k];
                } else {
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

    return 0;
}

pub fn get_structs(lines: Vec<String>) -> Vec<usize> {
    let numbers: Vec<usize> = lines.iter().map(|it| it.parse::<usize>().unwrap()).collect();

    return numbers;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day1/test_input"
                .to_string(),
        );
        let structs = super::get_structs(lines);

        assert_eq!(super::solve_part_1(structs.as_slice()), 514579);
    }

    #[test]
    fn part1_real_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day1/input"
                .to_string(),
        );
        let structs = super::get_structs(lines);

        assert_eq!(super::solve_part_1(structs.as_slice()), 1014171);
    }

    #[test]
    fn part2_test_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day1/test_input"
                .to_string(),
        );
        let structs = super::get_structs(lines);

        assert_eq!(super::solve_part_2(structs.as_slice()), 241861950);
    }

    #[test]
    fn part2_real_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day1/input"
                .to_string(),
        );
        let structs = super::get_structs(lines);

        assert_eq!(super::solve_part_2(structs.as_slice()), 46584630);
    }
}
