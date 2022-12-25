use std::{cell::RefCell, cmp::Ordering, collections::HashSet, io, ops::Add};

use crate::input_reader;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Add for Pos {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn adjust_pair_position(lead: &Pos, follow: &mut Pos) -> () {
    let delta_x = (lead.x - follow.x).abs();
    let delta_y = (lead.y - follow.y).abs();
    if delta_x < 2 && delta_y < 2 {
        return;
    }
    if delta_x > 1 && delta_y > 1 {
        *follow = Pos {
            x: match lead.x.cmp(&follow.x) {
                Ordering::Less => lead.x + 1,
                Ordering::Greater => lead.x - 1,
                _ => panic!("Not Possible"),
            },
            y: match lead.y.cmp(&follow.y) {
                Ordering::Less => lead.y + 1,
                Ordering::Greater => lead.y - 1,
                _ => panic!("Not Possible"),
            },
        };
    } else if delta_x > 1 {
        *follow = Pos {
            x: match lead.x.cmp(&follow.x) {
                Ordering::Less => lead.x + 1,
                Ordering::Greater => lead.x - 1,
                _ => panic!("Not Possible"),
            },
            y: lead.y,
        };
    } else if delta_y > 1 {
        *follow = Pos {
            x: lead.x,
            y: match lead.y.cmp(&follow.y) {
                Ordering::Less => lead.y + 1,
                Ordering::Greater => lead.y - 1,
                _ => panic!("Not Possible"),
            },
        };
    }
}

fn adjust_rope_position(rope: &Vec<RefCell<Pos>>) -> () {
    for idx in 0..rope.len() - 1 {
        adjust_pair_position(&*rope[idx].borrow(), &mut *rope[idx + 1].borrow_mut());
    }
}

fn make_moves_with_delta(
    rope: &Vec<RefCell<Pos>>,
    tail_visited: &mut HashSet<Pos>,
    delta: Pos,
    count: usize,
) -> () {
    for _ in 0..count {
        tail_visited.insert(rope.last().unwrap().borrow().clone());
        let new_head = rope.first().unwrap().borrow().clone() + delta.clone();
        *rope.first().unwrap().borrow_mut() = new_head;
        adjust_rope_position(rope);
    }
}

pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/day9.in")?;

    let rope: Vec<RefCell<Pos>> = Vec::from([
        RefCell::new(Pos::origin()),
        RefCell::new(Pos::origin()),
        RefCell::new(Pos::origin()),
        RefCell::new(Pos::origin()),
        RefCell::new(Pos::origin()),
        RefCell::new(Pos::origin()),
        RefCell::new(Pos::origin()),
        RefCell::new(Pos::origin()),
        RefCell::new(Pos::origin()),
        RefCell::new(Pos::origin()),
    ]);

    let mut tail_visited: HashSet<Pos> = HashSet::new();

    for line in &lines {
        let parts = line.split(" ").collect::<Vec<_>>();
        let delta = match parts[0] {
            "U" => Pos { x: 0, y: 1 },
            "D" => Pos { x: 0, y: -1 },
            "L" => Pos { x: -1, y: 0 },
            "R" => Pos { x: 1, y: 0 },
            unk => panic!("Unknown move: {}", unk),
        };
        make_moves_with_delta(
            &rope,
            &mut tail_visited,
            delta,
            parts[1].parse::<usize>().unwrap(),
        );
    }
    tail_visited.insert(rope.last().unwrap().borrow().clone());

    println!("Tail visited position count: {}", tail_visited.len());

    Ok(())
}
