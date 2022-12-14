use NestedList::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum NestedList {
    Number(u32),
    List(Vec<NestedList>),
}

impl PartialOrd for NestedList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Number(a), Number(b)) => a.partial_cmp(b),
            (List(a), Number(b)) => a.partial_cmp(&vec![List(vec![Number(*b)])]),
            (Number(a), List(b)) => vec![Number(*a)].partial_cmp(&b),
            (List(a), List(b)) => a.partial_cmp(&b),
        }
    }
}

impl Ord for NestedList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

fn parse_input(input: &str) -> Vec<NestedList> {
    let mut list_stack: Vec<NestedList> = vec![];
    let mut current_number = String::new();

    for c in input.chars() {
        match c {
            '[' => {
                let new_list = List(vec![]);
                list_stack.push(new_list);
            }
            '0'..='9' => {
                current_number.push(c);
            }
            ']' => {
                if !current_number.is_empty() {
                    if let Some(List(current_list)) = list_stack.last_mut() {
                        current_list.push(Number(current_number.parse().unwrap()));
                        current_number = String::new();
                    } else {
                        panic!();
                    }
                }

                let list = list_stack.pop().to_owned();
                if !list_stack.is_empty() {
                    if let Some(List(current_list)) = list_stack.last_mut() {
                        current_list.push(list.unwrap());
                    }
                } else if let Some(List(final_list)) = list {
                    return final_list;
                } else {
                    panic!();
                }
            }
            ',' => {
                if !current_number.is_empty() {
                    if let Some(List(current_list)) = list_stack.last_mut() {
                        current_list.push(Number(current_number.parse().unwrap()));
                        current_number = String::new();
                    } else {
                        panic!();
                    }
                }
            }
            ' ' => (),
            _ => panic!(),
        }
    }

    panic!();
}

pub fn solve_part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter_map(|(i, packets_str)| {
            let (left_packet_str, right_packet_str) = packets_str.split_once("\n").unwrap();

            let left_list = parse_input(left_packet_str.trim());
            let right_list = parse_input(right_packet_str.trim());

            if left_list < right_list {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut packets: Vec<Vec<NestedList>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| parse_input(line.trim()))
        .collect();

    let divider_packet_two = vec![List(vec![Number(2)])];
    let divider_packet_six = vec![List(vec![Number(6)])];

    packets.push(divider_packet_two.clone());
    packets.push(divider_packet_six.clone());
    packets.sort();

    let index_divider_packet_two = packets
        .iter()
        .position(|packet| *packet == divider_packet_two)
        .unwrap();
    let index_divider_packet_six = packets
        .iter()
        .position(|packet| *packet == divider_packet_six)
        .unwrap();

    (index_divider_packet_two + 1) * (index_divider_packet_six + 1)
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 13);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 6072);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 140);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 22184);
    }
}
