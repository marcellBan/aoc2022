use std::io;
use std::ops;

use crate::input_reader;
pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/day1.txt")?;
    let mut calories: Vec<Vec<i32>> = Vec::new();
    calories.push(Vec::new());
    let mut idx = 0usize;
    lines.into_iter().for_each(|x| -> () {
        if x.is_empty() {
            calories.push(Vec::new());
            idx += 1;
        } else {
            calories[idx].push(x.parse::<i32>().unwrap());
        }
    });

    let mut sum_calories = calories
        .into_iter()
        .map(|x| x.into_iter().fold(0, ops::Add::add))
        .collect::<Vec<_>>();
    sum_calories.sort_by(|a, b| b.cmp(a));
    println!("Max calories: {}", sum_calories.first().unwrap_or(&-1i32));

    assert!(sum_calories.len() >= 3);

    let top3_sum = sum_calories[..3].into_iter().fold(0, ops::Add::add);
    println!("Top3 sum: {}", top3_sum);

    Ok(())
}
