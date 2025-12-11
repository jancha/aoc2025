use super::*;
use aoc::auto_test;
use serde_json::*;
use std::fs;

#[test]
pub fn all_tests() {
    auto_test(solver)
}

fn write_to_file(graph: &[Rc<RefCell<Node>>], links: &Vec<Link>) {
    let plain_nodes = graph
        .iter()
        .map(|x| json!({"id":x.borrow().id}))
        .collect::<Vec<Value>>();
    let data = json!({"nodes": plain_nodes, "links": links});
    let data = data.to_string();
    fs::write("out.js", format!("data = {};", data)).unwrap();
}
