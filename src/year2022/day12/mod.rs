use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
struct Vertex {
    c: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Graph {
    vertices: Vec<Vec<Vertex>>,
}

impl Graph {
    fn from(input: &str) -> Result<(Graph, Vertex, Vertex), String> {
        let mut start_vertex: Option<Vertex> = None;
        let mut end_vertex: Option<Vertex> = None;

        let vertices: Vec<Vec<Vertex>> = input
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.chars()
                    .enumerate()
                    .map(|(y, c)| {
                        let vertex = Vertex { c, x, y };

                        if c == 'S' {
                            start_vertex = Some(vertex.clone());
                        }
                        if c == 'E' {
                            end_vertex = Some(vertex.clone());
                        }

                        vertex
                    })
                    .collect::<Vec<Vertex>>()
            })
            .collect();

        if start_vertex.is_none() {
            return Err(String::from("Did not find start vertex"));
        }

        if end_vertex.is_none() {
            return Err(String::from("Did not find end vertex"));
        }

        let graph = Graph { vertices };
        return Ok((graph, start_vertex.unwrap(), end_vertex.unwrap()));
    }

    fn get_char_value(c: char) -> isize {
        if c == 'S' {
            'a' as isize
        } else if c == 'E' {
            'z' as isize
        } else {
            c as isize
        }
    }

    fn solve(&self, start_vertex: &Vertex, end_vertex: &Vertex) -> Option<usize> {
        let mut visited: HashSet<&Vertex> = HashSet::new();
        let mut queue: VecDeque<(&Vertex, usize)> = VecDeque::new();

        queue.push_back((start_vertex, 0));
        visited.insert(&start_vertex);

        while let Some((current_vertex, current_distance)) = queue.pop_front() {
            if *current_vertex == *end_vertex {
                return Some(current_distance);
            }

            for neighbour in self.neighbours(&current_vertex, &visited) {
                if Self::get_char_value(neighbour.c) - Self::get_char_value(current_vertex.c) <= 1 {
                    queue.push_back((neighbour, current_distance + 1));
                    visited.insert(&neighbour);
                }
            }
        }

        None
    }

    fn neighbours(&self, current_vertex: &Vertex, visited: &HashSet<&Vertex>) -> Vec<&Vertex> {
        let mut result = vec![];

        if current_vertex.x + 1 < self.vertices.len() {
            let vertex = &self.vertices[current_vertex.x + 1][current_vertex.y];
            if !visited.contains(&vertex) {
                result.push(vertex);
            }
        }

        if current_vertex.x > 0 {
            let vertex = &self.vertices[current_vertex.x - 1][current_vertex.y];
            if !visited.contains(&vertex) {
                result.push(vertex);
            }
        }

        if current_vertex.y + 1 < self.vertices[0].len() {
            let vertex = &self.vertices[current_vertex.x][current_vertex.y + 1];
            if !visited.contains(&vertex) {
                result.push(vertex);
            }
        }
        // up
        if current_vertex.y > 0 {
            let vertex = &self.vertices[current_vertex.x][current_vertex.y - 1];
            if !visited.contains(&vertex) {
                result.push(vertex);
            }
        }

        result
    }

    fn get_vertices_for_char(&self, c: &char) -> Vec<Vertex> {
        self.vertices
            .clone()
            .into_iter()
            .map(|vec| vec.clone().into_iter().filter(|vertex| vertex.c == *c))
            .flatten()
            .collect()
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let (graph, start_vertex, end_vertex) = Graph::from(input).unwrap();
    return graph.solve(&start_vertex, &end_vertex).unwrap();
}

pub fn solve_part_2(input: &str) -> usize {
    let (graph, start_vertex, end_vertex) = Graph::from(input).unwrap();

    let mut starting_positions: Vec<Vertex> = graph.get_vertices_for_char(&'a');

    starting_positions.push(start_vertex);

    let mut results: Vec<usize> = starting_positions
        .iter()
        .map(|starting_position| {
            println!("Solving for {:?}", starting_position);

            if let Some(result) = graph.solve(&starting_position, &end_vertex) {
                println!("\tResult is {}", result);
                result
            } else {
                usize::max_value()
            }
        })
        .collect();

    results.sort();

    return results[0];
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1_test_input() {
        assert_eq!(super::solve_part_1(&include_str!("test_input")), 31);
    }

    #[test]
    fn part1_real_input() {
        assert_eq!(super::solve_part_1(&include_str!("input")), 462);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(super::solve_part_2(&include_str!("test_input")), 29);
    }

    #[test]
    fn part2_real_input() {
        assert_eq!(super::solve_part_2(&include_str!("input")), 451);
    }
}
