use aoc::Result;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
const MAX_LINE_WIDTH: usize = 141; // memory optimization
enum Mode {
    Search,
    Fill,
}
#[derive(Debug)]
struct LightPath {
    pos: usize,
    paths: Option<usize>,
    right: Option<Rc<RefCell<LightPath>>>,
    left: Option<Rc<RefCell<LightPath>>>,
}

impl LightPath {
    pub fn new(pos: usize) -> LightPath {
        LightPath {
            pos,
            paths: None,
            right: None,
            left: None,
        }
    }
    pub fn add_left(&mut self) -> &mut Self {
        if self.left.is_none() {
            self.left = Some(Rc::new(RefCell::new(LightPath::new(self.pos - 1))));
        }
        self
    }
    pub fn add_right(&mut self) -> &mut Self {
        if self.right.is_none() {
            self.right = Some(Rc::new(RefCell::new(LightPath::new(self.pos + 1))));
        }
        self
    }
    pub fn merge_right(&mut self, right: Option<Rc<RefCell<Self>>>) {
        self.right = right;
    }
    pub fn merge_left(&mut self, left: Option<Rc<RefCell<Self>>>) {
        self.left = left;
    }
    pub fn get_left(&self) -> Option<Rc<RefCell<Self>>> {
        self.left.clone()
    }
    pub fn get_right(&self) -> Option<Rc<RefCell<Self>>> {
        self.right.clone()
    }
    pub fn get_count(&mut self) -> usize {
        if let Some(paths) = self.paths {
            return paths;
        }
        let mut sub_paths = 0;
        if let Some(left) = &self.left {
            sub_paths += left.borrow_mut().get_count();
        }
        if let Some(right) = &self.right {
            sub_paths += right.borrow_mut().get_count();
        }
        if sub_paths == 0 {
            sub_paths = 1;
        }
        self.paths = Some(sub_paths);
        sub_paths
    }
}
pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut len = 0;
    const NONE: Option<Rc<RefCell<LightPath>>> = None;
    let mut beams: [Option<Rc<RefCell<LightPath>>>; MAX_LINE_WIDTH] = [NONE; MAX_LINE_WIDTH];
    let mut new_beams: [Option<Rc<RefCell<LightPath>>>; MAX_LINE_WIDTH] = [NONE; MAX_LINE_WIDTH];
    let mut mode: Mode = Mode::Search;
    let mut top_light_path: Option<Rc<RefCell<LightPath>>> = None;
    input.lines().for_each(|x| match mode {
        Mode::Search => {
            len = x.len();
            if let Some((pos, _x)) = x.char_indices().find(|(_pos, c)| *c == 'S') {
                mode = Mode::Fill;
                let light_path = Rc::new(RefCell::new(LightPath::new(pos)));
                beams[pos] = Some(light_path.clone());
                top_light_path = Some(light_path.clone());
            }
        }
        Mode::Fill => {
            x.char_indices()
                .filter(|(_pos, c)| *c == '^')
                .for_each(|(pos, _c)| {
                    let mut updates: Vec<(usize, Option<Rc<RefCell<LightPath>>>)> = Vec::new();
                    if let Some(light_path) = &beams[pos] {
                        let mut m = light_path.borrow_mut();
                        if pos > 0 {
                            if let Some(new_beam) = &new_beams[pos - 1] {
                                m.merge_left(Some(new_beam.clone()));
                            } else {
                                m.add_left();
                                updates.push((pos - 1, m.get_left()));
                            }
                        }
                        if pos < len - 1 {
                            if let Some(new_beam) = &new_beams[pos + 1] {
                                m.merge_right(Some(new_beam.clone()));
                            } else {
                                m.add_right();
                                updates.push((pos + 1, m.get_right()));
                            }
                        }
                        updates.push((pos, NONE));
                    }
                    for (i, l) in updates {
                        new_beams[i] = l.clone();
                        beams[i] = l;
                    }
                });
        }
    });

    let top_light_path = top_light_path.unwrap();
    let mut top_light_path = top_light_path.borrow_mut();
    let paths = top_light_path.get_count();
    Ok(format!("{paths}"))
}
fn main() {
    aoc::main(solver)
}
#[cfg(test)]
mod test;
