pub fn solve_part_1(input: &str) -> isize {
    let mut current_cycle: usize = 1;
    let mut x_register: isize = 1;
    let mut signal_strength: isize = 0;

    for instruction_line in input.lines() {
        let (cycle_count, value) = if instruction_line == "noop" {
            (1, 0)
        } else {
            let (_, value) = instruction_line.split_once(" ").unwrap();
            (2, value.parse::<isize>().unwrap())
        };

        for _ in 0..cycle_count {
            if current_cycle == 20 || (current_cycle as isize - 20) % 40 == 0 {
                signal_strength += current_cycle as isize * x_register;
            }
            current_cycle += 1;
        }

        x_register += value;
    }

    return signal_strength;
}

pub fn solve_part_2(input: &str) -> String {
    let mut x_register: isize = 1;
    let mut crt_image = String::new();
    let mut crt_position = 0;

    for instruction_line in input.lines() {
        let (cycle_count, value) = if instruction_line == "noop" {
            (1, 0)
        } else {
            let (_, value) = instruction_line.split_once(" ").unwrap();
            (2, value.parse::<isize>().unwrap())
        };

        for _ in 0..cycle_count {
            if crt_position != 0 && crt_position % 40 == 0 {
                crt_image += "\n";
                crt_position = 0;
            }

            if crt_position == x_register
                || crt_position == x_register - 1
                || crt_position == x_register + 1
            {
                crt_image += "#";
            } else {
                crt_image += ".";
            }

            crt_position += 1;
        }

        x_register += value;
    }

    return crt_image;
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 13140);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 13820);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            super::solve_part_2(&include_str!("test_input")),
            include_str!("test_result")
        );
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(
            super::solve_part_2(&include_str!("input")),
            include_str!("real_result")
        );
    }
}
