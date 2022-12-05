use std::collections::VecDeque;
use std::io;

use crate::input_reader;

pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/day5.txt")?;

    let mut stacks = lines[..9]
        .into_iter()
        .map(|x| x.chars().collect::<VecDeque<_>>())
        .collect::<Vec<_>>();

    let mut second_stacks = stacks.clone();

    lines[10..].into_iter().for_each(|x| {
        let parts = x.split(" ").collect::<Vec<_>>();
        for _ in 0..(parts[1].parse::<usize>().unwrap()) {
            let ch = stacks[parts[3].parse::<usize>().unwrap() - 1]
                .back()
                .unwrap()
                .clone();
            stacks[parts[5].parse::<usize>().unwrap() - 1].push_back(ch);
            stacks[parts[3].parse::<usize>().unwrap() - 1].pop_back();
        }
    });

    let mut msg: String = String::new();
    for stack in stacks {
        msg.push(stack.back().unwrap().clone());
    }

    println!("First message: {}", msg);

    lines[10..].into_iter().for_each(|x| {
        let parts = x.split(" ").collect::<Vec<_>>();
        let mut tmp: VecDeque<char> = VecDeque::new();
        for _ in 0..(parts[1].parse::<usize>().unwrap()) {
            let ch = second_stacks[parts[3].parse::<usize>().unwrap() - 1]
                .back()
                .unwrap()
                .clone();
            tmp.push_back(ch);
            second_stacks[parts[3].parse::<usize>().unwrap() - 1].pop_back();
        }
        while let Some(c) = tmp.back() {
            second_stacks[parts[5].parse::<usize>().unwrap() - 1].push_back(c.clone());
            tmp.pop_back();
        }
    });

    let mut second_msg: String = String::new();
    for stack in second_stacks {
        second_msg.push(stack.back().unwrap().clone());
    }

    println!("Second message: {}", second_msg);

    Ok(())
}
