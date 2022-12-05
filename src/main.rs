#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

mod utils;
mod year2020;
mod year2022;

macro_rules! solve {
    ($year:ident, $day:ident) => {
        let now = Instant::now();
        let lines_arc = Arc::new(utils::get_lines(format!(
            "/Users/lucas/src/github.com/lucasuyezu/aoc/src/{}/{}/input",
            stringify!($year),
            stringify!($day)
        )));
        let read_file_duration = now.elapsed();

        let lines_arc_t2 = lines_arc.clone();
        let t2 = thread::spawn(move || {
            let t2_instant = Instant::now();
            let result = $year::$day::solve_part_2(&lines_arc_t2);
            let solve_duration = t2_instant.elapsed();

            println!(
                "{} {} part2 read_file_duration={}\tsolve_duration={}\tresult={}",
                stringify!($year),
                stringify!($day),
                duration_with_colour(read_file_duration),
                duration_with_colour(solve_duration),
                result
            );
        });

        let lines_arc_t1 = lines_arc.clone();
        let t1 = thread::spawn(move || {
            let t1_instant = Instant::now();
            let result = $year::$day::solve_part_1(&lines_arc_t1);
            let solve_duration = t1_instant.elapsed();

            println!(
                "{} {} part1 read_file_duration={}\tsolve_duration={}\tresult={}",
                stringify!($year),
                stringify!($day),
                duration_with_colour(read_file_duration),
                duration_with_colour(solve_duration),
                result,
            );
        });

        t2.join().expect("$year $day part2 has panicked!");
        t1.join().expect("$year $day part1 has panicked!");
    };
}

fn duration_with_colour(duration: Duration) -> String {
    let color = if duration.as_micros() < 10 {
        92
    } else if duration.as_millis() < 1 {
        93
    } else {
        31
    };

    format!("{:<20}", format!("\x1b[{}m{:?}\x1b[0m", color, duration))
}

fn main() {
    solve!(year2022, day5);
    solve!(year2022, day4);
    solve!(year2022, day3);
    solve!(year2022, day2);
    solve!(year2022, day1);
    solve!(year2020, day9);
    solve!(year2020, day8);
    solve!(year2020, day7);
    solve!(year2020, day6);
    solve!(year2020, day5);
    solve!(year2020, day4);
    solve!(year2020, day3);
    solve!(year2020, day2);
    solve!(year2020, day1);
}
