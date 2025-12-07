use aoc::Result;
use std::fs;
const MAX_WIDTH: usize = 141; // memory optimization
const MAX_HEIGHT: usize = 142; // memory optimization
#[derive(Copy, Clone, PartialEq)]
enum Map {
    Empty,
    Start,
    Splitter,
}
pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut sums: [usize; MAX_WIDTH * MAX_HEIGHT] = [0; MAX_WIDTH * MAX_HEIGHT];
    let mut map: [Map; MAX_WIDTH * MAX_HEIGHT] = [Map::Empty; MAX_WIDTH * MAX_HEIGHT];
    let mut lines = 0;
    let mut line_width = 0;
    input.lines().for_each(|line| {
        if lines == 0 {
            line_width = line.len();
            if let Some((pos, _c)) = line.char_indices().find(|(_i, x)| *x == 'S') {
                map[pos] = Map::Start;
            }
        } else {
            line.char_indices()
                .filter(|(_pos, x)| *x == '^')
                .for_each(|(pos, _c)| {
                    map[lines * line_width + pos] = Map::Splitter;
                });
        }
        lines += 1;
    });
    for i in 0..lines - 2 {
        for j in 0..line_width {
            let index = i * line_width + j;
            match map[index] {
                Map::Start => {
                    sums[index + line_width] = 1;
                }
                Map::Splitter => {}
                Map::Empty => {
                    let sum = sums[index];
                    if sum > 0 {
                        let next_index = index + line_width;
                        if map[next_index] == Map::Splitter {
                            if j > 0 {
                                sums[next_index - 1] += sum;
                            }
                            if j < line_width - 1 {
                                sums[next_index + 1] += sum;
                            }
                        } else {
                            sums[next_index] += sum;
                        }
                    }
                }
            }
        }
    }
    Ok(sums[(lines - 2) * line_width..]
        .iter()
        .sum::<usize>()
        .to_string())
}

fn main() {
    aoc::main(solver)
}
#[cfg(test)]
mod test;
