use itertools::Itertools;

use crate::utils::print_2d_vec;

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
    println!("Comparing cols {} and {}", i, j);
    (0..table.len()).all(|x| {
        // dbg!(table[x][i]);
        // dbg!(table[x][j]);
        table[x][i] == table[x][j]
    })
}

fn rows_equal(table: &Table, i: usize, j: usize) -> bool {
    println!("Comparing rows {} and {}", i, j);
    table[i] == table[j]
}

fn solve(tuple: (usize, &Table)) -> usize {
    let (i, table) = tuple;
    println!();
    println!("Processing table {}", i);

    print_2d_vec(table);

    let x_len = table.len();
    let y_len = table[0].len();

    dbg!(x_len);
    dbg!(y_len);

    if let Some(col) = (0..y_len - 1)
        .filter(|col| cols_equal(table, *col, col + 1))
        .find(|col| {
            println!("Cols {} and {} are equal", col, col + 1);
            let mut mirror = true;

            let mut offset = 1;
            while mirror && *col >= offset && col + offset + 1 < y_len {
                if !cols_equal(table, col - offset, col + 1 + offset) {
                    println!("Cols {} and {} are NOT mirrored", col - offset, col + offset + 1);
                    mirror = false;
                }
                offset += 1;
                dbg!(offset);
            }

            if mirror {
                println!("Mirrored at cols {} and {}", col, col + 1);
            }

            mirror
        })
    {
        return col + 1;
    }

    if let Some(row) = (0..x_len - 1)
        .filter(|row| rows_equal(table, *row, row + 1))
        .find(|row| {
            println!("Rows {} and {} are equal", row, row + 1);
            let mut mirror = true;

            let mut offset = 1;
            while mirror && *row >= offset && row + offset + 1 < x_len {
                if !rows_equal(table, row - offset, row + 1 + offset) {
                    println!("Rows {} and {} are NOT mirrored", row - offset, row + offset);
                    mirror = false;
                }
                offset += 1;
                dbg!(offset);
            }

            if mirror {
                println!("Mirrored at rows {} and {}", row, row + 1);
            }

            mirror
        })
    {
        return (row + 1) * 100;
    }

    panic!("Did not find mirrored col or row");
}

pub fn solve_part_1(input: &str) -> usize {
    parse_input(input).iter().enumerate().map(solve).sum()
}

pub fn solve_part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day13/test_input")), 405);
    }

    #[test]
    fn part1_real_input() {
        // 21789 is too low
        assert_eq!(super::solve_part_1(&include_str!("day13/input")), 30_575);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day13/test_input")), 400);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day13/input")), 34_655_848);
    }
}
