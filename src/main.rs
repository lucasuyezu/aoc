#[macro_use]
extern crate lazy_static;

use crate::utils::get_arg;
use std::fs;
use std::time::{Duration, Instant};

mod utils;
mod year2015;
mod year2020;
mod year2021;
mod year2022;
mod year2023;

macro_rules! solve_str {
    ($year:ident, $day:ident) => {
        if get_arg("year").unwrap_or(stringify!($year).to_string()) == stringify!($year)
            && get_arg("day").unwrap_or(stringify!($day).to_string()) == stringify!($day)
        {
            let now = Instant::now();
            let lines = fs::read_to_string(format!(
                "/Users/lucas/src/github.com/lucasuyezu/aoc/src/{}/{}/input",
                stringify!($year),
                stringify!($day)
            ))
            .unwrap();
            let read_file_duration = now.elapsed();

            let t1_instant = Instant::now();
            let result_1 = $year::$day::solve_part_1(&lines);
            let solve_duration = t1_instant.elapsed();

            println!(
                "{} {} part1 read_file_duration={}\tsolve_duration={}\tresult={}",
                stringify!($year),
                stringify!($day),
                duration_with_colour(read_file_duration),
                duration_with_colour(solve_duration),
                result_1,
            );

            let t2_instant = Instant::now();
            let result_2 = $year::$day::solve_part_2(&lines);
            let solve_duration = t2_instant.elapsed();

            println!(
                "{} {} part2 read_file_duration={}\tsolve_duration={}\tresult={}",
                stringify!($year),
                stringify!($day),
                duration_with_colour(read_file_duration),
                duration_with_colour(solve_duration),
                result_2,
            );
        }
    };
}

fn duration_with_colour(duration: Duration) -> String {
    let color = if duration.as_millis() < 1 {
        92
    } else if duration.as_secs() < 1 {
        93
    } else {
        31
    };

    format!("{:<20}", format!("\x1b[{}m{:?}\x1b[0m", color, duration))
}

fn main() {
    solve_str!(year2023, day4);
    solve_str!(year2023, day3);
    solve_str!(year2023, day2);
    solve_str!(year2023, day1);
    solve_str!(year2022, day25);
    solve_str!(year2022, day24);
    solve_str!(year2022, day23);
    solve_str!(year2022, day22);
    solve_str!(year2022, day21);
    solve_str!(year2022, day20);
    solve_str!(year2022, day19);
    solve_str!(year2022, day18);
    solve_str!(year2022, day17);
    solve_str!(year2022, day16);
    solve_str!(year2022, day15);
    solve_str!(year2022, day14);
    solve_str!(year2022, day13);
    solve_str!(year2022, day12);
    solve_str!(year2022, day11);
    solve_str!(year2022, day10);
    solve_str!(year2022, day9);
    solve_str!(year2022, day8);
    solve_str!(year2022, day7);
    solve_str!(year2022, day6);
    solve_str!(year2022, day5);
    solve_str!(year2022, day4);
    solve_str!(year2022, day3);
    solve_str!(year2022, day2);
    solve_str!(year2022, day1);
    solve_str!(year2021, day2);
    solve_str!(year2021, day1);
    solve_str!(year2020, day9);
    solve_str!(year2020, day8);
    solve_str!(year2020, day7);
    solve_str!(year2020, day6);
    solve_str!(year2020, day5);
    solve_str!(year2020, day4);
    solve_str!(year2020, day3);
    solve_str!(year2020, day2);
    solve_str!(year2020, day1);
    solve_str!(year2015, day1);
}
