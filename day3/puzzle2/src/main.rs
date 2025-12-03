use aoc::Result;
use std::fs;

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut total_joltage: u64 = 0;
    input.lines().for_each(|l| {
        let mut cells: Vec<u32> = Vec::new();
        l.chars().for_each(|x| {
            cells.push(x.to_digit(10).unwrap());
        });
        total_joltage += get_max(&cells, 0, 12);
    });
    Ok(format!("{total_joltage}"))
}

fn get_max(cells: &Vec<u32>, start: usize, cell_count: usize) -> u64 {
    let (next_max, next_max_pos) = get_next_max(
        cells,
        start,
        (cells.len() - cell_count).max(start).min(cells.len() - 1),
    );
    let mut joltage = 10_u64.pow(cell_count as u32 - 1) * next_max as u64;
    if cell_count > 1 {
        joltage += get_max(cells, next_max_pos + 1, cell_count - 1);
    }
    joltage
}

fn get_next_max(cells: &[u32], start: usize, end: usize) -> (u32, usize) {
    let mut max = 0;
    let mut max_pos = start;
    let mut pos = start;
    for i in &cells[start..=end] {
        if *i > max {
            max = *i;
            max_pos = pos;
        }
        pos += 1;
    }
    (max, max_pos)
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
