#[derive(Debug)]
enum Instruction {
    Move(u16),
    RotateLeft,
    RotateRight,
}

const RIGHT: isize = 0;
const DOWN: isize = 1;
const LEFT: isize = 2;
const UP: isize = 3;

type Grid = Vec<Vec<char>>;

fn parse_input(input: &str) -> Grid {
    let max_line_len = input.lines().map(|line| line.len()).max().unwrap();

    input
        .lines()
        .map(|line| {
            let mut line_vec = line.chars().collect::<Vec<char>>();
            while line_vec.len() < max_line_len {
                line_vec.push(' ');
            }
            line_vec
        })
        .collect()
}

fn parse_instructions(instructions_str: &str) -> Vec<Instruction> {
    let mut instructions = vec![];
    let mut current_number: u16 = 0;

    for char in instructions_str.trim().chars() {
        if char.is_digit(10) {
            current_number *= 10;
            current_number += char.to_digit(10).unwrap() as u16;
        } else {
            if current_number != 0 {
                instructions.push(Instruction::Move(current_number));
                current_number = 0;
            }
            match char {
                'L' => instructions.push(Instruction::RotateLeft),
                'R' => instructions.push(Instruction::RotateRight),
                _ => panic!("Invalid instruction!"),
            }
        }
    }

    if current_number != 0 {
        instructions.push(Instruction::Move(current_number));
    }

    instructions
}

fn solve(grid: Grid, instructions: Vec<Instruction>, is_grid: bool) -> usize {
    let mut row = 0;
    let mut col = grid[0].iter().position(|line| *line == '.').unwrap();
    let mut direction: isize = 0;

    for instruction in instructions {
        match instruction {
            Instruction::RotateLeft => {
                direction -= 1;
                direction = direction.rem_euclid(4);
            }
            Instruction::RotateRight => {
                direction += 1;
                direction = direction.rem_euclid(4);
            }
            Instruction::Move(steps) => {
                for _ in 0..steps {
                    let (next_row, next_col, next_direction) = next_position(&grid, row, col, direction, is_grid);

                    if grid[next_row][next_col] == '#' {
                        break;
                    } else {
                        row = next_row;
                        col = next_col;
                        direction = next_direction;
                    }
                }
            }
        }
    }

    1000 * (row + 1) + 4 * (col + 1) + direction as usize
}

fn next_position(grid: &Grid, row: usize, col: usize, direction: isize, is_grid: bool) -> (usize, usize, isize) {
    if is_at_border(grid, row, col, direction) {
        if is_grid {
            wrap_around_grid(&grid, row, col, direction)
        } else {
            wrap_around_cube(row, col, direction)
        }
    } else {
        match direction {
            RIGHT => (row, col + 1, direction),
            DOWN => (row + 1, col, direction),
            LEFT => (row, col - 1, direction),
            UP => (row - 1, col, direction),
            d @ _ => panic!("Unsupported direction {}", d),
        }
    }
}

fn is_at_border(grid: &Grid, row: usize, col: usize, direction: isize) -> bool {
    match direction {
        RIGHT => col == grid[row].len() - 1 || grid[row][col + 1] == ' ',
        DOWN => row == grid.len() - 1 || grid[row + 1][col] == ' ',
        LEFT => col == 0 || grid[row][col - 1] == ' ',
        UP => row == 0 || grid[row - 1][col] == ' ',
        d @ _ => panic!("Unsupported direction {}", d),
    }
}

fn wrap_around_grid(grid: &Grid, row: usize, col: usize, direction: isize) -> (usize, usize, isize) {
    let mut next_row = row;
    let mut next_col = col;

    match direction {
        RIGHT => {
            next_col = grid[row].iter().position(|c| *c != ' ').unwrap();
        }
        DOWN => {
            for i in 0..grid.len() {
                if grid[i][col] != ' ' {
                    next_row = i;
                    break;
                }
            }
        }
        LEFT => {
            for i in (0..grid[row].len()).rev() {
                if grid[row][i] != ' ' {
                    next_col = i;
                    break;
                }
            }
        }
        UP => {
            for i in (0..grid.len()).rev() {
                if grid[i][col] != ' ' {
                    next_row = i;
                    break;
                }
            }
        }
        d @ _ => panic!("Unsupported direction {}", d),
    }

    (next_row, next_col, direction)
}

