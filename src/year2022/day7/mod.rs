use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Tree {
    current_path: String,
    directories: HashMap<String, Directory>,
}

#[derive(Debug)]
struct Directory {
    size: usize,
    parent_dir_path: String,
}

impl Tree {
    fn new() -> Tree {
        let mut tree = Tree {
            directories: HashMap::new(),
            current_path: "/".to_string(),
        };

        tree.directories.insert(
            "/".to_string(),
            Directory {
                size: 0,
                parent_dir_path: String::new(),
            },
        );
        return tree;
    }

    fn cd_up(&mut self) {
        self.current_path = self
            .directories
            .get(&self.current_path)
            .unwrap()
            .parent_dir_path
            .clone();
    }

    fn cd_into(&mut self, directory_name: &str) {
        if directory_name == "/" {
            self.current_path = "/".to_string();
        } else {
            self.current_path = format!("{}{}/", self.current_path, directory_name);
        }
    }

    fn create_dir(&mut self, directory_name: &str) {
        let full_path = format!("{}{}/", self.current_path, directory_name);
        self.directories.insert(
            full_path.to_string(),
            Directory {
                size: 0,
                parent_dir_path: self.current_path.clone(),
            },
        );
    }

    fn add_file(&mut self, size: usize) {
        let mut cwd = self.current_path.clone();

        while cwd != "" {
            let dir = self.directories.get_mut(&cwd).unwrap();

            dir.size += size;
            cwd = dir.parent_dir_path.clone();
        }
    }
}

fn build_tree(input: &str) -> Tree {
    let mut tree = Tree::new();
    let file_regex = Regex::new(r"^(\d+)\s(.+)$").unwrap();

    input.lines().for_each(|line| {
        if line == "$ cd .." {
            tree.cd_up();
        } else if line.starts_with("$ cd") {
            tree.cd_into(line.split(" ").nth(2).unwrap());
        } else if line.starts_with("dir ") {
            tree.create_dir(line.split(" ").nth(1).unwrap());
        } else if file_regex.is_match(line) {
            let captures = file_regex.captures_iter(line).next().unwrap();
            tree.add_file(captures[1].parse().unwrap());
        }
    });

    return tree;
}
pub fn solve_part_1(input: &str) -> usize {
    let tree = build_tree(input);

    return tree
        .directories
        .values()
        .filter(|dir| dir.size <= 100_000)
        .map(|dir| dir.size)
        .sum();
}

pub fn solve_part_2(input: &str) -> usize {
    let tree = build_tree(input);
    let root_dir_size = tree.directories.get("/").unwrap().size;

    return tree
        .directories
        .iter()
        .filter_map(|(_, dir)| {
            if dir.size + 70_000_000 - root_dir_size >= 30_000_000 {
                Some(dir)
            } else {
                None
            }
        })
        .map(|dir| dir.size)
        .min()
        .unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 95_437);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 1_206_825);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 24_933_642);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 9_608_311);
    }
}
