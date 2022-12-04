use std::fs;
use std::io;

pub fn solve() -> io::Result<()> {
    let content = fs::read_to_string("input/day4.txt")?;

    let pairs = content
        .split("\n")
        .map(|x| x.split(",").collect::<Vec<&str>>())
        .map(|x| {
            x.into_iter()
                .map(|z| {
                    z.split("-")
                        .map(|y| y.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
        })
        .collect::<Vec<Vec<Vec<i32>>>>();

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
