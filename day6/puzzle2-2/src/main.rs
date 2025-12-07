use aoc::Result;
use std::fs;

enum Transpose {
    CounterClockWise,
}

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let map: Vec<Vec<char>> = transpose(
        input.lines().map(|line| line.chars().collect()).collect(),
        Transpose::CounterClockWise,
        1,
    );
    let mut nums: Vec<usize> = Vec::new();
    let mut total = 0;
    map.iter().for_each(|row| {
        nums.push(
            row.iter()
                .filter(|x| x.is_ascii_digit())
                .rev()
                .enumerate()
                .map(|(pos, x)| {
                    (10_u32.pow(pos as u32) as usize) * (x.to_digit(10).unwrap() as usize)
                })
                .sum(),
        );

        if let Some(x) = row.iter().find(|x| **x == '+' || **x == '*') {
            total += if *x == '+' {
                nums.iter().filter(|x| **x != 0).sum::<usize>()
            } else {
                nums.iter().filter(|x| **x != 0).product::<usize>()
            };
            nums.clear();
        }
    });
    Ok(format!("{total}"))
}

fn transpose(map: Vec<Vec<char>>, direction: Transpose, times: usize) -> Vec<Vec<char>> {
    let mut transposed_map: Vec<Vec<char>> = Vec::new();
    for _i in 0..times {
        match direction {
            Transpose::CounterClockWise => {
                map.iter().for_each(|row| {
                    row.iter().rev().enumerate().for_each(|(col_index, char)| {
                        if let Some(row) = transposed_map.get_mut(col_index) {
                            row.push(*char);
                        } else {
                            transposed_map.push(vec![*char]);
                        };
                    });
                });
            }
        }
    }
    transposed_map
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
