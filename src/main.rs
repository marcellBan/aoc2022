#![allow(dead_code)]
#![feature(slice_as_chunks)]

mod day1;
mod day2;
mod day3;

fn main() -> Result<(), std::io::Error> {
    day3::solve()?;

    Ok(())
}
