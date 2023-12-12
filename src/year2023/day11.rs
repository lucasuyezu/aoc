use crate::utils::point::Point;
use itertools::Itertools;

fn parse_input(input: &str, expansion_factor: usize) -> Vec<Point> {
    let mut galaxies: Vec<Point> = vec![];
    for (x, row) in input.lines().enumerate() {
        for (y, c) in row.chars().enumerate() {
            if c == '#' {
                galaxies.push(Point {
                    x: (x as isize) + 1,
                    y: (y as isize) + 1,
                });
            }
        }
    }

    let x_len = input.lines().count();
    let y_len = input.lines().next().unwrap().len();

    let empty_rows: Vec<usize> = input
        .lines()
        .enumerate()
        .filter(|(_, line)| line.chars().all(|c| c == '.'))
        .map(|(x, _)| x + 1)
        .collect();

    let empty_cols: Vec<usize> = (0..y_len)
        .filter(|y| (0..x_len).all(|x| input.lines().nth(x).unwrap().chars().nth(*y).unwrap() == '.'))
        .map(|y| y + 1)
        .collect();

    for (offset, x) in empty_rows.iter().enumerate() {
        let min_x = x + offset * expansion_factor;
        for galaxy in galaxies.iter_mut().filter(|galaxy| galaxy.x as usize >= min_x) {
            galaxy.x += expansion_factor as isize;
        }
    }

    for (offset, y) in empty_cols.iter().enumerate() {
        let min_y = y + offset * expansion_factor;
        for galaxy in galaxies.iter_mut().filter(|galaxy| galaxy.y as usize >= min_y) {
            galaxy.y += expansion_factor as isize;
        }
    }

    galaxies
}

pub fn solve_part_1(input: &str) -> usize {
    let galaxies = parse_input(input, 1);

    galaxies
        .iter()
        .combinations(2)
        .unique()
        .map(|pair| pair[0].manhattan_distance(pair[1]))
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let galaxies = parse_input(input, 999_999);

    galaxies
        .iter()
        .combinations(2)
        .unique()
        .map(|pair| pair[0].manhattan_distance(pair[1]))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use itertools::Itertools;

    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day11/test_input")), 374);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day11/input")), 10_292_708);
    }

    #[test]
    fn part2_test_input() {
        let galaxies = parse_input(&include_str!("day11/test_input"), 9);

        let x: usize = galaxies
            .iter()
            .combinations(2)
            .unique()
            .map(|pair| pair.get(0).unwrap().manhattan_distance(pair.get(1).unwrap()))
            .sum();

        assert_eq!(x, 1030);

        let galaxies = parse_input(&include_str!("day11/test_input"), 99);

        let x: usize = galaxies
            .iter()
            .combinations(2)
            .unique()
            .map(|pair| pair.get(0).unwrap().manhattan_distance(pair.get(1).unwrap()))
            .sum();

        assert_eq!(x, 8410);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day11/input")), 790_194_712_336);
    }
}
