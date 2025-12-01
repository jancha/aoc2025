use aoc::{AOCError, Result};
use std::fs;

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut left_count = 0;
    let mut pos: isize = 50;
    let mut start_pos;
    for i in input.lines() {
        start_pos = pos;
        let dir = &i[0..1];
        let steps = i[1..].parse::<isize>().expect("Not a number");
        left_count += steps / 100;
        let steps = steps % 100;
        match dir {
            "L" => pos -= steps,
            "R" => pos += steps,
            _ => return Err(Box::new(AOCError::String("Invalid char".to_string()))),
        }
        let mut push_zero = false;
        if pos > 99 {
            push_zero = true;
            pos %= 100;
        } else if pos < 0 {
            push_zero = true;
            pos = 100 + pos % 100;
        }
        if pos == 0 || (push_zero && start_pos != 0) {
            left_count += 1;
        }
    }
    Ok(format!("{left_count}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
