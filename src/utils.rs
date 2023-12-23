pub mod geometry;
pub mod grid;
pub mod point;
pub mod point_v2;
pub mod snafu;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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

#[allow(dead_code)]
pub fn print_2d_vec(vec: &Vec<Vec<char>>) {
    let mut output = String::with_capacity(vec.len() * vec[0].len() + vec.len());
    for x in 0..vec.len() {
        for y in 0..vec[x].len() {
            output.push(vec[x][y]);
        }
        output.push('\n');
    }
    println!("{}", output);
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// a,b,c (0,0), (0,1), (0,2)
// d,e,f (1,0), (1,1), (1,2)
//
// becomes
//
// d,a (0,0), (0,1)
// e,b (1,0), (1,1)
// c,f (2,0), (2,1)
pub fn rotate(v: Vec<Vec<char>>) -> Option<Vec<Vec<char>>> {
    let x_len = v.len();
    let y_len = v[0].len();

    let mut result = vec![];

    for new_y in 0..y_len {
        let mut new_row: Vec<char> = vec![];
        for new_x in (0..x_len).rev() {
            new_row.push(v[new_x][new_y].clone());
        }
        result.push(new_row);
    }

    Some(result)
}

// a,b,c (0,0), (0,1), (0,2)
// d,e,f (1,0), (1,1), (1,2)
//
// becomes
//
// c,f (0,0), (0,1)
// b,e (1,0), (1,1)
// a,d (2,0), (2,1)
#[allow(dead_code)]
pub fn rotate_left(v: Vec<Vec<char>>) -> Option<Vec<Vec<char>>> {
    let x_len = v.len();
    let y_len = v[0].len();

    let mut result = vec![];

    for new_y in (0..y_len).rev() {
        let mut new_row: Vec<char> = vec![];
        for new_x in 0..x_len {
            new_row.push(v[new_x][new_y].clone());
        }
        result.push(new_row);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::rotate;
    use super::rotate_left;

    #[test]
    fn test_rotate() {
        let vec = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        assert_eq!(rotate(vec), Some(vec![vec!['d', 'a'], vec!['e', 'b'], vec!['f', 'c']]));
    }

    #[test]
    fn test_rotate_left() {
        let vec = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        assert_eq!(
            rotate_left(vec),
            Some(vec![vec!['c', 'f'], vec!['b', 'e'], vec!['a', 'd']])
        );
    }
}
