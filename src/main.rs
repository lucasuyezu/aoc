#[macro_use]
extern crate lazy_static;

use std::sync::Arc;
use std::thread;
use std::time::Instant;

mod year2020;

macro_rules! solve {
    ($year:ident, $day:ident) => {
        let mut now = Instant::now();
        let lines = $year::get_lines(
            format!("/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/{}/{}/input", stringify!($year), stringify!($day))
        );
        let read_file_duration = now.elapsed();

        now = Instant::now();
        let parsed_struct = Arc::new($year::$day::get_structs(lines));
        let parse_duration = now.elapsed();

        let parsed_struct_t2 = parsed_struct.clone();
        let t2 = thread::spawn(move || {
            let t2_instant = Instant::now();
            let result = $year::$day::solve_part_2(&parsed_struct_t2);
            let solve_duration = t2_instant.elapsed();

            println!(
                "{} {} part2 read_file_duration={:?}\tparse_duration={:?}\tsolve_duration={:?}\tresult={}",
                stringify!($year),
                stringify!($day),
                read_file_duration,
                parse_duration,
                solve_duration,
                result
            );
        });

        let parsed_struct_t1 = parsed_struct.clone();
        let t1 = thread::spawn(move || {
            let t1_instant = Instant::now();
            let result = $year::$day::solve_part_1(&parsed_struct_t1);
            let solve_duration = t1_instant.elapsed();

            println!(
                "{} {} part1 read_file_duration={:?}\tparse_duration={:?}\tsolve_duration={:?}\tresult={}",
                stringify!($year),
                stringify!($day),
                read_file_duration,
                parse_duration,
                solve_duration,
                result,
            );
        });

        t2.join().expect("$year $day part2 has panicked!");
        t1.join().expect("$year $day part1 has panicked!");
    }
}

fn main() {
    let threads = vec![
        thread::spawn(|| { solve!(year2020, day4); }),
        thread::spawn(|| { solve!(year2020, day3); }),
        thread::spawn(|| { solve!(year2020, day2); }),
        thread::spawn(|| { solve!(year2020, day1); }),
    ];

    threads.into_iter().for_each(|thread| thread.join().unwrap());
}
