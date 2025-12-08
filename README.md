# aoc2025
Advent of code 2025 (rust)


Helper libraries:


**bench**
Provides function benchmarking, used by **aoc** library

Usage:

    cargo run --release bench=100

This will run solver for default input 100 times to calculate total and average time

    cargo run --release bench=100 input=input_puzzle2.txt

This will run solver for specified input data 100 times to calculate total and average time


**aoc**
Provides `main` and `auto_test` functions

    fn main

This function helps you automate execution mode (single input or bench mode)

    fn auto_test

This function finds all `input_test*` files and runs them and compares output to respective `ouput_test*` files


**dayX**

For each day copy this simple template and implement solver method (and update Cargo.toml)

This will automatically let you use:

A. Run solver against input_puzzle1.txt which is expected default input

    cargo run --release

B. Run solver against input_puzzle1.txt which is expected default input

    cargo run --release input=input_puzzle2.txt

C. Run benchmarking feature for specific input

    cargo run --release input=input_puzzle1.txt bench=100

D. run all available tests

    cargo test -- --nocapture


## Performance

|Day |Puzzle | Average time | Comment                                                                 |
|----|-------|--------------|-------------------------------------------------------------------------|
|Day1|Puzzle1|  0.055ms     |                                                                         |
|Day1|Puzzle2|  0.056ms     | Basic mods                                                              |
|Day2|Puzzle1|  3.780ms     | is_multiple_of(n) as an alternative to n % 2 == 0 (but % feels faster)  |
|Day2|Puzzle2| 10.590ms     |                                                                         |
|Day3|Puzzle1|  0.080ms     |                                                                         |
|Day3|Puzzle2|  0.090ms     | max_by_key finds LAST max (not first)                                   |
|Day4|Puzzle1|  0.087ms     |                                                                         |
|Day4|Puzzle2|  1.900ms     | using optimised rollback for fast recalc                                |
|Day5|Puzzle1|  0.103ms     |                                                                         |
|Day5|Puzzle2|  0.076ms     | using range merge would be easier                                       |
|Day6|Puzzle1|  0.085ms     | split_whitespace is nice way to split by any number of spaces           |
|Day6|Puzzle2|  0.046ms     |                                                                         |
|Day6|Puzzle2|  0.193ms     | bonus solution - using transposing. much shorter code, but slower       |
|Day7|Puzzle1|  0.016ms     | filter + foreach is nice way to work on matching iterator elements      |
|Day7|Puzzle2|  0.084ms     | using DAG - Option<Rc<RefCell<LightBeam>>>> - complex but nice)         |
|Day7|Puzzle2|  0.034ms     | using GRID - straight foward                                            |
|Day8|Puzzle1| 16.300ms     | graphs and more                                                         |
|Day8|Puzzle2|120.150ms     | graphs and more brutforce (might revisit later)                         |

*Total: 153.4399ms*

