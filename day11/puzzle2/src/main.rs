use aoc::Result;
use serde::Serialize;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::fs;
use std::rc::Rc;

#[derive(Debug, Clone, Serialize, PartialEq)]
struct Node {
    id: String,
    #[serde(skip)]
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
        let node_id = line.first().unwrap().to_string();
        let node = if let Some(node) = graph.iter().find(|node| (*node).borrow().id == node_id) {
            (*node).clone()
        } else {
            let node = Rc::new(RefCell::new(Node::new(node_id)));
            graph.push(node.clone());
            node.clone()
        };
        if let Some(targets) = line.get(1) {
            let targets: Vec<&str> = targets.trim().split(" ").collect::<Vec<&str>>();
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

    let fft = graph.iter().find(|x| x.borrow().id == "fft").unwrap();
    let dac = graph.iter().find(|x| x.borrow().id == "dac").unwrap();
    let svr = graph.iter().find(|x| x.borrow().id == "svr").unwrap();
    let out = graph.iter().find(|x| x.borrow().id == "out").unwrap();

    let topology = build_topology(&graph);

    let svr_fft = walk_graph_bfs(&topology, svr, &graph, fft);
    let fft_dac = walk_graph_bfs(&topology, fft, &graph, dac);
    let dac_out = walk_graph_bfs(&topology, dac, &graph, out);

    let total = svr_fft * fft_dac * dac_out;

    Ok(total.to_string())
}

fn walk_graph_bfs(
    topology: &VecDeque<String>,
    start: &Rc<RefCell<Node>>,
    graph: &[Rc<RefCell<Node>>],
    end: &Rc<RefCell<Node>>,
) -> usize {
    let mut ways: BTreeMap<String, usize> = BTreeMap::new();

    ways.insert(start.borrow().id.clone(), 1);

    topology.iter().for_each(|x| {
        let paths = if let Some(paths) = ways.get(x) {
            *paths
        } else {
            0
        };
        graph.iter().filter(|y| y.borrow().id == *x).for_each(|y| {
            y.borrow().children.iter().for_each(|c| {
                ways.entry(c.borrow().id.clone())
                    .and_modify(|w| {
                        *w += paths;
                    })
                    .or_insert(paths);
            });
        });
    });

    if let Some(value) = ways.get(&end.borrow().id) {
        *value
    } else {
        panic!("End not found in paths");
    }
}

fn build_topology(graph: &[Rc<RefCell<Node>>]) -> VecDeque<String> {
    let mut indegree: BTreeMap<String, usize> = BTreeMap::new();
    graph.iter().for_each(|n| {
        n.borrow().children.iter().for_each(|x| {
            indegree
                .entry(x.borrow().id.clone())
                .and_modify(|x| {
                    *x += 1;
                })
                .or_insert(1);
        });
        indegree.entry(n.borrow().id.clone()).or_insert(0);
    });

    let mut queue: VecDeque<String> = VecDeque::with_capacity(1000);

    indegree
        .iter()
        .filter(|x| *x.1 == 0)
        .for_each(|(node_id, _degree)| {
            queue.push_back(node_id.clone());
        });

    let mut topology: VecDeque<String> = VecDeque::with_capacity(1000);

    while let Some(node) = queue.pop_front() {
        topology.push_back(node.clone());
        if let Some(node_rc) = graph.iter().find(|x| x.borrow().id == node) {
            node_rc.borrow().children.iter().for_each(|n| {
                indegree.entry(n.borrow().id.clone()).and_modify(|x| {
                    *x -= 1;
                    if *x == 0 {
                        queue.push_back(n.borrow().id.clone());
                    }
                });
            });
        }
    }

    topology
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
