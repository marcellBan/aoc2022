#![allow(dead_code)]
#![feature(slice_as_chunks)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod input_reader;

fn main() -> Result<(), std::io::Error> {
    day6::solve()?;

    Ok(())
}
