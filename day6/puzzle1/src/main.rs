use aoc::Result;
use std::fs;

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut rows: Vec<Vec<usize>> = Vec::new();
    let mut sum: usize = 0;
    for line in input.lines() {
        let row = line.replace("  ", " ");
        let row = row.trim();
        let row = row
            .split(" ")
            .filter(|&x| !x.is_empty())
            .collect::<Vec<&str>>();
        if row[0] == "+" || row[0] == "*" {
            row.iter().enumerate().for_each(|(i, x)| match *x {
                "*" => {
                    let mut row_val: usize = 0;
                    rows.get(i).unwrap().iter().for_each(|x| {
                        if row_val == 0 {
                            row_val = *x;
                        } else {
                            row_val *= *x;
                        }
                    });

                    sum += row_val;
                }
                "+" => {
                    let mut row_val: usize = 0;
                    rows.get(i).unwrap().iter().for_each(|x| {
                        if row_val == 0 {
                            row_val = *x;
                        } else {
                            row_val += *x;
                        }
                    });

                    sum += row_val;
                }
                _ => panic!("Unsupported operator"),
            });
        } else {
            row.iter().enumerate().for_each(|(i, x)| {
                let vec_row = if let Some(row) = rows.get_mut(i) {
                    row
                } else {
                    rows.push(Vec::new());
                    rows.get_mut(i).unwrap()
                };
                vec_row.push(x.parse::<usize>().unwrap());
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
