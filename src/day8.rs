use std::io;

use crate::input_reader;

fn is_visible(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> bool {
    if row == 0 || col == 0 || row == (trees.len() - 1) || col == (trees[0].len() - 1) {
        true
    } else {
        (0..row).all(|y| trees[y][col] < trees[row][col])
            || (row + 1..trees.len()).all(|y| trees[y][col] < trees[row][col])
            || (0..col).all(|x| trees[row][x] < trees[row][col])
            || (col + 1..trees[0].len()).all(|x| trees[row][x] < trees[row][col])
    }
}

fn calc_scenic_score(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> u32 {
    if row == 0 || col == 0 || row == (trees.len() - 1) || col == (trees[0].len() - 1) {
        0
    } else {
        let up = {
            let mut sc = 0;
            for y in (0..row).rev() {
                sc += 1;
                if trees[y][col] >= trees[row][col] {
                    break;
                }
            }
            sc
        };
        let down = {
            let mut sc = 0;
            for y in row + 1..trees.len() {
                sc += 1;
                if trees[y][col] >= trees[row][col] {
                    break;
                }
            }
            sc
        };
        let left = {
            let mut sc = 0;
            for x in (0..col).rev() {
                sc += 1;
                if trees[row][x] >= trees[row][col] {
                    break;
                }
            }
            sc
        };
        let right = {
            let mut sc = 0;
            for x in col + 1..trees[0].len() {
                sc += 1;
                if trees[row][x] >= trees[row][col] {
                    break;
                }
            }
            sc
        };
        up * down * left * right
    }
}

pub fn solve() -> io::Result<()> {
    let lines = input_reader::read_input("input/day8.in")?;

    let trees = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| (c as u32) - ('0' as u32))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let visible_trees = (0..trees.len())
        .map(|r| {
            (0..trees[0].len())
                .filter(|c| is_visible(r, *c, &trees))
                .count()
        })
        .sum::<usize>();

    println!("Visible trees: {}", visible_trees);

    let max_scenic_score = (0..trees.len())
        .map(|r| {
            (0..trees[0].len())
                .map(|c| calc_scenic_score(r, c, &trees))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("Max scenic score: {}", max_scenic_score);

    Ok(())
}
