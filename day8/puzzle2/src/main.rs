use aoc::{AOCError, Result};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs;
use std::rc::Rc;

#[derive(Debug)]
pub struct Socket {
    x: isize,
    y: isize,
    z: isize,
}

impl Socket {
    fn distance_to(&self, other: &Socket) -> f64 {
        let dx: isize = self.x - other.x;
        let dy: isize = self.y - other.y;
        let dz: isize = self.z - other.z;
        ((dx.pow(2) + dy.pow(2) + dz.pow(2)) as f64).sqrt()
    }
}

impl From<Vec<isize>> for Socket {
    fn from(coordinates: Vec<isize>) -> Self {
        Socket {
            x: *coordinates.first().expect("X position not available"),
            y: *coordinates.get(1).expect("Y position not available"),
            z: *coordinates.get(2).expect("Z position not available"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct PairDistance {
    from: usize,
    to: usize,
    distance: f64,
}

struct ConnectedSocket {
    edges: Vec<Rc<RefCell<ConnectedSocket>>>,
    visited: bool,
}

impl ConnectedSocket {
    fn walk(from: &Rc<RefCell<ConnectedSocket>>) -> usize {
        if from.borrow().visited {
            0
        } else {
            let mut s = from.borrow_mut();
            s.visited = true;
            drop(s);

            let mut sum = 1;
            for to in &from.borrow().edges {
                sum += ConnectedSocket::walk(to);
            }
            sum
        }
    }
}

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut distances: Vec<PairDistance> = Vec::new();
    let mut sockets: Vec<Socket> = Vec::new();
    let mut socket_count = 0;
    input.lines().enumerate().for_each(|(line_num, xyz)| {
        let socket = xyz
            .split(",")
            .map(|i| i.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        let next_socket = Socket::from(socket);
        sockets.iter().enumerate().for_each(|(pos, socket)| {
            let distance = socket.distance_to(&next_socket);
            distances.push(PairDistance {
                from: line_num,
                to: pos,
                distance,
            });
        });
        sockets.push(next_socket);
        socket_count = line_num + 1;
    });

    distances.sort_by(|a, b| a.distance.total_cmp(&b.distance));
    let mut connected_sockets: BTreeMap<usize, Rc<RefCell<ConnectedSocket>>> = BTreeMap::new();
    for pair in distances.iter() {
        let from = if let Some((_id, socket)) =
            connected_sockets.iter().find(|(id, _s)| **id == pair.from)
        {
            socket
        } else {
            let socket = Rc::new(RefCell::new(ConnectedSocket {
                edges: Vec::new(),
                visited: false,
            }));
            connected_sockets.insert(pair.from, socket);
            connected_sockets.get(&pair.from).unwrap()
        };
        let from = from.clone();
        let to = if let Some((_id, socket)) =
            connected_sockets.iter().find(|(id, _s)| **id == pair.to)
        {
            socket
        } else {
            let socket = Rc::new(RefCell::new(ConnectedSocket {
                edges: Vec::new(),
                visited: false,
            }));
            connected_sockets.insert(pair.to, socket);
            connected_sockets.get(&pair.to).unwrap()
        };
        from.borrow_mut().edges.push(to.clone());
        to.borrow_mut().edges.push(from.clone());
        for socket in connected_sockets.values() {
            socket.borrow_mut().visited = false;
        }
        // count and see if have at least one, that has walk size of entire network
        let mut max_walk = 0;
        for socket in connected_sockets.values() {
            if !socket.borrow().visited {
                max_walk = max_walk.max(ConnectedSocket::walk(socket));
                if max_walk == socket_count {
                    return Ok(format!("{}", sockets[pair.from].x * sockets[pair.to].x));
                }
            }
        }
    }
    Err(Box::new(AOCError::String(
        "Could not find solution".to_string(),
    )))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
