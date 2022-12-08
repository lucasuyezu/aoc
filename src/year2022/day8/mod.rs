pub fn solve_part_1(lines: &[String]) -> usize {
    let mut height_grid: Vec<Vec<u32>> = vec![];
    let mut visible_grid: Vec<Vec<bool>> = vec![];

    for line in lines {
        height_grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        visible_grid.push(vec![false; line.len()]);
    }

    let row_count = height_grid.len();
    for i in 0..row_count {
        for j in 0..row_count {
            if i == 0 || j == 0 || i == row_count - 1 || j == row_count - 1 {
                visible_grid[i][j] = true;
            } else {
                let current_height = height_grid[i][j];
                let visible_top = (0..i).all(|k| height_grid[k][j] < current_height);
                let visible_left = (0..j).all(|k| height_grid[i][k] < current_height);
                let visible_right = (j + 1..row_count).all(|k| height_grid[i][k] < current_height);
                let visible_bottom = (i + 1..row_count).all(|k| height_grid[k][j] < current_height);

                visible_grid[i][j] = visible_top || visible_left || visible_right || visible_bottom;
            }
        }
    }

    return visible_grid.iter().flatten().filter(|cell| **cell).count();
}

pub fn solve_part_2(lines: &[String]) -> usize {
    let mut height_grid: Vec<Vec<u32>> = vec![];
    let mut scenic_score_grid: Vec<Vec<usize>> = vec![];

    for line in lines {
        height_grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        scenic_score_grid.push(vec![0; line.len()]);
    }

    let row_count = height_grid.len();
    for i in 0..height_grid.len() {
        for j in 0..height_grid.len() {
            if i == 0 || j == 0 || i == row_count - 1 || j == row_count - 1 {
                scenic_score_grid[i][j] = 0;
            } else {
                let current_height = height_grid[i][j];
                let mut scenic_score_top = 0;
                let mut scenic_score_left = 0;
                let mut scenic_score_right = 0;
                let mut scenic_score_bottom = 0;

                // look up
                for k in (0..i).rev() {
                    scenic_score_top += 1;

                    if height_grid[k][j] >= current_height {
                        break;
                    }
                }

                // look left
                for k in (0..j).rev() {
                    scenic_score_left += 1;

                    if height_grid[i][k] >= current_height {
                        break;
                    }
                }

                // look right
                for k in j + 1..height_grid[0].len() {
                    scenic_score_right += 1;

                    if height_grid[i][k] >= current_height {
                        break;
                    }
                }

                // look down
                for k in i + 1..height_grid.len() {
                    scenic_score_bottom += 1;

                    if height_grid[k][j] >= current_height {
                        break;
                    }
                }

                scenic_score_grid[i][j] =
                    scenic_score_top * scenic_score_left * scenic_score_right * scenic_score_bottom;
            }
        }
    }

    return scenic_score_grid.iter().flatten().max().unwrap().clone();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        let lines: Vec<String> = include_str!("test_input")
            .lines()
            .map(|str| str.to_string())
            .collect();
        assert_eq!(super::solve_part_1(&lines), 21);
    }

    #[test]
    fn part1_real_input() {
        let lines: Vec<String> = include_str!("input")
            .lines()
            .map(|str| str.to_string())
            .collect();
        assert_eq!(super::solve_part_1(&lines), 1_713);
    }

    #[test]
    fn part2_test_input() {
        let lines: Vec<String> = include_str!("test_input")
            .lines()
            .map(|str| str.to_string())
            .collect();
        assert_eq!(super::solve_part_2(&lines), 8);
    }

    #[test]
    fn part2_real_input() {
        let lines: Vec<String> = include_str!("input")
            .lines()
            .map(|str| str.to_string())
            .collect();
        assert_eq!(super::solve_part_2(&lines), 268_464);
    }
}
