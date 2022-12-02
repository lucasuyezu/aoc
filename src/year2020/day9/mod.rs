pub fn solve_part_1(lines: &[String]) -> usize {
    let numbers: Vec<usize> = lines
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    invalid_number(&numbers, 25)
}

pub fn solve_part_2(lines: &[String]) -> usize {
    let numbers: Vec<usize> = lines
        .iter()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let invalid_number = invalid_number(&numbers, 25);
    encryption_weakness(&numbers, invalid_number)
}

fn invalid_number(numbers: &[usize], preamble_size: usize) -> usize {
    let mut preamble = Vec::<usize>::with_capacity(preamble_size + 1);

    for number in numbers.iter().take(preamble_size) {
        preamble.push(*number);
    }

    'outer: for number in &numbers[preamble_size..] {
        // dbg!(&number);
        // dbg!(&preamble);

        for preamble_item_1 in preamble.iter() {
            for preamble_item_2 in preamble.iter() {
                // dbg!(&preamble_item_1);
                // dbg!(&preamble_item_2);
                if preamble_item_1 == preamble_item_2 {
                    continue;
                }

                if preamble_item_1 + preamble_item_2 == *number {
                    // println!(
                    //     "{} is valid because it is the sum of {} and {}",
                    //     *number, preamble_item_1, preamble_item_2
                    // );

                    preamble.push(*number);
                    preamble.remove(0);
                    continue 'outer;
                }
            }
        }

        return *number;
    }

    panic!("Should have found a number by now");
}

fn encryption_weakness(numbers: &[usize], invalid_number: usize) -> usize {
    // dbg!(&numbers);
    // dbg!(&invalid_number);

    'outer: for i in 0..numbers.len() {
        let left_number = numbers[i];

        let mut acc = left_number;
        // println!("start: i={} left_number={} acc={}", i, left_number, acc);

        if left_number > invalid_number {
            // println!("Continuing because {} > {}", acc, invalid_number);
            continue;
        }

        for j in i + 1..numbers.len() {
            let right_number = numbers[j];
            acc += right_number;
            // println!("j={}, right_number={}, acc={}", j, right_number, acc);

            if acc == invalid_number {
                let range = numbers[i..j].to_vec();
                // dbg!(&range);
                let smallest = *range.iter().min().unwrap();
                let largest = *range.iter().max().unwrap();

                return smallest + largest;
            }

            if acc > invalid_number {
                // println!("Continuing because {} > {}", acc, invalid_number);
                continue 'outer;
            }
        }
    }

    panic!("Should have found a range")
}

#[cfg(test)]
mod tests {
    use super::super::super::utils;

    #[test]
    fn part1_test_input() {
        let lines = utils::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day9/test_input"
                .to_string(),
        );

        let numbers: Vec<usize> = lines
            .iter()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();

        assert_eq!(super::invalid_number(&numbers, 5), 127);
    }

    #[test]
    fn part2_test_input() {
        let lines = utils::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day9/test_input"
                .to_string(),
        );

        let numbers: Vec<usize> = lines
            .iter()
            .map(|line| line.parse::<usize>().unwrap())
            .collect();

        assert_eq!(super::encryption_weakness(&numbers, 127), 62);
    }

    #[test]
    fn part1_real_input() {
        let lines = utils::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day9/input"
                .to_string(),
        );

        assert_eq!(super::solve_part_1(&lines), 69316178);
    }

    #[test]
    fn part2_real_input() {
        let lines = utils::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day9/input"
                .to_string(),
        );

        assert_eq!(super::solve_part_2(&lines), 9351526);
    }
}
