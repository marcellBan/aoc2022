use std::io;

use crate::input_reader;

pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/day4.txt")?;

    let pairs = lines
        .into_iter()
        .map(|x| {
            x.split(",")
                .map(|v| {
                    v.split("-")
                        .map(|y| y.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut free_elves = 0;
    let mut overlaps = 0;

    for pair in pairs {
        assert!(pair.len() == 2);
        assert!(pair[0].len() == 2);
        assert!(pair[1].len() == 2);
        if (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][1])
            || (pair[0][0] >= pair[1][0] && pair[0][1] <= pair[1][1])
        {
            free_elves += 1;
        }
        if (pair[0][0] <= pair[1][0] && pair[0][1] >= pair[1][0])
            || (pair[1][0] <= pair[0][0] && pair[1][1] >= pair[0][0])
        {
            overlaps += 1;
        }
    }
    println!("Free elves: {}", free_elves);
    println!("Overlaps: {}", overlaps);

    Ok(())
}