fn wrap_around_cube(row: usize, col: usize, direction: isize) -> (usize, usize, isize) {
    let current_quadrant = if row < 50 {
        if col < 100 {
            1
        } else {
            2
        }
    } else if row < 100 {
        3
    } else if row < 150 {
        if col < 50 {
            4
        } else {
            5
        }
    } else {
        6
    };

    let new_row;
    let new_col;
    let new_direction;

    if current_quadrant == 1 && direction == UP {
        new_row = col + 100;
        new_col = 0;
        new_direction = RIGHT;
    } else if current_quadrant == 1 && direction == LEFT {
        new_row = 149 - row;
        new_col = 0;
        new_direction = RIGHT;
    } else if current_quadrant == 2 && direction == UP {
        new_row = 199;
        new_col = col - 100;
        new_direction = UP;
    } else if current_quadrant == 2 && direction == RIGHT {
        new_row = 149 - row;
        new_col = 99;
        new_direction = LEFT;
    } else if current_quadrant == 2 && direction == DOWN {
        new_row = col - 50;
        new_col = 99;
        new_direction = LEFT;
    } else if current_quadrant == 3 && direction == LEFT {
        new_row = 100;
        new_col = row - 50;
        new_direction = DOWN;
    } else if current_quadrant == 3 && direction == RIGHT {
        new_row = 49;
        new_col = row + 50;
        new_direction = UP;
    } else if current_quadrant == 4 && direction == UP {
        new_row = col + 50;
        new_col = 50;
        new_direction = RIGHT;
    } else if current_quadrant == 4 && direction == LEFT {
        new_row = 149 - row;
        new_col = 50;
        new_direction = RIGHT;
    } else if current_quadrant == 5 && direction == RIGHT {
        new_row = 149 - row;
        new_col = 149;
        new_direction = LEFT;
    } else if current_quadrant == 5 && direction == DOWN {
        new_row = col + 100;
        new_col = 49;
        new_direction = LEFT;
    } else if current_quadrant == 6 && direction == RIGHT {
        new_row = 149;
        new_col = row - 100;
        new_direction = UP;
    } else if current_quadrant == 6 && direction == DOWN {
        new_row = 0;
        new_col = col + 100;
        new_direction = DOWN;
    } else if current_quadrant == 6 && direction == LEFT {
        new_row = 0;
        new_col = row - 100;
        new_direction = DOWN;
    } else {
        panic!();
    }

    (new_row, new_col, new_direction)
}

pub fn solve_part_1(input: &str) -> usize {
    let (grid_str, instructions_str) = input.split_once("\n\n").unwrap();
    let grid = parse_input(grid_str);
    let instructions = parse_instructions(instructions_str);

    solve(grid, instructions, true)
}

pub fn solve_part_2(input: &str) -> usize {
    let (cube_str, instructions_str) = input.split_once("\n\n").unwrap();
    let cube = parse_input(cube_str);
    let instructions = parse_instructions(instructions_str);

    solve(cube, instructions, false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::year2022::day22::{LEFT, RIGHT, UP};

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part_1(&include_str!("test_input")), 6032);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(solve_part_1(&include_str!("input")), 65368);
    }

    #[test]
    fn part2_test_wrap_around_cube() {
        assert_eq!(wrap_around_cube(0, 50, UP), (150, 0, RIGHT));
        assert_eq!(wrap_around_cube(0, 99, UP), (199, 0, RIGHT));
        assert_eq!(wrap_around_cube(0, 50, LEFT), (149, 0, RIGHT));
        assert_eq!(wrap_around_cube(49, 50, LEFT), (100, 0, RIGHT));

        assert_eq!(wrap_around_cube(0, 100, UP), (199, 0, UP));
        assert_eq!(wrap_around_cube(0, 149, UP), (199, 49, UP));
        assert_eq!(wrap_around_cube(0, 149, RIGHT), (149, 99, LEFT));
        assert_eq!(wrap_around_cube(49, 149, RIGHT), (100, 99, LEFT));
        assert_eq!(wrap_around_cube(49, 100, DOWN), (50, 99, LEFT));
        assert_eq!(wrap_around_cube(49, 149, DOWN), (99, 99, LEFT));

        assert_eq!(wrap_around_cube(50, 50, LEFT), (100, 0, DOWN));
        assert_eq!(wrap_around_cube(99, 50, LEFT), (100, 49, DOWN));
        assert_eq!(wrap_around_cube(50, 99, RIGHT), (49, 100, UP));
        assert_eq!(wrap_around_cube(99, 99, RIGHT), (49, 149, UP));

        assert_eq!(wrap_around_cube(100, 0, UP), (50, 50, RIGHT));
        assert_eq!(wrap_around_cube(100, 49, UP), (99, 50, RIGHT));
        assert_eq!(wrap_around_cube(100, 0, LEFT), (49, 50, RIGHT));
        assert_eq!(wrap_around_cube(149, 0, LEFT), (0, 50, RIGHT));

        assert_eq!(wrap_around_cube(100, 99, RIGHT), (49, 149, LEFT));
        assert_eq!(wrap_around_cube(149, 99, RIGHT), (0, 149, LEFT));
        assert_eq!(wrap_around_cube(149, 50, DOWN), (150, 49, LEFT));
        assert_eq!(wrap_around_cube(149, 50, DOWN), (150, 49, LEFT));
        assert_eq!(wrap_around_cube(149, 99, DOWN), (199, 49, LEFT));

        assert_eq!(wrap_around_cube(150, 49, RIGHT), (149, 50, UP));
        assert_eq!(wrap_around_cube(199, 49, RIGHT), (149, 99, UP));
        assert_eq!(wrap_around_cube(199, 0, DOWN), (0, 100, DOWN));
        assert_eq!(wrap_around_cube(199, 49, DOWN), (0, 149, DOWN));
        assert_eq!(wrap_around_cube(150, 0, LEFT), (0, 50, DOWN));
        assert_eq!(wrap_around_cube(199, 0, LEFT), (0, 99, DOWN));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(solve_part_2(&include_str!("input")), 156166);
    }
}
