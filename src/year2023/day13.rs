use itertools::Itertools;

type Table = Vec<Vec<char>>;

fn parse_input(input: &str) -> Vec<Table> {
    input
        .split("\n\n")
        .map(|table| {
            table
                .lines()
                .map(|table_line| table_line.chars().collect_vec())
                .collect_vec()
        })
        .collect_vec()
}

fn cols_equal(table: &Table, i: usize, j: usize) -> bool {
    (0..table.len()).all(|x| table[x][i] == table[x][j])
}

fn rows_equal(table: &Table, i: usize, j: usize) -> bool {
    table[i] == table[j]
}

fn solve(
    table: &Table,
    ignore_x: Option<usize>,
    ignore_y: Option<usize>,
) -> Option<(Option<usize>, Option<usize>, usize)> {
    // TODO: Transpose and DRY this.
    let x_len = table.len();
    let y_len = table[0].len();

    if let Some(y) = (0..y_len - 1).filter(|y| cols_equal(table, *y, y + 1)).find(|y| {
        let mut mirror = true;

        let mut offset = 1;
        while mirror && *y >= offset && y + offset + 1 < y_len {
            if !cols_equal(table, y - offset, y + 1 + offset) {
                mirror = false;
            }
            offset += 1;
        }

        if mirror && ignore_y.is_some() && ignore_y.unwrap() == *y {
            mirror = false;
        }

        mirror
    }) {
        return Some((None, Some(y), y + 1));
    }

    if let Some(x) = (0..x_len - 1).filter(|y| rows_equal(table, *y, y + 1)).find(|x| {
        let mut mirror = true;

        let mut offset = 1;
        while mirror && *x >= offset && x + offset + 1 < x_len {
            if !rows_equal(table, x - offset, x + 1 + offset) {
                mirror = false;
            }
            offset += 1;
        }

        if mirror && ignore_x.is_some() && ignore_x.unwrap() == *x {
            mirror = false;
        }

        mirror
    }) {
        return Some((Some(x), None, (x + 1) * 100));
    }

    None
}

pub fn solve_part_1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|table| solve(table, None, None).unwrap().2)
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|table| (table, solve(&table, None, None).unwrap()))
        .map(|(table, (result_x, result_y, result))| {
            for x in 0..table.len() {
                for y in 0..table[0].len() {
                    let mut new_table = table.clone();
                    if new_table[x][y] == '.' {
                        new_table[x][y] = '#';
                    } else {
                        new_table[x][y] = '.';
                    }
                    if let Some((_, _, new_result)) = solve(&new_table, result_x, result_y) {
                        if new_result == result {
                            panic!("Results should have been different");
                        }
                        return new_result;
                    }
                }
            }

            panic!("Should have found a new result");
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day13/test_input")), 405);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day13/input")), 30_575);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day13/test_input")), 400);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day13/input")), 37_478);
    }
}
