use aoc::Result;
use std::fs;
enum Mode {
    Search,
    Fill,
}
pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut splits = 0;
    let mut len = 0;
    let mut beams: [u8; 141] = [0; 141];
    let mut mode: Mode = Mode::Search;
    input.lines().for_each(|x| match mode {
        Mode::Search => {
            len = x.len();
            if let Some((pos, _x)) = x.char_indices().find(|(_pos, c)| *c == 'S') {
                mode = Mode::Fill;
                beams[pos] = 1;
            }
        }
        Mode::Fill => {
            x.char_indices()
                .filter(|(_pos, c)| *c == '^')
                .for_each(|(pos, _c)| {
                    if beams[pos] == 1 {
                        splits += 1;
                        if pos > 0 {
                            beams[pos - 1] = 1;
                        }
                        if pos < len - 1 {
                            beams[pos + 1] = 1;
                        }
                        beams[pos] = 0;
                    }
                });
        }
    });
    Ok(format!("{splits}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
