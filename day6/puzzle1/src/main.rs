use aoc::Result;
use std::fs;

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut sum: usize = 0;
    for line in input.lines() {
        let row = line.split_whitespace().collect::<Vec<&str>>();
        if row[0] == "+" || row[0] == "*" {
            row.iter().enumerate().for_each(|(i, x)| match *x {
                "*" => {
                    let mut row_val: usize = 1;
                    rows.get(i).unwrap().iter().for_each(|x| row_val *= *x);
                    sum += row_val;
                }
                "+" => {
                    rows.get(i).unwrap().iter().for_each(|x| sum += *x);
                }
                _ => panic!("Unsupported operator"),
            });
        } else {
            row.iter().enumerate().for_each(|(i, x)| {
                let x = x.parse::<usize>().unwrap();
                if let Some(row) = rows.get_mut(i) {
                    row.push(x);
                } else {
                    rows.push(vec![x]);
                };
            })
        }
    }
    Ok(format!("{sum}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
