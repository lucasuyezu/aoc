use std::collections::HashSet;

#[derive(Clone)]
struct Position {
    x: isize,
    y: isize,
}

struct Rope {
    knots: Vec<Position>,
    tail_positions_visited: HashSet<String>,
}

impl Rope {
    fn new(size: usize) -> Rope {
        let mut rope = Rope {
            knots: (0..size).map(|_| Position { x: 0, y: 0 }).collect(),
            tail_positions_visited: HashSet::new(),
        };

        rope.tail_positions_visited.insert("[0][0]".to_string());
        return rope;
    }

    fn execute_moves(&mut self, input: &str) {
        for line in input.lines() {
            let (direction, steps) = line.split_once(" ").unwrap();
            self.execute_move(direction, steps.parse::<usize>().unwrap());
        }
    }

    fn execute_move(&mut self, direction: &str, steps: usize) {
        for _ in 0..steps {
            match direction {
                "R" => self.knots[0].x += 1,
                "U" => self.knots[0].y += 1,
                "D" => self.knots[0].y -= 1,
                "L" => self.knots[0].x -= 1,
                _ => {
                    panic!("\tInvalid step direction! {}", direction);
                }
            }

            for i in 1..self.knots.len() {
                self.follow_knot(i);
            }

            let last_knot = self.knots.last().unwrap();
            self.tail_positions_visited
                .insert(format!("[{}][{}]", last_knot.x, last_knot.y));
        }
    }

    fn follow_knot(&mut self, knot_index: usize) {
        let head = self.knots[knot_index.checked_sub(1).unwrap()].clone();
        let tail = self.knots.get_mut(knot_index).unwrap();

        let x_distance = head.x - tail.x;
        let y_distance = head.y - tail.y;

        if x_distance.abs() < 2 && y_distance.abs() < 2 {
            return;
        }

        if x_distance == 0 {
            tail.y += if y_distance < 0 { -1 } else { 1 };
        } else if y_distance == 0 {
            tail.x += if x_distance < 0 { -1 } else { 1 };
        } else {
            tail.x += if x_distance < 0 { -1 } else { 1 };
            tail.y += if y_distance < 0 { -1 } else { 1 };
        }

        assert!((head.x - tail.x).abs() < 2);
        assert!((head.y - tail.y).abs() < 2);
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let mut rope = Rope::new(2);
    rope.execute_moves(input);

    return rope.tail_positions_visited.len();
}

pub fn solve_part_2(input: &str) -> usize {
    let mut rope = Rope::new(10);
    rope.execute_moves(input);

    return rope.tail_positions_visited.len();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 13);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 6_367);
    }

    #[test]
    fn part2_test_input_1() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 1);
    }

    #[test]
    fn part2_test_input_2() {
        assert_eq!(super::solve_part_2(&include_str!("test_input_2")), 36);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 2_536);
    }
}
