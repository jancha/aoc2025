use aoc::Result;
use std::fs;

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");

    let mut total_joltage: usize = 0;

    let mut bank_len = 0;
    input.lines().for_each(|l| {
        let l = l.trim();
        let mut bank_joltage = 0;
        let mut max = 0;
        if bank_len == 0 {
            bank_len = l.len();
        }
        let mut cells: Vec<u32> = Vec::new();
        l.chars().for_each(|x| {
            cells.push(x.to_digit(10).unwrap());
        });
        for i in 0..bank_len - 1 {
            let cell = cells[i];
            if cell >= max {
                let remaining_max = cells[i + 1..].iter().max().unwrap();
                let joltage = cell * 10 + remaining_max;

                if joltage > bank_joltage {
                    bank_joltage = joltage;
                    max = cell;
                }
            }
        }
        total_joltage += bank_joltage as usize;
    });

    Ok(format!("{total_joltage}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
