use std::io;

use crate::input_reader;
pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/day10.in")?;

    let mut X = 1;
    let mut cycle = 0;
    let mut next_interesting_cycle = 20;
    let mut acc = 0;
    let mut register_in_time = Vec::from([1; 241]);

    for line in &lines {
        let cmd = line.split(" ").collect::<Vec<_>>();
        let delta = match cmd[0] {
            "noop" => {
                cycle += 1;
                register_in_time[cycle as usize - 1] = X;
                None
            }
            "addx" => {
                cycle += 2;
                register_in_time[cycle as usize - 2] = X;
                register_in_time[cycle as usize - 1] = X;
                Some(cmd[1].parse::<i32>().unwrap())
            }
            _ => panic!("Unknown cmd"),
        };
        if cycle >= next_interesting_cycle {
            acc += next_interesting_cycle * X;
            next_interesting_cycle += 40;
        }
        if let Some(d) = delta {
            X += d;
        }
    }

    println!("Sum of the six signal strengths: {}", acc);

    for (i, val) in register_in_time.iter().enumerate() {
        if i % 40 == 0 && i != 0 {
            print!("\n");
        }
        if i as i128 % 40 >= *val as i128 - 1 && i as i128 % 40 <= *val as i128 + 1 {
            print!("#");
        } else {
            print!(".");
        }
    }

    Ok(())
}
