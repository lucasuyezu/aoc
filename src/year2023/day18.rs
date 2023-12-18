use crate::utils::{geometry::Polygon, point::Point};

fn parse_input(s: &str, hex: bool) -> (Polygon, usize) {
    let mut current_point = Point::origin();
    let mut perimeter = 0;

    let vertices = s
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(" ").collect();

            let count: usize = if hex {
                usize::from_str_radix(&split[2][2..7], 16).unwrap()
            } else {
                split[1].parse().unwrap()
            };
            perimeter += count;

            let direction = if hex { &split[2][7..8] } else { split[0] };

            match direction {
                "R" | "0" => current_point.x += count as isize,
                "D" | "1" => current_point.y += count as isize,
                "L" | "2" => current_point.x -= count as isize,
                "U" | "3" => current_point.y -= count as isize,
                _ => panic!("Invalid direction"),
            };

            current_point.clone()
        })
        .collect::<Vec<_>>();

    (Polygon { vertices }, perimeter)
}

pub fn solve_part_1(input: &str) -> usize {
    let (polygon, perimeter) = parse_input(input, false);
    polygon.shoelace() as usize + perimeter / 2 + 1
}

pub fn solve_part_2(input: &str) -> usize {
    let (polygon, perimeter) = parse_input(input, true);
    polygon.shoelace() as usize + perimeter / 2 + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day18/test_input")), 62);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day18/input")), 35_244);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day18/test_input")), 952_408_144_115);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day18/input")), 85_070_763_635_666);
    }
}
