use aoc::Result;
use std::fs;

#[derive(Debug)]
pub struct Socket {
    x: isize,
    y: isize,
    z: isize,
}

impl Socket {
    fn distance_to(&self, other: &Socket) -> usize {
        let dx: isize = self.x - other.x;
        let dy: isize = self.y - other.y;
        let dz: isize = self.z - other.z;
        (dx.pow(2) + dy.pow(2) + dz.pow(2)) as usize
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
    distance: usize,
}

#[derive(Clone)]
struct DisjointSetUnion {
    parent: Vec<usize>,
    size: Vec<usize>,
    components: usize,
}

impl DisjointSetUnion {
    fn new(components: usize) -> Self {
        let parent = (0..components).collect::<Vec<usize>>();
        let size: Vec<usize> = vec![1; components];
        Self {
            parent,
            size,
            components,
        }
    }
    fn find(&mut self, component: usize) -> usize {
        if self.parent[component] != component {
            let top = self.find(self.parent[component]);
            self.parent[component] = top;
        }
        self.parent[component]
    }

    fn union(&mut self, component_left: usize, component_right: usize) -> bool {
        let mut left = self.find(component_left);
        let mut right = self.find(component_right);
        if left == right {
            return false;
        }
        if self.size[left] < self.size[right] {
            std::mem::swap(&mut left, &mut right);
        }
        self.parent[right] = left;
        self.size[left] += self.size[right];
        self.components -= 1;
        true
    }
    fn size(&mut self, component: usize) -> usize {
        let parent = self.find(component);
        self.size[parent]
    }
}

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let top = if file_path.contains("test") { 10 } else { 1000 };
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
    distances.sort_by(|a, b| a.distance.cmp(&b.distance));
    let mut dsu = DisjointSetUnion::new(socket_count);
    for pair in distances.iter().take(top) {
        dsu.union(pair.from, pair.to);
    }
    dsu.size.sort();
    let result = dsu.size.iter().rev().take(3).product::<usize>();
    Ok(format!("{result}"))
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
