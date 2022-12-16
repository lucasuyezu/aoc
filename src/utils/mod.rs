pub mod point;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn get_lines(filename: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();

    if let Ok(file_lines) = read_lines(&filename) {
        for file_line in file_lines {
            if let Ok(line_string) = file_line {
                lines.push(line_string);
            }
        }
    } else {
        println!("Error reading file {}", filename);
    }

    lines
}

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
