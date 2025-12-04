use aoc::Result;
use std::fs;
use std::collections::HashMap;

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut rolls = 0;
    let mut map: Vec<u8> = Vec::new();
    let mut lines = 0;
    input.trim().lines().for_each(|line| {
        let mut line = line.as_bytes().to_owned();
        map.append(&mut line);
        lines += 1;
    });
    let len = map.len() as isize;
    let line_len = (map.len() / lines) as isize;
    let mut pos:isize = 0;

    let mut neighbour_map:HashMap<isize, u8> = HashMap::new();

    loop {
        let x = &map[pos as usize];
        let mut next_pos = pos + 1;
        if *x != 46 {
            let pos_x = pos % line_len;
            let start_x = if pos_x > 0 { -1 } else {0};
            let end_x = if pos_x < line_len -1 { 1 } else {0};
            let mut neighbours = 0;
            'outer: for x in start_x..=end_x {
                for y in -1_isize..=1 {
                    if !(x == 0 && y == 0) {
                        let index = pos + x+ y * line_len;
                        if index >= 0 && index < len && let Some(v) = map.get(index as usize) && *v == 64 {
                            neighbours += 1;
                            if neighbours > 3 {
                                break 'outer;
                            }
                        }
                    }
                }
            }
            if neighbours < 4 {
                map[pos as usize] = 46; // remove roll
                rolls += 1;
                for x in start_x..=end_x {
                    for y in -1_isize..=1 {
                        if !(x == 0 && y == 0) {
                            let index = pos + x + y * line_len;
                            if index >= 0 && index < len {
                                neighbour_map.entry(index).and_modify(|prev| {
                                    *prev -= 1;
                                    if *prev < 4 {
                                        // let's remove it
                                        next_pos = next_pos.min(index);
                                    }
                                });
                            }
                        }
                    }
                }
                let _v =neighbour_map.remove(&pos);
            } else {
                neighbour_map.entry(pos).and_modify(|x| *x = neighbours).or_insert(neighbours);
            }
        }
        pos = next_pos;
        if pos == len {
            break;
        }
    }
    Ok(format!("{rolls}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
