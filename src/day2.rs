use std::collections::HashMap;
use std::fs;
use std::ops;

pub fn solve() -> Result<(), std::io::Error> {
    let scores = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);
    let moves = HashMap::from([
        ("A X", "A Z"),
        ("A Y", "A X"),
        ("A Z", "A Y"),
        ("B X", "B X"),
        ("B Y", "B Y"),
        ("B Z", "B Z"),
        ("C X", "C Y"),
        ("C Y", "C Z"),
        ("C Z", "C X"),
    ]);

    let content = fs::read_to_string("input/day2.txt")?;
    let score = content
        .split("\n")
        .map(|x| scores[x])
        .fold(0, ops::Add::add);

    println!("Basic Score: {}", score);

    let score = content
        .split("\n")
        .map(|x| scores[moves[x]])
        .fold(0, ops::Add::add);

    println!("Correct Score: {}", score);

    Ok(())
}
