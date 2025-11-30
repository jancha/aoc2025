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
