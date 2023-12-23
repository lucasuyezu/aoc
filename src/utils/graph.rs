use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Edge {
    from: String,
    to: String,
}

#[derive(Debug, PartialEq, Eq)]
struct Graph {
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

    pub fn insert_edge(&mut self, from: String, to: String, distance: usize) {
        self.nodes.insert(from.clone());
        self.nodes.insert(to.clone());

        let new_edge = Edge {
            from: from.clone(),
            to: to.clone(),
        };

        self.edges.insert(new_edge.clone());

        self.distances.insert(new_edge, distance);
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
    //      v
    //      H
    pub fn compact(&mut self) {
        let mut stop = false;
        while !stop {
            match self.removable_node() {
                None => {
                    stop = true;
                }
                Some(node) => {
                    let edge_to_me = self.edges.iter().find(|edge| edge.to == node).unwrap().clone();
                    let edge_from_me = self.edges.iter().find(|edge| edge.from == node).unwrap().clone();

                    let new_edge = Edge {
                        from: edge_to_me.from.clone(),
                        to: edge_from_me.to.clone(),
                    };

                    let distance_to_me = self.distances.get(&edge_to_me).unwrap();
                    let distance_from_me = self.distances.get(&edge_from_me).unwrap();

                    let new_distance = distance_to_me + distance_from_me;

                    self.nodes.remove(&node);

                    self.edges.remove(&edge_from_me);
                    self.edges.remove(&edge_to_me);
                    self.edges.insert(new_edge.clone());

                    self.distances.remove(&edge_to_me);
                    self.distances.remove(&edge_from_me);
                    self.distances.insert(new_edge, new_distance);
                }
            }
        }
    }

    fn removable_node(&self) -> Option<String> {
        match self.nodes.iter().find(|node| {
            let edges_to_me = self.edges.iter().filter(|edge| edge.to == **node).count();
            let edges_from_me = self.edges.iter().filter(|edge| edge.from == **node).count();

            edges_to_me == 1 && edges_from_me == 1
        }) {
            None => None,
            Some(node) => Some(node.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::graph::Graph;

    #[test]
    fn compact() {
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
            actual.insert_edge(String::from(from), String::from(to), distance);
        }

        let mut expected = Graph::new();
        for (from, to, distance) in vec![("A", "B", 2), ("B", "E", 15), ("B", "H", 41)] {
            expected.insert_edge(String::from(from), String::from(to), distance);
        }

        actual.compact();

        assert_eq!(actual, expected);
    }
}
