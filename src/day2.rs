use std::collections::HashMap;
use std::io;
use std::ops;

use crate::input_reader;

pub fn solve() -> io::Result<()> {
    let scores = HashMap::from([
        ("A X".to_owned(), 4),
        ("A Y".to_owned(), 8),
        ("A Z".to_owned(), 3),
        ("B X".to_owned(), 1),
        ("B Y".to_owned(), 5),
        ("B Z".to_owned(), 9),
        ("C X".to_owned(), 7),
        ("C Y".to_owned(), 2),
        ("C Z".to_owned(), 6),
    ]);
    let moves = HashMap::from([
        ("A X".to_owned(), "A Z".to_owned()),
        ("A Y".to_owned(), "A X".to_owned()),
        ("A Z".to_owned(), "A Y".to_owned()),
        ("B X".to_owned(), "B X".to_owned()),
        ("B Y".to_owned(), "B Y".to_owned()),
        ("B Z".to_owned(), "B Z".to_owned()),
        ("C X".to_owned(), "C Y".to_owned()),
        ("C Y".to_owned(), "C Z".to_owned()),
        ("C Z".to_owned(), "C X".to_owned()),
    ]);

    let lines = input_reader::read_input("input/day2.txt")?;
    let score = lines
        .clone()
        .into_iter()
        .map(|x| scores[&x])
        .fold(0, ops::Add::add);

    println!("Basic Score: {}", score);

    let score = lines
        .into_iter()
        .map(|x| scores[&moves[&x]])
        .fold(0, ops::Add::add);

    println!("Correct Score: {}", score);

    Ok(())
}
