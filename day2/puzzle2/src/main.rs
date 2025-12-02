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

    let mut i = 1;
    let half_len = len / 2;
    let mut split = 10;

    loop {
        if len % i == 0 {
            // this is faster than len.is_multiple_of(i)
            let cmp = input % split;
            let mut remainder = input / split;

            loop {
                let cur = remainder % split;
                if cur != cmp {
                    //not symmetrical
                    break;
                }
                remainder /= split;
                if remainder == 0 {
                    // all equal
                    return input;
                }
            }
        }
        i += 1;
        split *= 10;
        if i > half_len {
            return 0;
        }
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
    assert_eq!(check_if_invalid(111), 111);
    assert_eq!(check_if_invalid(1188511885), 1188511885);
    assert_eq!(check_if_invalid(1010), 1010);
    assert_eq!(check_if_invalid(123123123), 123123123);
    assert_eq!(check_if_invalid(824824824), 824824824);
    assert_eq!(check_if_invalid(123123123123123), 123123123123123);
}
