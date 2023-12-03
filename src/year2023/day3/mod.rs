use crate::utils::point::Point;

#[derive(Debug)]
struct Number {
    n: usize,
    points: Vec<Point>,
}

#[derive(Debug)]
pub struct Symbol {
    c: char,
    pos: Point,
}
impl Symbol {
    fn is_touching(&self, n: &Number) -> bool {
        n.points.iter().any(|p| p.is_touching(self.pos))
    }

    fn gear_ratio(&self, nums: &Vec<Number>) -> usize {
        if self.c != '*' {
            return 0;
        }

        let adj_nums: Vec<&Number> = nums.iter().filter(|n| self.is_touching(n)).collect();

        if adj_nums.len() != 2 {
            return 0;
        }

        adj_nums[0].n * adj_nums[1].n
    }
}

fn parse_input(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers: Vec<Number> = vec![];
    let mut symbols: Vec<Symbol> = vec![];

    let mut current_number = Number { n: 0, points: vec![] };
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let current_point = Point {
                x: x as isize,
                y: y as isize,
            };
            if c.is_digit(10) {
                current_number.n = current_number.n * 10 + c.to_digit(10).unwrap() as usize;
                current_number.points.push(current_point);
            } else {
                if current_number.n != 0 {
                    numbers.push(current_number);
                    current_number = Number { n: 0, points: vec![] };
                }
                if c != '.' {
                    symbols.push(Symbol { c, pos: current_point });
                }
            }
        }
    }

    (numbers, symbols)
}

pub fn solve_part_1(input: &str) -> usize {
    let (numbers, symbols) = parse_input(input);

    let touching: Vec<&Number> = numbers
        .iter()
        .filter(|n| symbols.iter().any(|s| s.is_touching(n)))
        .collect();

    touching.iter().map(|n| n.n).sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let (numbers, symbols) = parse_input(input);

    symbols.iter().map(|s| s.gear_ratio(&numbers)).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 4361);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 538_046);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 467_835);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 81_709_807);
    }
}
