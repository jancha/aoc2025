use aoc::Result;
use std::fs;

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut total_joltage: u32 = 0;
    input.lines().for_each(|l| {
        let mut cells: Vec<u32> = Vec::new();
        l.chars().for_each(|x| {
            cells.push(x.to_digit(10).unwrap());
        });
        if let Some((key, value)) = cells[0..cells.len() - 1]
            .iter()
            .rev()
            .enumerate()
            .max_by_key(|&(_k, v)| v)
        {
            total_joltage += *value * 10 + cells[cells.len() - key - 1..].iter().max().unwrap()
        }
    });
    Ok(format!("{total_joltage}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
