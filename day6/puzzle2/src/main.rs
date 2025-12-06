use aoc::Result;
use std::fs;

pub fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<char>) {
    // determine column widths
    let mut lines = input.lines().collect::<Vec<&str>>();

    let operator_line = lines.pop().unwrap();

    let mut column_widths: Vec<usize> = Vec::new();
    let mut column_operators: Vec<char> = Vec::new();

    let mut column_width = 0;
    for i in 0..operator_line.len() {
        match &operator_line[i..i + 1] {
            "*" => {
                if column_width > 1 {
                    column_widths.push(column_width - 1);
                }
                column_width = 1;
                column_operators.push('*');
            }
            "+" => {
                if column_width > 1 {
                    column_widths.push(column_width - 1);
                }
                column_width = 1;
                column_operators.push('+');
            }
            _ => {
                column_width += 1;
            }
        }
    }
    column_widths.push(column_width);

    let mut blocks: Vec<Vec<usize>> = Vec::new();

    for i in lines {
        let mut offset = 0;
        for (column_index, width) in column_widths.iter().enumerate() {
            let chunk = &i[offset..offset + width];

            let block = if let Some(block) = blocks.get_mut(column_index) {
                block
            } else {
                blocks.push(Vec::new());
                blocks.get_mut(column_index).unwrap()
            };

            for j in 0..chunk.len() {
                let byte = &chunk[j..j + 1];

                let column = if let Some(column) = block.get_mut(j) {
                    column
                } else {
                    block.push(0);
                    block.get_mut(j).unwrap()
                };

                if byte != " " {
                    let digit = &chunk[j..j + 1].parse::<usize>().unwrap();

                    if *column > 0 {
                        *column = *column * 10 + *digit;
                    } else {
                        *column = *digit;
                    }
                }
            }

            offset += width + 1; // add separator
        }
    }
    (blocks, column_operators)
}

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let (blocks, operators) = parse(&input);

    let mut sum = 0;
    for (column_index, operator) in operators.iter().enumerate() {
        let values = blocks.get(column_index).unwrap();
        match operator {
            '+' => {
                sum += values.iter().sum::<usize>();
            }
            '*' => {
                let mut mult = 1;
                values.iter().for_each(|x| mult *= x);
                sum += mult;
            }
            _ => panic!("Unsupported operator"),
        }
    }

    Ok(format!("{sum}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
