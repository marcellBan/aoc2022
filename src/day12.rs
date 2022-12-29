use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io,
    rc::Rc,
};

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct Pos {
    x: usize,
    y: usize,
}

impl Default for Pos {
    fn default() -> Self {
        Pos { x: 0, y: 0 }
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Pos {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Node {
    height: char,
    pos: Pos,
    neighbours: RefCell<Vec<Rc<Node>>>,
}

struct Map {
    nodes: HashMap<Pos, Rc<Node>>,
    start: Rc<Node>,
    end: Rc<Node>,
}

impl From<&Vec<String>> for Map {
    fn from(value: &Vec<String>) -> Self {
        let mut height_map = value
            .iter()
            .map(|l| l.chars().map(char::into).collect::<Vec<u32>>())
            .collect::<Vec<_>>();

        let mut end = Pos::default();
        let mut start = Pos::default();

        for row in 0..height_map.len() {
            for col in 0..height_map[0].len() {
                match char::from_u32(height_map[row][col]).unwrap() {
                    'S' => {
                        start = Pos::from((col, row));
                        height_map[row][col] = 'a' as u32;
                    }
                    'E' => {
                        end = Pos::from((col, row));
                        height_map[row][col] = 'z' as u32;
                    }
                    _ => (),
                }
            }
        }
        let mut nodes: HashMap<Pos, Rc<Node>> = HashMap::new();
        (0..height_map.len()).for_each(|row| {
            (0..height_map[0].len()).for_each(|col| {
                nodes.insert(
                    Pos::from((col, row)),
                    Node {
                        height: char::from_u32(height_map[row][col]).unwrap(),
                        pos: Pos::from((col, row)),
                        neighbours: Vec::new().into(),
                    }
                    .into(),
                );
            })
        });
        for (p, n) in &nodes {
            *n.neighbours.borrow_mut() = generate_valid_neighbour_positions(p, &height_map)
                .iter()
                .map(|x| nodes.get(x).unwrap().to_owned())
                .collect::<Vec<_>>();
        }
        let start = nodes.get(&start).unwrap().to_owned();
        let end = nodes.get(&end).unwrap().to_owned();
        Map { nodes, start, end }
    }
}

impl Map {
    fn find_shortest_path_from_start(self: &Self, start_pos: &Pos) -> usize {
        let mut dist: HashMap<Pos, usize> = HashMap::new();
        let mut prev: HashMap<Pos, Option<Rc<Node>>> = HashMap::new();
        let mut to_process: HashSet<Pos> = HashSet::new();

        for (p, _) in &self.nodes {
            dist.insert(p.clone(), usize::MAX);
            prev.insert(p.clone(), None);
            to_process.insert(p.clone());
        }
        *dist.get_mut(start_pos).unwrap() = 0;

        while !to_process.is_empty() {
            let curr = to_process
                .iter()
                .min_by_key(|p| dist.get(p).unwrap())
                .unwrap()
                .clone();
            if curr == self.end.pos || dist.get(&curr).unwrap() == &usize::MAX {
                break;
            }
            to_process.remove(&curr);

            let interesting_neighbours = self
                .nodes
                .get(&curr)
                .unwrap()
                .neighbours
                .borrow()
                .iter()
                .filter(|n| to_process.contains(&n.pos))
                .map(|n| n.pos.clone())
                .collect::<Vec<_>>();

            for neighbour_pos in interesting_neighbours {
                let alt = dist.get(&curr).unwrap() + 1;
                if alt < *dist.get(&neighbour_pos).unwrap() {
                    *dist.get_mut(&neighbour_pos).unwrap() = alt;
                    *prev.get_mut(&neighbour_pos).unwrap() =
                        self.nodes.get(&curr).unwrap().to_owned().into();
                }
            }
        }

        *dist.get(&self.end.pos).unwrap()
    }
    fn find_shortest_path_from_end(self: &Self) -> usize {
        self.nodes
            .iter()
            .filter(|(_, n)| n.height == 'a')
            .map(|(p, _)| self.find_shortest_path_from_start(p))
            .min()
            .unwrap()
    }
}

fn generate_valid_neighbour_positions(pos: &Pos, map: &Vec<Vec<u32>>) -> Vec<Pos> {
    let mut neighbours = Vec::new();
    if pos.x > 0 {
        neighbours.push(Pos::from((pos.x - 1, pos.y)));
    }
    if pos.x < map[0].len() - 1 {
        neighbours.push(Pos::from((pos.x + 1, pos.y)));
    }
    if pos.y > 0 {
        neighbours.push(Pos::from((pos.x, pos.y - 1)));
    }
    if pos.y < map.len() - 1 {
        neighbours.push(Pos::from((pos.x, pos.y + 1)));
    }
    neighbours = neighbours
        .iter()
        .filter(|p| map[p.y][p.x] <= map[pos.y][pos.x] + 1)
        .map(|p| p.clone())
        .collect::<Vec<_>>();
    neighbours
}

use crate::input_reader;
pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/day12.in")?;

    let map = Map::from(&lines);

    println!(
        "Took {} steps from start",
        map.find_shortest_path_from_start(&map.start.pos)
    );

    println!(
        "Took {} steps from closest point",
        map.find_shortest_path_from_end()
    );

    Ok(())
}
