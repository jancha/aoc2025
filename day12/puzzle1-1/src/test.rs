use super::*;
use aoc::auto_test;

#[test]
pub fn all_tests() {
    auto_test(solver)
}

#[test]
fn test_shape_build() {
    let shape = Shape::from((&vec!["###", "##.", "##."], 0));
    println!("Shape: {:#?}", shape)
}
