use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    io,
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

struct TracePos {
    pos: Pos,
    possible_steps: RefCell<Vec<Pos>>,
}

struct Map {
    data: Vec<Vec<u32>>,
    start: Pos,
    end: Pos,
}

impl From<&Vec<String>> for Map {
    fn from(value: &Vec<String>) -> Self {
        let mut height_map = value
            .iter()
            .map(|l| l.chars().map(char::into).collect::<Vec<u32>>())
            .collect::<Vec<_>>();

        let mut end = Pos::default();
        let mut current = Pos::default();

        for row in 0..height_map.len() {
            for col in 0..height_map[0].len() {
                match char::from_u32(height_map[row][col]).unwrap() {
                    'S' => {
                        current = Pos::from((col, row));
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
        Map {
            data: height_map,
            start: current,
            end: end,
        }
    }
}

impl Map {
    fn at(self: &Self, pos: &Pos) -> u32 {
        self.data[pos.y][pos.x]
    }

    fn size(self: &Self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }
}

fn generate_valid_steps(pos: &Pos, map: &Map, visited: &HashSet<Pos>) -> Vec<Pos> {
    let mut steps = Vec::new();
    let (rows, cols) = map.size();
    if pos.x > 0 {
        steps.push(Pos::from((pos.x - 1, pos.y)));
    }
    if pos.x < cols - 1 {
        steps.push(Pos::from((pos.x + 1, pos.y)));
    }
    if pos.y > 0 {
        steps.push(Pos::from((pos.x, pos.y - 1)));
    }
    if pos.y < rows - 1 {
        steps.push(Pos::from((pos.x, pos.y + 1)));
    }
    steps = steps
        .iter()
        .filter(|p| map.at(p) <= map.at(pos) + 1 && !visited.contains(p))
        .map(|p| p.clone())
        .collect::<Vec<_>>();
    steps.sort_by_key(|p| map.at(p));
    steps
}

use crate::input_reader;
pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/day12.in")?;

    let map = Map::from(&lines);
    let mut current_pos = map.start.clone();

    let mut visited: HashSet<Pos> = HashSet::new();
    visited.insert(current_pos.clone());
    let mut trace: VecDeque<TracePos> = VecDeque::new();
    trace.push_back(TracePos {
        pos: current_pos.clone(),
        possible_steps: generate_valid_steps(&current_pos, &map, &visited).into(),
    });

    while current_pos != map.end && !trace.is_empty() {
        let maybe_next_pos = trace.back().unwrap().possible_steps.borrow_mut().pop();
        let next_pos = if let Some(p) = maybe_next_pos {
            p
        } else {
            trace.pop_back().unwrap();
            continue;
        };
        trace.push_back(TracePos {
            pos: next_pos.clone(),
            possible_steps: generate_valid_steps(&next_pos, &map, &visited).into(),
        });
        visited.insert(next_pos.clone());
        current_pos = next_pos;
    }

    println!("Took {} steps", trace.len() - 1);

    Ok(())
}
