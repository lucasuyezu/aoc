#[derive(Debug)]
pub struct SlopeMap {
    map: Vec<String>,
}

impl SlopeMap {
    fn traverse(&self, right_slope: usize, down_slope: usize) -> usize {
        let row_count = self.map.len();
        let column_count = self.map[0].len();
        // println!("row_count={} column_count={}", row_count, column_count);

        let mut current_row = 0;
        let mut current_column = 0;
        let mut trees_touched = 0;
        let mut cell: char;

        while current_row + 1 < row_count {
            current_row += down_slope;
            current_column += right_slope;

            if current_column >= column_count {
                current_column %= column_count;
            }

            let row = &self.map[current_row];

            cell = row.chars().nth(current_column).unwrap();
            if cell == '#' {
                trees_touched += 1;
            }

            // println!("row={} current_row={} current_column={} cell={} trees_touched={}", row, current_row, current_column, cell, trees_touched);
        }

        trees_touched
    }
}

pub fn solve_part_1(lines: &[String]) -> usize {
    let slope_map = get_structs(lines);

    slope_map.traverse(3, 1)
}

pub fn solve_part_2(lines: &[String]) -> usize {
    let slope_map = get_structs(lines);

    let result_1 = slope_map.traverse(1, 1);
    let result_2 = slope_map.traverse(3, 1);
    let result_3 = slope_map.traverse(5, 1);
    let result_4 = slope_map.traverse(7, 1);
    let result_5 = slope_map.traverse(1, 2);

    result_1 * result_2 * result_3 * result_4 * result_5
}

fn get_structs(lines: &[String]) -> SlopeMap {
    SlopeMap {
        map: lines.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day3/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 7);
    }

    #[test]
    fn part2_test_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day3/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 336);
    }

    #[test]
    fn part1_real_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day3/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 164);
    }

    #[test]
    fn part2_real_input() {
        let lines = super::super::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day3/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_2(&lines), 5007658656);
    }
}
