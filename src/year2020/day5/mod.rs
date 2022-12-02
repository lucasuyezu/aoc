use std::collections::HashSet;

#[derive(Debug)]
struct BoardingPass<'a> {
    string: &'a str,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Seat {
    row_id: usize,
    column_id: usize,
}

impl Seat {
    fn seat_id(&self) -> usize {
        self.row_id * 8 + self.column_id
    }
}

impl BoardingPass<'_> {
    fn row_id(&self) -> usize {
        self.find_id(0, 127, &self.string[0..7])
    }

    fn column_id(&self) -> usize {
        self.find_id(0, 7, &self.string[7..10])
    }

    fn find_id(&self, mut range_low: usize, mut range_high: usize, substr: &str) -> usize {
        let mut last = false;

        for current_char in substr.chars() {
            // println!(
            //     "range_low={} range_high={} current_char={}",
            //     range_low, range_high, current_char
            // );
            if current_char == 'F' || current_char == 'L' {
                range_high = range_low + (range_high - range_low) / 2;
                last = true;
            } else {
                range_low += (range_high - range_low) / 2 + 1;
                last = false;
            }
        }

        if last {
            range_low
        } else {
            range_high
        }
    }
}

pub fn solve_part_1(lines: &[String]) -> usize {
    let mut highest_seat_id = 0;

    for line in lines {
        let boarding_pass = BoardingPass { string: line };
        let seat = Seat {
            row_id: boarding_pass.row_id(),
            column_id: boarding_pass.column_id(),
        };

        let current_seat_id = seat.seat_id();

        if highest_seat_id < current_seat_id {
            highest_seat_id = current_seat_id;
        }
    }

    highest_seat_id
}

pub fn solve_part_2(lines: &[String]) -> usize {
    // create set to store all seats, based on row_id + column_id
    let mut seats = HashSet::new();
    let mut current_seat: Seat;

    for line in lines {
        let boarding_pass = BoardingPass { string: line };
        let seat = Seat {
            row_id: boarding_pass.row_id(),
            column_id: boarding_pass.column_id(),
        };

        seats.insert(seat);
    }

    for row_id in 9..109 {
        for column_id in 0..7 {
            if (row_id == 9 && column_id < 3) || (row_id == 108 && column_id > 0) {
                continue;
            }

            // println!("Testing row_id {} column_id {}", row_id, column_id);
            current_seat = Seat { row_id, column_id };
            if !seats.contains(&current_seat) {
                // println!(
                //     "Missing seat {:?} with seat id {}",
                //     current_seat,
                //     current_seat.seat_id()
                // );
                return current_seat.seat_id();
            }
        }
    }

    panic!("This should have not happened!");
}

#[cfg(test)]
mod tests {
    use super::super::super::utils;

    #[test]
    fn part1_test_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day5/test_input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 820);
    }

    #[test]
    fn part1_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day5/input"
                .to_string(),
        );
        assert_eq!(super::solve_part_1(&lines), 864);
    }

    #[test]
    fn part2_real_input() {
        let lines = utils::get_lines(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/year2020/day5/input"
                .to_string(),
        );
        let result = super::solve_part_2(&lines);
        assert_ne!(result, 0);
        assert_ne!(result, 8);
    }
}
