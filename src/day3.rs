use std::fs;
use std::io;
use std::ops;

pub fn solve() -> io::Result<()> {
    let content = fs::read_to_string("input/day3.txt")?;

    let sacks = content.split("\n").collect::<Vec<&str>>();
    let compartments = sacks
        .clone()
        .into_iter()
        .map(|x| (&x[..x.len() / 2], &x[x.len() / 2..]))
        .collect::<Vec<(&str, &str)>>();

    let appears_in_both = compartments
        .into_iter()
        .map(|x| -> char {
            let (one, two) = x;
            for c in one.chars() {
                if two.contains(c) {
                    return c;
                }
            }
            panic!("No badge found for {:?}", x);
        })
        .collect::<Vec<char>>();

    let priority_sum = appears_in_both
        .into_iter()
        .map(priority)
        .fold(0, ops::Add::add);

    println!("Individual priority sum: {}", priority_sum);

    let (groups, _) = sacks.as_chunks::<3>();

    let group_badges_priority_sum = groups
        .into_iter()
        .map(|x| -> char {
            for c in x[0].chars() {
                if x[1..].into_iter().all(|y| y.contains(c)) {
                    return c;
                }
            }
            panic!("No badge found for {:?}", x);
        })
        .map(priority)
        .fold(0, ops::Add::add);

    println!("Badge priority sum: {}", group_badges_priority_sum);

    Ok(())
}

fn priority(c: char) -> i32 {
    if c.is_lowercase() {
        (c as i32) - ('a' as i32) + 1
    } else {
        (c as i32) - ('A' as i32) + 27
    }
}
