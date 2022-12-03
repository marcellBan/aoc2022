use std::fs;
use std::ops;
pub fn solve() -> Result<(), std::io::Error> {
    let content = fs::read_to_string("input/day1.txt")?;
    let mut calories: Vec<Vec<i32>> = Vec::new();
    calories.push(Vec::new());
    let mut idx = 0usize;
    content.split("\n").for_each(|x| -> () {
        if x.is_empty() {
            calories.push(Vec::new());
            idx += 1;
        } else {
            calories[idx].push(x.parse::<i32>().unwrap());
        }
    });

    let mut sum_calories = calories
        .into_iter()
        .map(|x| -> i32 { x.into_iter().fold(0, ops::Add::add) })
        .collect::<Vec<i32>>();
    sum_calories.sort_by(|a, b| b.cmp(a));
    println!("Max calories: {}", sum_calories.first().unwrap_or(&-1i32));

    assert!(sum_calories.len() >= 3);

    let top3_sum = sum_calories[..3].into_iter().fold(0, ops::Add::add);
    println!("Top3 sum: {}", top3_sum);

    Ok(())
}
