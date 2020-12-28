use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod day1;
pub mod day2;
pub mod day3;

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

    return lines;
}
