use aoc::Result;
use std::fs;

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut ingredients = 0;
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    for i in input.lines() {
        let i = i.trim();
        if i.is_empty() {
            break;
        }
        let row = i.split("-").collect::<Vec<&str>>();
        let current_start = row.first().unwrap().parse::<i64>().unwrap();
        let current_end = row.last().unwrap().parse::<i64>().unwrap();
        ranges.push((current_start, current_end));
    }
    let len = ranges.len();
    for i in 0..len {
        let mut sub_ranges: Vec<(i64, i64)> = vec![ranges[i]];

        ranges.iter().take(i).for_each(|(rs, re)| {
            let mut new_sub_ranges: Vec<(i64, i64)> = Vec::new();
            sub_ranges.iter_mut().for_each(|(s, e)| {
                if *s >= *rs && *e <= *re {
                    //inclusive range, make it 0 for sum
                    *s = *rs;
                    *e = *rs - 1;
                } else if *s < *rs && *e >= *rs {
                    if *e > *re {
                        new_sub_ranges.push((*re + 1, *e));
                    }
                    *e = *rs - 1;
                } else if *s <= *re && *e > *re {
                    if *s < *rs {
                        new_sub_ranges.push((*s + 1, *rs - 1));
                    }
                    *s = *re + 1;
                }
            });

            if !new_sub_ranges.is_empty() {
                sub_ranges.append(&mut new_sub_ranges);
            }
        });
        sub_ranges
            .iter()
            .for_each(|(s, e)| ingredients += e - s + 1);
    }

    Ok(format!("{ingredients}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
