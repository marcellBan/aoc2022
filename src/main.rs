#![allow(dead_code)]
#![feature(slice_as_chunks)]

mod day1;
mod day2;
mod day3;
mod day4;
mod input_reader;

fn main() -> Result<(), std::io::Error> {
    day4::solve()?;

    Ok(())
}
