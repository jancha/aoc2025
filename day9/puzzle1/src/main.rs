use aoc::Result;
use std::fs;
pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut rect = 0;
    let mut xy: Vec<(usize, usize)> = Vec::new();

    input.lines().for_each(|line| {
        let mut line = line.split(",");
        xy.push((
            line.next().unwrap().parse::<usize>().unwrap(),
            line.next().unwrap().parse::<usize>().unwrap(),
        ));
    });

    for p1 in &xy {
        for p2 in &xy {
            if p1 == p2 {
                continue;
            }
            let dx = (p1.0 as isize - p2.0 as isize).abs() + 1;
            let dy = (p1.1 as isize - p2.1 as isize).abs() + 1;
            rect = rect.max(dx * dy);
        }
    }

    Ok(format!("{rect}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
