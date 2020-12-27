use std::sync::Arc;
use std::thread;

mod year2020;

fn main() {
    let t2020_03 = thread::spawn(|| {
        let lines = year2020::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day3/input",
        );
        let slope_map = Arc::new(year2020::day3::get_structs(lines));

        let slope_map_t2 = slope_map.clone();
        let t2 = thread::spawn(move || {
            let result_1 = year2020::day3::solve(&slope_map_t2, 1, 1);
            let result_2 = year2020::day3::solve(&slope_map_t2, 3, 1);
            let result_3 = year2020::day3::solve(&slope_map_t2, 5, 1);
            let result_4 = year2020::day3::solve(&slope_map_t2, 7, 1);
            let result_5 = year2020::day3::solve(&slope_map_t2, 1, 2);

            println!(
                "Year 2020 Day 3 Part 2 result is {}",
                result_1 * result_2 * result_3 * result_4 * result_5
            );
        });

        let slope_map_t1 = slope_map.clone();
        let t1 = thread::spawn(move || {
            let result = year2020::day3::solve(&slope_map_t1, 3, 1);
            println!("Year 2020 Day 3 Part 1 result is {}", result);
        });

        t2.join().expect("Year 2020 Day 3 Part 2 has panicked!");
        t1.join().expect("Year 2020 Day 3 Part 1 has panicked!");
    });

    let t2020_02 = thread::spawn(|| {
        let lines = year2020::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day2/input",
        );
        let structs = Arc::new(year2020::day2::get_structs(lines.iter()));

        let structs_t2 = structs.clone();
        let t2 = thread::spawn(move || {
            let result = year2020::day2::solve_part_2(structs_t2.as_slice());
            println!("Year 2020 Day 2 Part 2 result is {}", result);
        });

        let structs_t1 = structs.clone();
        let t1 = thread::spawn(move || {
            let result = year2020::day2::solve_part_1(structs_t1.as_slice());
            println!("Year 2020 Day 2 Part 1 result is {}", result);
        });

        t2.join().expect("Year 2020 Day 2 Part 2 has panicked!");
        t1.join().expect("Year 2020 Day 2 Part 1 has panicked!");
    });

    let t2020_01 = thread::spawn(|| {
        let lines = year2020::get_lines(
            "/Users/lucasuyezushopify/src/github.com/lucasuyezu/aoc/src/year2020/day1/input",
        );
        let structs = Arc::new(year2020::day1::get_structs(lines.iter()));

        let structs_t2 = structs.clone();
        let t2 = thread::spawn(move || {
            let result = year2020::day1::solve_part_2(structs_t2.as_slice());
            println!("Year 2020 Day 1 Part 2 result is {}", result);
        });

        let structs_t1 = structs.clone();
        let t1 = thread::spawn(move || {
            let result = year2020::day1::solve_part_1(structs_t1.as_slice());
            println!("Year 2020 Day 1 Part 1 result is {}", result);
        });

        t2.join().expect("Year 2020 Day 1 Part 2 has panicked!");
        t1.join().expect("Year 2020 Day 1 Part 1 has panicked!");
    });

    t2020_03
        .join()
        .expect("Year 2020 Day 3 thread has panicked!");
    t2020_02
        .join()
        .expect("Year 2020 Day 2 thread has panicked!");
    t2020_01
        .join()
        .expect("Year 2020 Day 1 thread has panicked!");
}
