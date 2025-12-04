use aoc::Result;
use std::fs;

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
    map.iter().enumerate().for_each(|(pos, x)| {
        if *x == 46 {
            return;
        }
        let pos_x = pos as isize % line_len;
        let start_x = if pos_x > 0 { -1 } else {0};
        let end_x = if pos_x < line_len -1 { 1 } else {0};
        
        let mut neighbours = 0;
        'outer: for x in start_x..=end_x {
            for y in -1_isize..=1 {
                if !(x == 0 && y == 0) {
                    let index = pos as isize + x+ y * line_len;
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
            rolls += 1;
        }
    });

    Ok(format!("{rolls}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
