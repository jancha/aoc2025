use aoc::Result;
use std::fs;
pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let output = "";
    Ok(format!("{output}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
