use std::{env, fs, io};

pub fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(match env::var("AOC_TEST") {
        Ok(_) => "input/test.in",
        _ => file_path,
    })?;
    Ok(content.split("\n").map(str::to_owned).collect::<Vec<_>>())
}
