#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use std::thread;
use std::time::Instant;

mod year2020;

macro_rules! solve {
    ($year:ident, $day:ident) => {
        let now = Instant::now();
        let lines_arc = Arc::new($year::get_lines(format!(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/{}/{}/input",
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
                "{} {} part2 read_file_duration={:?}\tsolve_duration={:?}\tresult={}",
                stringify!($year),
                stringify!($day),
                read_file_duration,
                solve_duration,
                result
            );
        });

        let lines_arc_t1 = lines_arc.clone();
        let t1 = thread::spawn(move || {
            let t1_instant = Instant::now();
            let result = $year::$day::solve_part_1(&lines_arc_t1);
            let solve_duration = t1_instant.elapsed();

            println!(
                "{} {} part1 read_file_duration={:?}\tsolve_duration={:?}\tresult={}",
                stringify!($year),
                stringify!($day),
                read_file_duration,
                solve_duration,
                result,
            );
        });

        t2.join().expect("$year $day part2 has panicked!");
        t1.join().expect("$year $day part1 has panicked!");
    };
}

fn main() {
    let threads = vec![
        thread::spawn(|| {
            solve!(year2020, day7);
        }),
        thread::spawn(|| {
            solve!(year2020, day6);
        }),
        thread::spawn(|| {
            solve!(year2020, day5);
        }),
        thread::spawn(|| {
            solve!(year2020, day4);
        }),
        thread::spawn(|| {
            solve!(year2020, day3);
        }),
        thread::spawn(|| {
            solve!(year2020, day2);
        }),
        thread::spawn(|| {
            solve!(year2020, day1);
        }),
    ];

    threads
        .into_iter()
        .for_each(|thread| thread.join().unwrap());
}
