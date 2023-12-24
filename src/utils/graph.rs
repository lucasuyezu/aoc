use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

use itertools::Itertools;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Edge {
    from: String,
    to: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Graph {
    nodes: HashSet<String>,
    edges: HashSet<Edge>,
    distances: HashMap<Edge, usize>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashSet::new(),
            distances: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn insert_edge_compacting(&mut self, from: String, to: String, distance: usize) {
        match self.edges.clone().iter().find(|edge| edge.to == from) {
            None => {
                self.insert_edge_directed(from, to, distance);
            }
            Some(dead_edge) => {
                self.nodes.remove(&from);
                self.nodes.insert(to.clone());

                let new_edge = Edge {
                    from: dead_edge.from.clone(),
                    to: to.clone(),
                };

                self.edges.remove(&dead_edge);
                self.edges.insert(new_edge.clone());

                let old_distance = self.distances.remove(dead_edge).unwrap();
                self.distances.insert(new_edge, old_distance + distance);
            }
        }
    }

    // Insert a directed node:
    // A -> B
    //   2
    pub fn insert_edge_directed(&mut self, from: String, to: String, distance: usize) {
        self.nodes.insert(from.clone());
        self.nodes.insert(to.clone());

        if from == to {
            panic!();
        }

        let new_edge = Edge { from, to };

        self.edges.insert(new_edge.clone());

        self.distances.insert(new_edge, distance);
    }

    // Inserts an undirected node:
    // A --- B
    //    2
    #[allow(dead_code)]
    pub fn insert_edge_undirected(&mut self, from: String, to: String, distance: usize) {
        self.insert_edge_directed(from.clone(), to.clone(), distance);
        self.insert_edge_directed(to, from, distance);
    }

    // A graph with these edges and distances:
    // A -> B -> C -> D -> E
    //   2  | 3    5    7
    //      |
    //   11 |
    //      v
    //      F --> G --> H
    //        13    17
    //
    // Should be compacted to something like this:
    // A -> B --> E
    //   2  | 15
    //      |
    //   41 |
    //      |
    //      v
    //      H
    #[allow(dead_code)]
    pub fn compact(&mut self) {
        println!("Compacting graph...");
        let mut stop = false;
        while !stop {
            match self.removable_node() {
                None => {
                    stop = true;
                }
                Some((node, edges_to_me, edges_from_me)) => {
                    println!("Removing node {}.", node);

                    self.nodes.remove(&node);

                    if edges_to_me.len() == 1 {
                        self.merge_edges(edges_to_me.first().unwrap(), edges_from_me.first().unwrap());
                    } else {
                        let first_edge_to_me = edges_to_me.first().unwrap();
                        let first_edge_from_me = edges_from_me
                            .iter()
                            .find(|edge| edge.to != first_edge_to_me.from)
                            .unwrap();

                        // dbg!(&first_edge_to_me);
                        // dbg!(&first_edge_from_me);
                        self.merge_edges(first_edge_to_me, first_edge_from_me);

                        let second_edge_to_me = edges_to_me.last().unwrap();
                        let second_edge_from_me = edges_from_me
                            .iter()
                            .find(|edge| edge.to != second_edge_to_me.from)
                            .unwrap();

                        // dbg!(&second_edge_to_me);
                        // dbg!(&second_edge_from_me);
                        self.merge_edges(second_edge_to_me, second_edge_from_me);
                    }
                }
            }
        }
        println!("Graph is fully compacted.");
    }

    fn merge_edges(&mut self, edge_to_me: &Edge, edge_from_me: &Edge) {
        let new_edge = Edge {
            from: edge_to_me.from.clone(),
            to: edge_from_me.to.clone(),
        };

        let distance_to_me = self.distances.get(edge_to_me).unwrap();
        let distance_from_me = self.distances.get(edge_from_me).unwrap();

        let new_distance = distance_to_me + distance_from_me;

        // println!("Replacing {:?} and {:?} with {:?}", edge_to_me, edge_from_me, new_edge);

        self.edges.remove(edge_to_me);
        self.edges.remove(edge_from_me);
        self.edges.insert(new_edge.clone());

        self.distances.remove(edge_to_me);
        self.distances.remove(edge_from_me);
        self.distances.insert(new_edge, new_distance);
    }

    fn removable_node(&self) -> Option<(String, Vec<Edge>, Vec<Edge>)> {
        // dbg!(&self);
        for node in self.nodes.iter() {
            // dbg!(&node);
            let edges_to_me = self
                .edges
                .iter()
                .filter(|edge| edge.to == **node)
                .map(|edge| edge.clone())
                .collect_vec();
            // dbg!(&edges_to_me);
            let nodes_to_me: HashSet<&String> = HashSet::from_iter(edges_to_me.iter().map(|edge| &edge.from));
            // dbg!(&nodes_to_me);

            let edges_from_me = self
                .edges
                .iter()
                .filter(|edge| edge.from == **node)
                .map(|edge| edge.clone())
                .collect_vec();
            // dbg!(&edges_from_me);
            let nodes_from_me: HashSet<&String> = HashSet::from_iter(edges_from_me.iter().map(|edge| &edge.to));
            // dbg!(&nodes_from_me);

            if edges_to_me.len() == 1 && edges_from_me.len() == 1 && nodes_from_me != nodes_to_me {
                return Some((node.clone(), edges_to_me, edges_from_me));
            } else if edges_to_me.len() == 2 && edges_to_me.len() == 2 && nodes_from_me == nodes_to_me {
                return Some((node.clone(), edges_to_me, edges_from_me));
            } else {
                // println!(
                //     "Node {} is not removable: edges to: {:?}, edges from: {:?}, nodes to: {:?}, nodes from: {:?}",
                //     node, edges_to_me, edges_from_me, nodes_to_me, nodes_from_me
                // );
                // println!();
            }
        }

        None
    }

    #[allow(dead_code)]
    pub fn print_dot(&self) {
        let mut output = String::new();
        output.push_str("digraph x { ");

        for edge in &self.edges {
            let distance = self.distances.get(edge).unwrap();
            output.push_str(
                format!(
                    "\"{}\"->\"{}\"[label=\"{}\",weight=\"{}\"];",
                    edge.from, edge.to, distance, distance,
                )
                .as_str(),
            );
        }

        output.push_str(" }");

        println!("{}", output);
    }

    pub(crate) fn neighbours(&self, node: String) -> Vec<(String, usize)> {
        self.edges
            .iter()
            .filter(|edge| edge.from == node)
            .map(|edge| (edge.to.clone(), *self.distances.get(&edge).unwrap()))
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::graph::Graph;

    #[test]
    fn compact_directed() {
        let mut actual = Graph::new();
        for (from, to, distance) in vec![
            ("A", "B", 2),
            ("B", "C", 3),
            ("C", "D", 5),
            ("D", "E", 7),
            ("B", "F", 11),
            ("F", "G", 13),
            ("G", "H", 17),
        ] {
            actual.insert_edge_directed(String::from(from), String::from(to), distance);
        }

        let mut expected = Graph::new();
        for (from, to, distance) in vec![("A", "B", 2), ("B", "E", 15), ("B", "H", 41)] {
            expected.insert_edge_directed(String::from(from), String::from(to), distance);
        }

        actual.compact();

        assert_eq!(actual, expected);
    }

    #[test]
    fn compact_undirected() {
        let mut actual = Graph::new();
        for (from, to, distance) in vec![
            ("A", "B", 2),
            ("B", "C", 3),
            ("C", "D", 5),
            ("D", "E", 7),
            ("B", "F", 11),
            ("F", "G", 13),
            ("G", "H", 17),
        ] {
            actual.insert_edge_undirected(String::from(from), String::from(to), distance);
        }

        let mut expected = Graph::new();
        for (from, to, distance) in vec![("A", "B", 2), ("B", "E", 15), ("B", "H", 41)] {
            expected.insert_edge_undirected(String::from(from), String::from(to), distance);
        }

        actual.compact();

        assert_eq!(actual, expected);
    }
}
