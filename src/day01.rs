use std::fs;
use std::error::Error;

const INPUT: &str = "inputs/day01.txt";

fn read_file_as_string() -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(INPUT)?;
    Ok(contents)
}

pub fn part1() -> Result<i32, &str> {
    let file = read_file_as_string().unwrap_or_else(|err| {
        return Err("Error reading file.");
    });

    let mut floor = 0;
    for c in file.chars() {
        floor += match c {
            '(' => 1,
            ')' => -1
        }
    }

    Ok(floor)
}
