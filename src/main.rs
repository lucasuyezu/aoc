use std::thread;

mod year2020;

fn main() {
    let t2020_02 = thread::spawn(|| {
        let year2020_day2_lines = year2020::get_lines("/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day2/input");

        let result = year2020::day2::solve_part_2(
            year2020_day2_lines.iter()
        );
        println!("Year 2020 Day 2 Part 2 result is {}", result);

        let result = year2020::day2::solve_part_1(
            year2020_day2_lines.iter()
        );
        println!("Year 2020 Day 2 Part 1 result is {}", result);
    });

    let t2020_01 = thread::spawn(|| {
        let year2020_day1_lines = year2020::get_lines("/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day1/input");

        let result = year2020::day1::solve_part_2(
            year2020_day1_lines.iter()
        );
        println!("Year 2020 Day 1 Part 2 result is {}", result);

        let result = year2020::day1::solve_part_1(
            year2020_day1_lines.iter()
        );
        println!("Year 2020 Day 1 Part 1 result is {}", result);
    });

    t2020_02.join().expect("Year 2020 Day 2 thread has panicked!");
    t2020_01.join().expect("Year 2020 Day 1 thread has panicked!");
}
