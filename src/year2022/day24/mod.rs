use std::collections::{HashSet, VecDeque};

use crate::utils::point::*;

#[derive(PartialEq, Eq, Hash, Clone)]
struct Blizzard {
    pos: Point,
    direction: char,
}

#[derive(Debug, Clone)]
struct Grid {
    max_x: isize,
    max_y: isize,
}

fn parse_input(s: &str) -> (Grid, Vec<Blizzard>, Point, Point) {
    let mut blizzards = vec![];
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    let mut max_x = 0 as isize;
    let mut max_y = 0 as isize;

    for (x, line) in s.lines().enumerate() {
        max_x = max_x.max(x as isize);
        for (y, c) in line.chars().enumerate() {
            max_y = max_y.max(y as isize);

            let pos = Point {
                x: x as isize,
                y: y as isize,
            };

            if x == 0 && c == '.' {
                start.y = y as isize;
            }

            if c != '.' && c != '#' {
                blizzards.push(Blizzard { pos, direction: c });
            }
        }
    }

    end.x = max_x;
    end.y = s.lines().last().unwrap().chars().position(|c| c == '.').unwrap() as isize;

    (Grid { max_x, max_y }, blizzards, start, end)
}

fn print_frame(grid: &Grid, start: &Point, end: &Point, blizzards: &Vec<Blizzard>, pos: &Point) {
    let mut frame = String::new();
    for x in 0..=grid.max_x {
        for y in 0..=grid.max_y {
            let point = Point { x, y };

            if *pos == point {
                frame.push('E');
            } else if point == *start {
                frame.push('S');
            } else if point == *end {
                frame.push('G');
            } else if x == 0 || x == grid.max_x || y == 0 || y == grid.max_y {
                frame.push('#');
            } else {
                let pos_blizzards: Vec<&Blizzard> = blizzards.iter().filter(|blizzard| blizzard.pos == point).collect();

                if pos_blizzards.len() > 1 {
                    frame.push_str(pos_blizzards.len().to_string().as_str());
                } else if pos_blizzards.len() == 1 {
                    frame.push(pos_blizzards[0].direction);
                } else {
                    frame.push('.');
                }
            }
        }
        frame.push('\n');
    }

    print!("{}", frame);
}

fn bfs(grid: &Grid, start: &Point, end: &Point, blizzards: &Vec<Blizzard>) -> (usize, Vec<Blizzard>) {
    let mut visited: HashSet<(Point, usize)> = HashSet::new();
    let mut queue: VecDeque<(Point, Vec<Blizzard>, usize)> = VecDeque::new();

    queue.push_back((*start, blizzards.clone(), 0));
    visited.insert((*start, 0));

    while let Some((current_pos, current_blizzards, current_time)) = queue.pop_front() {
        if current_pos == *end {
            return (current_time, current_blizzards);
        }

        let (neighbours, moved_blizzards) = neighbours(&grid, start, end, &current_pos, &current_blizzards);
        for neighbour in neighbours {
            if !visited.contains(&(neighbour, current_time + 1)) {
                queue.push_back((neighbour, moved_blizzards.clone(), current_time + 1));
                visited.insert((neighbour, current_time + 1));
            }
        }
    }

    panic!("Didn't find a way from {:?} to {:?}", start, end);
}

fn neighbours(
    grid: &Grid,
    start: &Point,
    end: &Point,
    current_pos: &Point,
    blizzards: &[Blizzard],
) -> (Vec<Point>, Vec<Blizzard>) {
    let moved_blizzards = move_blizzards(grid, blizzards);

    // Calculate all positions I can move into.
    let points: Vec<Point> = vec![
        *current_pos + NORTH,
        *current_pos + WEST,
        *current_pos + EAST,
        *current_pos + SOUTH,
        *current_pos,
    ];

    // Reject neighbours who are out of bounds
    let mut neighbours: Vec<Point> = points
        .into_iter()
        .filter(|point| {
            point == start
                || point == end
                || ((point.x > 0 && point.x <= grid.max_x - 1) && (point.y > 0 && point.y <= grid.max_y - 1))
        })
        .collect();

    // Reject neighbours who will have a blizzard in the next minute
    neighbours = neighbours
        .into_iter()
        .filter(|point| {
            moved_blizzards
                .iter()
                .all(|moved_blizzard| moved_blizzard.pos != *point)
        })
        .collect();

    (neighbours, moved_blizzards)
}

fn move_blizzards(grid: &Grid, blizzards: &[Blizzard]) -> Vec<Blizzard> {
    blizzards
        .iter()
        .map(|blizzard| {
            let mut new_pos = match blizzard.direction {
                '>' => blizzard.pos + EAST,
                'v' => blizzard.pos + SOUTH,
                '<' => blizzard.pos + WEST,
                '^' => blizzard.pos + NORTH,
                d @ _ => panic!("Unsupported cell value {}.", d),
            };

            if new_pos.x <= 0 {
                new_pos.x = grid.max_x - 1;
            }

            if new_pos.x >= grid.max_x {
                new_pos.x = 1;
            }

            if new_pos.y <= 0 {
                new_pos.y = grid.max_y - 1;
            }

            if new_pos.y >= grid.max_y {
                new_pos.y = 1;
            }

            Blizzard {
                pos: new_pos,
                direction: blizzard.direction,
            }
        })
        .collect()
}

pub fn solve_part_1(input: &str) -> usize {
    let (grid, blizzards, start, end) = parse_input(input);
    let (time, _) = bfs(&grid, &start, &end, &blizzards);

    time
}

pub fn solve_part_2(input: &str) -> usize {
    let (grid, blizzards, start, end) = parse_input(input);
    let (time_1, blizzards) = bfs(&grid, &start, &end, &blizzards);
    let (time_2, blizzards) = bfs(&grid, &end, &start, &blizzards);
    let (time_3, _) = bfs(&grid, &start, &end, &blizzards);

    time_1 + time_2 + time_3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(solve_part_1(&include_str!("test_input")), 18);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(solve_part_1(&include_str!("input")), 232);
    }

    #[test]
    fn part2_test_input() {
        let (grid, blizzards, start, end) = parse_input(&include_str!("test_input"));
        let (time, blizzards) = bfs(&grid, &start, &end, &blizzards);
        assert_eq!(time, 18);

        let (time, blizzards) = bfs(&grid, &end, &start, &blizzards);
        assert_eq!(time, 23);

        let (time, _) = bfs(&grid, &start, &end, &blizzards);
        assert_eq!(time, 13);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(solve_part_2(&include_str!("input")), 715);
    }
}
