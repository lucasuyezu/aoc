use std::collections::HashMap;

const UNKNOWN: &str = "?";
const OPERATIONAL: &str = ".";
const DAMAGED: &str = "#";

fn parse_input(input: &str, n: usize) -> Vec<(String, Vec<usize>)> {
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
                chunks.extend(chunks_vec.iter());
            }

            (data_vec.join(UNKNOWN), chunks)
        })
        .collect()
}

fn count<'a>(data: &'a str, groups: &'a [usize], cache: &mut HashMap<(&'a str, &'a [usize]), usize>) -> usize {
    if let Some(cache_hit) = cache.get(&(data, groups)) {
        return *cache_hit;
    }

    if groups.is_empty() {
        // There are no more groups to check. This means that the string is valid
        // as long as there are no more '#' in the string.
        return if data.contains(DAMAGED) { 0 } else { 1 };
    }

    if data.is_empty() {
        // The string is empty. If there are groups left, this string is invalid.
        return if groups.is_empty() { 1 } else { 0 };
    }

    let mut result = 0;

    if data.starts_with(OPERATIONAL) || data.starts_with(UNKNOWN) {
        // Recurse assuming '?' is '.'
        result += count(&data[1..], groups, cache);
    }

    if data.starts_with(DAMAGED) || data.starts_with(UNKNOWN) {
        // If the first group is of size N and the string has less than N characters, this line is invalid.
        // Example: I'm expecting 3 damaged springs in a row, and the whole string is "##".
        if data.len() < groups[0] {
            cache.insert((data, groups), result);
            return result;
        }

        // If the first group is of size N and I have a '.' in the next N chars, this line is invalid.
        // Example: I'm expecting 3 damaged springs in a row, and the beginning of the string is "#.#", or "#?.".
        if data[..groups[0]].contains(OPERATIONAL) {
            cache.insert((data, groups), result);
            return result;
        }

        if data.len() == groups[0] {
            // The string has exactly N chars. Pop the first block and recurse with an empty string.
            result += count("", &groups[1..], cache);
        } else {
            // The string is longer than the group size.
            // If the first char after N is '#' the line is invalid.
            // Example: I'm expecting 3 damaged springs in a row. This means the 4th char MUST NOT be a '#'.
            if data[groups[0]..(groups[0] + 1)] == *DAMAGED {
                cache.insert((data, groups), result);
                return result;
            }

            // If I'm here it means I can eliminate this entire group, and recurse into the next.
            // Also recurse assuming '?' is '#', so we cover both possibilities.
            result += count(&data[(groups[0] + 1)..], &groups[1..], cache);
        }
    }

    cache.insert((data, groups), result);
    result
}

pub fn solve_part_1(input: &str) -> usize {
    let mut cache: HashMap<(&str, &[usize]), usize> = HashMap::new();

    parse_input(input, 1)
        .iter()
        .map(|(data, groups)| count(data.as_str(), &groups, &mut cache))
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut cache: HashMap<(&str, &[usize]), usize> = HashMap::new();

    parse_input(input, 5)
        .iter()
        .map(|(data, groups)| count(data.as_str(), &groups, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day12/test_input")), 21);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day12/input")), 7_307);
    }

    #[test]
    fn part2_parser() {
        assert_eq!(
            super::parse_input(&"???.### 1,1,3", 5),
            vec![(
                String::from("???.###????.###????.###????.###????.###"),
                vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
            )]
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day12/test_input")), 21);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day12/input")), 3_415_570_893_842);
    }
}
