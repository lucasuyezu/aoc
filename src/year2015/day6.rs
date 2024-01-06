use regex::Regex;

pub fn solve_part_1(input: &str) -> usize {
    let re = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut grid = vec![vec![false; 1_000]; 1_000];

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let min_x: usize = captures[1].parse().unwrap();
        let min_y: usize = captures[2].parse().unwrap();
        let max_x: usize = captures[3].parse().unwrap();
        let max_y: usize = captures[4].parse().unwrap();

        if line.starts_with("turn on") {
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    grid[x][y] = true;
                }
            }
        } else if line.starts_with("turn off") {
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    grid[x][y] = false;
                }
            }
        } else {
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    grid[x][y] = !grid[x][y];
                }
            }
        }
    }

    grid.into_iter()
        .map(|row| row.into_iter().filter(|cell| *cell).count())
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let re = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)").unwrap();
    let mut grid = vec![vec![0 as usize; 1_000]; 1_000];

    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let min_x: usize = captures[1].parse().unwrap();
        let min_y: usize = captures[2].parse().unwrap();
        let max_x: usize = captures[3].parse().unwrap();
        let max_y: usize = captures[4].parse().unwrap();

        if line.starts_with("turn on") {
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    grid[x][y] += 1;
                }
            }
        } else if line.starts_with("turn off") {
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    grid[x][y] = if grid[x][y] > 0 { grid[x][y] - 1 } else { 0 };
                }
            }
        } else {
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    grid[x][y] += 2;
                }
            }
        }
    }

    grid.into_iter().map(|row| row.into_iter().sum::<usize>()).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day6/test_input")), 1_000_000);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day6/input")), 377_891);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&"turn on 0,0 through 0,0"), 1);
        assert_eq!(super::solve_part_2(&"toggle 0,0 through 999,999"), 2_000_000);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day6/input")), 14_110_788);
    }
}
