type Box = Vec<(String, usize)>;

fn hash(s: &str) -> usize {
    s.chars().fold(0, |mut acc, c| {
        acc += c as usize;
        acc *= 17;
        acc.rem_euclid(256)
    })
}

fn populate_boxes(s: &str, boxes: &mut Vec<Box>) {
    let split_char = match s.contains("=") {
        true => '=',
        false => '-',
    };

    let (label, focal_length_str) = s.split_once(split_char).unwrap();
    let box_id = hash(label);

    if s.contains("=") {
        let focal_length: usize = focal_length_str.parse().unwrap();
        if let Some(pos) = boxes[box_id].iter().position(|(lens_label, _)| lens_label == label) {
            boxes[box_id].remove(pos);
            boxes[box_id].insert(pos, (label.to_string(), focal_length));
        } else {
            boxes[box_id].push((label.to_string(), focal_length));
        }
    } else {
        if let Some(pos) = boxes[box_id].iter().position(|(lens_label, _)| lens_label == label) {
            boxes[box_id].remove(pos);
        }
    }
}

fn focusing_power((box_id, cur_box): (usize, &Box)) -> usize {
    cur_box
        .iter()
        .enumerate()
        .map(|(slot_id, (_label, focal_length))| (1 + box_id) * (1 + slot_id) * focal_length)
        .sum::<usize>()
}

pub fn solve_part_1(input: &str) -> usize {
    input.trim().split(",").map(hash).sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut boxes: Vec<Box> = vec![vec![]; 256];

    input.trim().split(",").for_each(|s| {
        populate_boxes(s, &mut boxes);
    });

    boxes.iter().enumerate().map(focusing_power).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn hash() {
        assert_eq!(super::hash("HASH"), 52);
        assert_eq!(super::hash("rn=1"), 30);
        assert_eq!(super::hash("cm-"), 253);
    }

    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("day15/test_input")), 1_320);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("day15/input")), 512_283);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("day15/test_input")), 145);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("day15/input")), 215_827);
    }
}
