use aoc::Result;
use std::fs;

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut ingredients = 0;
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut ranges_read = false;
    for i in input.lines() {
        let i = i.trim();
        if i.is_empty() {
            ranges_read = true;
            continue;
        }
        match ranges_read {
            false => {
                let row = i.split("-").collect::<Vec<&str>>();
                ranges.push((
                    row.first().unwrap().parse::<usize>().unwrap(),
                    row.last().unwrap().parse::<usize>().unwrap(),
                ));
            }
            true => {
                let ingredient = i.parse::<usize>().unwrap();
                if ranges
                    .iter()
                    .any(|(from, to)| ingredient >= *from && ingredient <= *to)
                {
                    ingredients += 1;
                }
            }
        }
    }
    Ok(format!("{ingredients}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
