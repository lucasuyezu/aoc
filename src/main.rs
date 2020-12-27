use std::thread;
use std::sync::Arc;

mod year2020;

fn main() {
    let t2020_02 = thread::spawn(|| {
        let lines = year2020::get_lines("/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day2/input");
        let structs = Arc::new(year2020::day2::get_structs(lines.iter()));

        let structs_t2 = structs.clone();
        let t2 = thread::spawn(move || {
            let result = year2020::day2::solve_part_2(
                structs_t2.as_slice()
            );
            println!("Year 2020 Day 2 Part 2 result is {}", result);
        });

        let structs_t1 = structs.clone();
        let t1 = thread::spawn(move || {
            let result = year2020::day2::solve_part_1(
                structs_t1.as_slice()
            );
            println!("Year 2020 Day 2 Part 1 result is {}", result);
        });

        t2.join().expect("Year 2020 Day 2 Part 2 has panicked!");
        t1.join().expect("Year 2020 Day 2 Part 1 has panicked!");
    });

    let t2020_01 = thread::spawn(|| {
        let lines = year2020::get_lines("/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day1/input");
        let structs = Arc::new(year2020::day1::get_structs(lines.iter()));

        let structs_t2 = structs.clone();
        let t2 = thread::spawn(move || {
            let result = year2020::day1::solve_part_2(
                structs_t2.as_slice()
            );
            println!("Year 2020 Day 1 Part 2 result is {}", result);
        });

        let structs_t1 = structs.clone();
        let t1 = thread::spawn(move || {
            let result = year2020::day1::solve_part_1(
                structs_t1.as_slice()
            );
            println!("Year 2020 Day 1 Part 1 result is {}", result);
        });

        t2.join().expect("Year 2020 Day 1 Part 2 has panicked!");
        t1.join().expect("Year 2020 Day 1 Part 1 has panicked!");
    });

    t2020_02.join().expect("Year 2020 Day 2 thread has panicked!");
    t2020_01.join().expect("Year 2020 Day 1 thread has panicked!");
}
