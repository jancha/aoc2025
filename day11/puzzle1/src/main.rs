use aoc::Result;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
#[derive(Debug, Clone)]
struct Node {
    id: String,
    children: Vec<Rc<RefCell<Node>>>,
    out: bool,
}

impl Node {
    fn new(id: String) -> Node {
        let out = id == "out";
        Node {
            id,
            children: Vec::new(),
            out,
        }
    }
}

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");

    let mut graph: Vec<Rc<RefCell<Node>>> = Vec::new();

    input.lines().for_each(|line| {
        let line = line.split(":").collect::<Vec<&str>>();
        let node_id = line.first().unwrap();
        let node = if let Some(node) = graph.iter().find(|node| (*node).borrow().id == *node_id) {
            (*node).clone()
        } else {
            let node = Rc::new(RefCell::new(Node::new(line.first().unwrap().to_string())));
            graph.push(node.clone());
            node.clone()
        };

        if let Some(targets) = line.get(1) {
            let targets: Vec<&str> = targets.split(" ").collect::<Vec<&str>>();
            targets.iter().for_each(|target| {
                let target_node = if let Some(target_node) =
                    graph.iter().find(|x| (**x).borrow().id == *target)
                {
                    target_node.clone()
                } else {
                    let target_node = Rc::new(RefCell::new(Node::new(target.to_string())));
                    graph.push(target_node.clone());
                    target_node
                };
                node.borrow_mut().children.push(target_node);
            });
        }
    });

    let start = graph.iter().find(|x| x.borrow().id == "you").unwrap();
    let mut path: Vec<String> = Vec::new();
    Ok(walk_graph(start, &mut path).to_string())
}

fn walk_graph(start: &Rc<RefCell<Node>>, path: &mut Vec<String>) -> usize {
    let mut paths = 0;
    path.push(start.borrow().id.clone());
    start.borrow().children.iter().for_each(|target| {
        let target_b = target.borrow();
        if path.contains(&target_b.id) {
            return;
        }
        if target_b.out {
            paths += 1;
        } else {
            paths += walk_graph(target, path);
        }
    });
    path.pop();
    paths
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
