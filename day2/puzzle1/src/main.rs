use aoc::Result;
use std::fs;

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");

    let mut sum = 0;
    input.trim().split(",").for_each(|x| {
        if x.starts_with("0") {
            // invalid ID
            return;
        }
        let x = x.split("-").collect::<Vec<&str>>();

        let a = x
            .first()
            .unwrap()
            .parse::<usize>()
            .expect("Left side not a number");
        let b = x
            .last()
            .unwrap()
            .parse::<usize>()
            .expect("Right side not a number");

        for i in a..=b {
            sum += check_if_invalid(i);
        }
    });
    Ok(format!("{sum}"))
}

fn check_if_invalid(input: usize) -> usize {
    if input < 10 {
        // valid ID
        return 0;
    }
    let base = (input - input % 10) as isize;
    let len = base.ilog10() as usize + 1;

    if !len.is_multiple_of(2) {
        //not symetrical, valid ID
        return 0;
    }
    let split: usize = 10_usize.pow((len / 2).try_into().unwrap());
    let right = input % split;
    let left = input / split;

    if left == right {
        input
    } else {
        0
    }
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;

#[test]

fn test_check() {
    assert_eq!(check_if_invalid(1), 0);
    assert_eq!(check_if_invalid(10), 0);
    assert_eq!(check_if_invalid(101), 0);
    assert_eq!(check_if_invalid(1010), 1010);
}
