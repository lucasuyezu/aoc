use rayon::prelude::*;

fn possible_strings(data: &str) -> Vec<String> {
    if !data.contains("?") {
        return vec![data.to_string()];
    }
    // dbg!(data);

    if data.is_empty() {
        // println!("Data is empty.");
        return vec![];
    }

    let (first_char, rest) = data.split_at(1);
    let strings = possible_strings(&rest.to_string());

    if strings.is_empty() {
        if first_char == "?" {
            return vec![String::from("."), String::from("#")];
        } else {
            return vec![first_char.to_string()];
        }
    }

    if first_char == "?" {
        strings
            .par_iter()
            .map(|string| {
                // dbg!(&string);
                vec![format!("#{}", string), format!(".{}", string)]
            })
            .flatten()
            .collect::<Vec<String>>()
    } else {
        // println!("First char is not ?");
        strings
            .par_iter()
            .map(|string| {
                // dbg!(&string);
                vec![format!("{}{}", first_char, string)]
            })
            .flatten()
            .collect::<Vec<String>>()
    }
}

fn parse_input(input: &str, n: u8) -> Vec<(String, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (data_str, rest) = line.split_once(" ").unwrap();

            let mut chunks: Vec<usize> = vec![];
            let mut data_vec = vec![];
            let chunks_vec: Vec<usize> = rest
                .split(",")
                .map(|chunk_str| chunk_str.parse::<usize>().unwrap())
                .collect();

            for _ in 0..n {
                data_vec.push(data_str);
                chunks.append(&mut chunks_vec.clone());
            }

            (data_vec.join("?"), chunks)
        })
        .collect()
}

fn solve(records: Vec<(String, Vec<usize>)>) -> usize {
    records
        .into_par_iter()
        .map(|(data, chunks)| {
            println!("Solving line {}", data);
            possible_strings(data.as_str())
                .into_par_iter()
                .filter(|possible_string| {
                    let string_chunks = possible_string
                        .split(".")
                        .filter(|chunk| !chunk.is_empty())
                        .map(|chunk| chunk.len())
                        .collect::<Vec<usize>>();

                    string_chunks == *chunks
                })
                .count()
        })
        .sum()
}

pub fn solve_part_1(input: &str) -> usize {
    let records = parse_input(input, 1);
    solve(records)
}

pub fn solve_part_2(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&"???.### 1,1,3"), 1);
        assert_eq!(super::solve_part_1(&".??..??...?##. 1,1,3"), 4);
        assert_eq!(super::solve_part_1(&"?###???????? 3,2,1"), 10);
        assert_eq!(super::solve_part_1(&include_str!("day12/test_input")), 21);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day12/input")), 7_307);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            super::parse_input(&"???.### 1,1,3", 5),
            vec![(
                String::from("???.###????.###????.###????.###????.###"),
                vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
            )]
        );
        assert_eq!(super::solve_part_2(&"???.### 1,1,3"), 1);
        assert_eq!(super::solve_part_2(&".??..??...?##. 1,1,3"), 16384);
        // assert_eq!(super::solve_part_1(&"?###???????? 3,2,1"), 10);
        // assert_eq!(super::solve_part_1(&include_str!("day12/test_input")), 21);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day12/input")), 34_655_848);
    }
}
