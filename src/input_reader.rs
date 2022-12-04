use std::fs;
use std::io;

pub fn read_input(file_path: &str) -> io::Result<Vec<String>> {
    let content = fs::read_to_string(file_path)?;
    Ok(content.split("\n").map(str::to_owned).collect::<Vec<_>>())
}
