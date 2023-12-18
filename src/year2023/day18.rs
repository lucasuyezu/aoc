use std::collections::VecDeque;

use crate::utils::{
    point::{Point, EAST, NORTH, SOUTH, WEST},
    print_2d_vec, Grid,
};

#[derive(Debug)]
struct DigPlan {
    direction: Point,
    count: usize,
    #[allow(dead_code)]
    hex: String,
}

fn parse_input(s: &str) -> Vec<DigPlan> {
    s.lines()
        .map(|line| {
            let mut split = line.split(" ");

            DigPlan {
                direction: direction(split.next().unwrap()),
                count: split.next().unwrap().parse().unwrap(),
                hex: split.next().unwrap().to_string(),
            }
        })
        .collect::<Vec<_>>()
}

fn floodfill(grid: &mut Grid<char>, pos: Point) {
    // println!("Floodfilling from {:?}", pos);

    if grid.data[pos.x as usize][pos.y as usize] != '.' {
        return;
    }

    print_2d_vec(&grid.data);

    let mut queue: VecDeque<Point> = VecDeque::new();
    queue.push_back(pos);

    while let Some(cur_pos) = queue.pop_front() {
        // println!("Floodfilling cell {:?}", pos);

        if grid.data[cur_pos.x as usize][cur_pos.y as usize] != '.' {
            // println!("Already floodfilled {:?}", cur_pos);
            continue;
        }

        grid.data[cur_pos.x as usize][cur_pos.y as usize] = 'X';

        let north = cur_pos + NORTH;
        if grid.is_inside(&north) && grid.data[north.x as usize][north.y as usize] == '.' {
            queue.push_back(north);
        }

        let east = cur_pos + EAST;
        if grid.is_inside(&east) && grid.data[east.x as usize][east.y as usize] == '.' {
            queue.push_back(east);
        }

        let south = cur_pos + SOUTH;
        if grid.is_inside(&south) && grid.data[south.x as usize][south.y as usize] == '.' {
            queue.push_back(south);
        }

        let west = cur_pos + WEST;
        if grid.is_inside(&west) && grid.data[west.x as usize][west.y as usize] == '.' {
            queue.push_back(west);
        }
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let dig_plans = parse_input(input);

    // let mut points: HashSet<Point> = HashSet::new();
    let mut points = vec![];

    let mut current = Point::origin();
    points.push(current);

    for dig_plan in dig_plans {
        for _ in 0..dig_plan.count {
            current = current + dig_plan.direction;
            points.push(current);
        }
    }
    // dbg!(&points);

    let min_x = points.iter().map(|point| point.x).min().unwrap();
    let min_y = points.iter().map(|point| point.y).min().unwrap();

    // dbg!(&min_x);
    // dbg!(&min_y);
    // dbg!(&max_x);
    // dbg!(&max_y);

    if min_x < 0 {
        points.iter_mut().for_each(|point| point.x += min_x * -1);
    }
    if min_y < 0 {
        points.iter_mut().for_each(|point| point.y += min_y * -1);
    }

    let x_len: usize = points.iter().map(|point| point.x).max().unwrap() as usize + 1;
    let y_len: usize = points.iter().map(|point| point.y).max().unwrap() as usize + 1;

    let mut vec: Vec<Vec<char>> = vec![vec!['.'; y_len]; x_len];

    points.iter().for_each(|point| {
        // dbg!(point);
        vec[point.x as usize][point.y as usize] = '#'
    });

    let mut grid = Grid {
        data: vec,
        x_len,
        y_len,
    };

    // print_2d_vec(&grid.data);

    // let mut count = 0;
    // for row in grid.data.iter() {
    //     for cell in row {
    //         if *cell == '#' {
    //             count += 1
    //         }
    //     }
    // }

    // assert_eq!(count, 38);

    let mut border_cells = vec![];

    dbg!(&x_len);
    dbg!(&y_len);

    for x in 0..x_len {
        for y in 0..y_len {
            if (x == 0 || y == 0 || x == x_len - 1 || y == y_len - 1) && grid.data[x][y] == '.' {
                border_cells.push(Point {
                    x: x as isize,
                    y: y as isize,
                });
            }
        }
    }

    println!("Found border cells");
    // dbg!(&border_cells);

    for border_cell in border_cells {
        floodfill(&mut grid, border_cell);
        // print_2d_vec(&grid.data);
    }

    print_2d_vec(&grid.data);

    let mut count = 0;
    for row in grid.data.iter() {
        for cell in row {
            if *cell != 'X' {
                count += 1;
            }
        }
    }

    count
}

fn direction(dir: &str) -> Point {
    match dir {
        "U" => NORTH,
        "R" => EAST,
        "D" => SOUTH,
        "L" => WEST,
        _ => panic!("Invalid direction"),
    }
}

pub fn solve_part_2(_: &str) -> usize {
    todo!()
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
        assert_eq!(super::solve_part_2(&include_str!("day18/test_input")), 1);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day18/input")), 1);
    }
}
