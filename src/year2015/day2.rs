fn parse_input(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split("x");

            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

pub fn solve_part_1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|(l, w, h)| {
            let surface_area = 2 * l * w + 2 * w * h + 2 * h * l;

            surface_area + (l * w).min(w * h).min(h * l)
        })
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|(l, w, h)| {
            let perimeter = if l.max(w).max(h) == l {
                w + w + h + h
            } else if l.max(w).max(h) == w {
                l + l + h + h
            } else {
                l + l + w + w
            };

            let volume = l * w * h;

            perimeter + volume
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day2/test_input")), 101);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day2/input")), 1_588_178);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day2/test_input")), 48);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day2/input")), 3_783_758);
    }
}
