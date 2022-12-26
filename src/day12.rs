use std::{collections::HashSet, io};

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

fn generate_valid_steps(pos: &Pos, row_count: usize, col_count: usize) -> Vec<Pos> {
    let mut steps = Vec::new();
    if pos.x > 0 {
        steps.push(Pos::from((pos.x - 1, pos.y)));
    }
    if pos.x < col_count - 1 {
        steps.push(Pos::from((pos.x + 1, pos.y)));
    }
    if pos.y > 0 {
        steps.push(Pos::from((pos.x, pos.y - 1)));
    }
    if pos.y < row_count - 1 {
        steps.push(Pos::from((pos.x, pos.y + 1)));
    }
    println!("Possible: {:?}", steps);
    steps
}

fn find_best_step(pos: &Pos, map: &Vec<Vec<u32>>) -> Pos {
    generate_valid_steps(pos, map.len(), map[0].len())
        .iter()
        .filter(|p| map[p.y][p.x] <= map[pos.y][pos.x] + 1)
        .max_by_key(|p| map[p.y][p.x])
        .unwrap()
        .clone()
}

use crate::input_reader;
pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/test.in")?;

    let mut height_map = lines
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

    let mut steps: HashSet<Pos> = HashSet::new();

    while current != end {
        steps.insert(current.clone());
        current = find_best_step(&current, &height_map);
        println!("Updated: {:?}", current);
        if steps.contains(&current) {
            break;
        }
    }

    Ok(())
}
