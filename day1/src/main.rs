use aoc::Result;
use std::fs;

pub fn solver(file_path: &str) -> Result<String> {
    let _input = fs::read_to_string(file_path).expect("Could not read input file");
    Ok("Ok".to_string())
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
