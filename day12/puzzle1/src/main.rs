use aoc::{AOCError, Result};
use std::{collections::VecDeque, fs};

const AREA_HEIGHT: usize = 51;
type ShapeRow = [u8; 3];
type Area = [u64; AREA_HEIGHT];

#[derive(Debug)]
pub struct Shape {
    pub rows: ShapeRow,
    pub variants: Vec<ShapeRow>,
    pub id: usize,
}

impl Shape {
    fn new(rows: [u8; 3], id: usize) -> Shape {
        let mut shape = Shape {
            rows,
            variants: Vec::with_capacity(8),
            id,
        };
        shape.build_variants();
        shape
    }

    fn rotate(&mut self) {
        let map: [[u8; 4]; 9] = [
            [0, 0, 0, 2], // first row bitwise operations, therefore from right side
            [1, 0, 0, 1], // first row
            [2, 0, 0, 0], // first row
            //
            [0, 1, 1, 2], // middle row
            [1, 1, 1, 1], // middle row
            [2, 1, 1, 0], // middle row
            //
            [0, 2, 2, 2], // bottom row
            [1, 2, 2, 1], // bottom row
            [2, 2, 2, 0], //
        ];
        //
        self.transform(&map);
    }

    fn transform(&mut self, map: &[[u8; 4]; 9]) {
        let mut new_rows: ShapeRow = [0; 3];
        map.iter().for_each(|byte| {
            let x_from = byte[0];
            let y_from = byte[1] as usize;

            let x_to = byte[2];
            let y_to = byte[3] as usize;

            let from_bit = (self.rows[y_from] & (1 << x_from)) >> x_from;

            let to_val = from_bit << x_to;

            new_rows[y_to] |= to_val;
        });

        self.rows = new_rows;
    }

    fn build_variants(&mut self) {
        for _j in 0..2 {
            for _i in 0..4 {
                if !self.variants.contains(&self.rows) {
                    self.variants.push(self.rows);
                }
                self.rotate();
            }
            self.flip_vertical();
        }
    }

    fn flip_vertical(&mut self) {
        let r0 = self.rows[0];
        let r2 = self.rows[2];
        self.rows[0] = r2;
        self.rows[2] = r0;
    }

    fn row_from(s: &str) -> u8 {
        let mut row: u8 = 0;

        s.chars().rev().enumerate().for_each(|(pos, c)| {
            if c == '#' {
                row |= 1 << pos
            }
        });
        row
    }
}

impl From<(&Vec<&str>, usize)> for Shape {
    fn from(shape: (&Vec<&str>, usize)) -> Shape {
        let mut rows: [u8; 3] = [0; 3];
        shape
            .0
            .iter()
            .take(3)
            .enumerate()
            .for_each(|(row, s)| rows[row] = Shape::row_from(s));

        Shape::new(rows, shape.1)
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    required_shapes: [usize; 6],
}

use good_lp::{
    constraint, solvers::coin_cbc::coin_cbc, variable, variables, Expression, SolverModel, Variable,
};

#[derive(Debug)]
struct Placement {
    piece_type: usize,
    covered_cells: Vec<usize>,
}

impl Region {
    fn new(width: usize, height: usize, required_shapes: [usize; 6]) -> Region {
        Region {
            width,
            height,
            required_shapes,
        }
    }

    fn enumerate_placements(&self, shapes: &[Shape]) -> Vec<Placement> {
        let mut out = Vec::<Placement>::new();

        for shape in shapes {
            for v in &shape.variants {
                for r0 in 0..=(self.height.saturating_sub(3)) {
                    for c0 in 0..=(self.width.saturating_sub(3)) {
                        let mut cells = Vec::new();
                        for (dy, rowbits) in v.iter().enumerate() {
                            for bit in 0..3 {
                                if (rowbits & (1 << bit)) != 0 {
                                    let x_from_left = 2 - bit;
                                    let rr = r0 + dy;
                                    let cc = c0 + x_from_left;
                                    let cid = rr * self.width + cc;
                                    cells.push(cid);
                                }
                            }
                        }
                        if !cells.is_empty() {
                            out.push(Placement {
                                piece_type: shape.id,
                                covered_cells: cells,
                            });
                        }
                    }
                }
            }
        }

        out
    }

    fn try_fit(&self, shapes: &[Shape]) -> bool {
        let placements = self.enumerate_placements(shapes);
        let mut vars = variables!();
        let x: Vec<Variable> = placements
            .iter()
            .map(|_| vars.add(variable().binary()))
            .collect();

        let cell_count = self.width * self.height;
        let mut cell_to_vars: Vec<Vec<Variable>> = vec![Vec::new(); cell_count];
        let mut type_to_vars: Vec<Vec<Variable>> = vec![Vec::new(); 6];

        for (j, p) in placements.iter().enumerate() {
            let v = x[j];
            type_to_vars[p.piece_type].push(v);
            for &cid in &p.covered_cells {
                cell_to_vars[cid].push(v);
            }
        }
        // minimise 0
        let mut model = vars.minimise(0).using(coin_cbc);

        // Overlap constraints
        for vars_here in &cell_to_vars {
            if !vars_here.is_empty() {
                let expr: Expression = vars_here.iter().copied().sum();
                model = model.with(constraint!(expr <= 1));
            }
        }
        // counts per type
        for t in 0..6 {
            let req = self.required_shapes[t] as f64;
            let expr: Expression = type_to_vars[t].iter().copied().sum();

            model = model.with(constraint!(expr == req));
        }

        model.solve().is_ok()
    }
}

impl From<&str> for Region {
    fn from(region: &str) -> Region {
        let s = region.split(":").collect::<Vec<&str>>();
        let s1 = s
            .first()
            .unwrap()
            .split("x")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let s2 = s
            .last()
            .unwrap()
            .trim()
            .split(" ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let required_shapes: [usize; 6] = s2.try_into().unwrap();

        Region::new(s1[0], s1[1], required_shapes)
    }
}

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");

    let mut regions: Vec<Region> = Vec::with_capacity(1000);
    let mut shapes: Vec<Shape> = Vec::with_capacity(6);
    let mut shape: Vec<&str> = Vec::with_capacity(3);
    let mut mode: usize = 0;

    input.lines().for_each(|x| {
        if x.is_empty() {
            mode = 0;
            return;
        }

        if x.len() == 2 {
            mode = 1;
            return;
        }

        if mode == 1 {
            shape.push(x);

            if shape.len() == 3 {
                shapes.push(Shape::from((&shape, shapes.len())));
                shape.clear();
            }
        } else {
            regions.push(Region::from(x))
        }
    });

    let mut sum = 0;

    regions
        .iter_mut()
        .for_each(|r| sum += if r.try_fit(&shapes) { 1 } else { 0 });
    Ok(sum.to_string())
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
