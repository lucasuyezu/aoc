use std::{collections::HashSet, str::FromStr};

use crate::utils::{
    point::{Point, EAST, NORTH, SOUTH, WEST},
    ParseInputError,
};

#[derive(Debug)]
pub struct Maze {
    data: Vec<Vec<char>>,
    pub starting_point: Point,
}

impl FromStr for Maze {
    type Err = ParseInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maze = Maze {
            data: s.lines().map(|line| line.chars().collect()).collect(),
            starting_point: Point { x: 0, y: 0 },
        };

        maze.starting_point = start_point(&maze);

        Ok(maze)
    }
}

fn start_point(maze: &Maze) -> Point {
    for (x, line) in maze.data.iter().enumerate() {
        for (y, c) in line.iter().enumerate() {
            if *c == 'S' {
                return Point {
                    x: (x as isize),
                    y: (y as isize),
                };
            }
        }
    }

    panic!("Could not find the starting point")
}

impl Maze {
    pub fn walk(&self) -> usize {
        println!("Starting at {:?}", self.starting_point);

        let mut visited_points: HashSet<Point> = HashSet::new();
        visited_points.insert(self.starting_point);

        let mut n = 1;
        let mut current_point = self.starting_neighbour(self.starting_point);
        while current_point != self.starting_point {
            let c = self.get_char(current_point).unwrap();
            println!("Now on {} at {:?}", c, current_point);
            visited_points.insert(current_point);
            let neighbours = self.neighbours(current_point);
            println!("\tNeighbours are {:?}", neighbours);

            assert!(neighbours.len() <= 2);

            current_point = *neighbours
                .iter()
                .find(|neighbour| !visited_points.contains(neighbour))
                .unwrap_or(&self.starting_point);

            n += 1;
        }

        println!("Came back to starting pos after {:?} nodes", n);

        n / 2
    }

    fn starting_neighbour(&self, point: Point) -> Point {
        if self.connecting_points(point + NORTH).contains(&point) {
            point + NORTH
        } else if self.connecting_points(point + EAST).contains(&point) {
            point + EAST
        } else if self.connecting_points(point + SOUTH).contains(&point) {
            point + SOUTH
        } else if self.connecting_points(point + WEST).contains(&point) {
            point + WEST
        } else {
            panic!("Could not find starting neighbour");
        }
    }

    fn neighbours(&self, point: Point) -> Vec<Point> {
        let mut neighbours: Vec<Point> = vec![];

        let my_connections = self.connecting_points(point);
        dbg!(&my_connections);

        let north = point + NORTH;
        if my_connections.contains(&north) && self.connecting_points(north).contains(&point) {
            neighbours.push(point + NORTH);
        }
        dbg!(&neighbours);

        let east = point + EAST;
        if my_connections.contains(&east) && self.connecting_points(east).contains(&point) {
            neighbours.push(point + EAST);
        }
        dbg!(&neighbours);

        let south = point + SOUTH;
        if my_connections.contains(&south) && self.connecting_points(south).contains(&point) {
            neighbours.push(point + SOUTH);
        }
        dbg!(&neighbours);

        let west = point + WEST;
        if my_connections.contains(&west) && self.connecting_points(west).contains(&point) {
            neighbours.push(point + WEST);
        }
        dbg!(&neighbours);

        neighbours
    }

    // fn neighbours(&self, point: Point) -> Vec<Point> {
    //     let mine: HashSet<Point> = HashSet::from_iter(self.connects_to(point));
    //     dbg!(&mine);

    //     let mut theirs: Vec<Point> = vec![];
    //     theirs.append(&mut self.connects_to(point + NORTH));
    //     dbg!(&theirs);
    //     todo!();
    //     theirs.append(&mut self.connects_to(point + EAST));
    //     dbg!(&theirs);
    //     theirs.append(&mut self.connects_to(point + SOUTH));
    //     dbg!(&theirs);
    //     theirs.append(&mut self.connects_to(point + WEST));
    //     dbg!(&theirs);

    //     dbg!(&theirs);
    //     mine.intersection(&HashSet::from_iter(theirs)).map(|p| *p).collect()
    // }

    // pub fn pipe_type(&self, point: Point) -> Result<char, &'static str> {
    //     let east = self.get_char(point + EAST);
    //     let west = self.get_char(point + WEST);
    //     let south = self.get_char(point + SOUTH);
    //     let north = self.get_char(point + NORTH);
    //     println!("Point to the east is {:?}", east);
    //     println!("Point to the south is {:?}", south);
    //     println!("Point to the west is {:?}", west);

    //     if self.connects_to(point, EAST) && self.connects_to(point, SOUTH) {
    //         Ok('F')
    //     } else {
    //         Err("Could not find pipe type")
    //     }
    // }

    fn connecting_points(&self, point: Point) -> Vec<Point> {
        let c = self.get_char(point).unwrap_or('x');
        println!("\tPoints connecting to {} at {:?}", c, point);
        match c {
            '|' => vec![point + NORTH, point + SOUTH],
            '-' => vec![point + EAST, point + WEST],
            'L' => vec![point + NORTH, point + EAST],
            'J' => vec![point + NORTH, point + WEST],
            '7' => vec![point + SOUTH, point + WEST],
            'F' => vec![point + SOUTH, point + EAST],
            _ => vec![],
        }
    }

    fn has_neighbour(&self, point: Point) -> bool {
        (0..self.data.len()).contains(&(point.x as usize))
            && (0..self.data.get(point.x as usize).unwrap().len()).contains(&(point.y as usize))
    }

    pub fn get_char(&self, point: Point) -> Option<char> {
        if self.has_neighbour(point) {
            Some(self.data.as_slice()[point.x as usize].as_slice()[point.y as usize])
        } else {
            None
        }
    }
}
