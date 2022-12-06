use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

use crate::input_reader;

pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/day6.txt")?;

    let mut last4: VecDeque<char> = VecDeque::new();

    for (i, c) in lines[0].chars().enumerate() {
        last4.push_back(c);
        if last4.len() > 4 {
            last4.pop_front();
        }
        let set: HashSet<char> = HashSet::from_iter(last4.iter().cloned());
        if set.len() == 4 {
            println!("Packet begins after {} characters", i + 1);
            break;
        }
    }

    last4 = VecDeque::new();

    for (i, c) in lines[0].chars().enumerate() {
        last4.push_back(c);
        if last4.len() > 14 {
            last4.pop_front();
        }
        let set: HashSet<char> = HashSet::from_iter(last4.iter().cloned());
        if set.len() == 14 {
            println!("Message begins after {} characters", i + 1);
            break;
        }
    }

    Ok(())
}
