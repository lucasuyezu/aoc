pub mod point;
pub mod snafu;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseInputError;

pub fn get_arg(name: &str) -> Option<String> {
    let args: Vec<String> = std::env::args().collect();

    match args.iter().position(|arg| arg == name) {
        Some(arg_index) => Some(args[arg_index + 1].clone()),
        None => None,
    }
}

pub fn get_timer_millis() -> Option<u64> {
    let fps = get_arg("fps")?.parse::<u64>().unwrap();
    Some((1.0 / fps as f32 * 1_000.0) as u64)
}

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

pub fn lcm(arr: &[usize]) -> usize {
    arr.iter().fold(arr[0], |acc, num| (acc * num) / (gcd(*num, acc)))
}

pub fn print_2d_vec(vec: &Vec<Vec<char>>) {
    for x in 0..vec.len() {
        for y in 0..vec[x].len() {
            print!("{}", vec[x][y]);
        }
        println!();
    }
    println!();
}
