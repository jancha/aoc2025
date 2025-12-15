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

    fn get_vecu64_variants(&self) -> Vec<Vec<u64>> {
        let mut v: Vec<Vec<u64>> = Vec::with_capacity(self.variants.len());
        for i in &self.variants {
            v.push(i.iter().map(|x| *x as u64).collect::<Vec<u64>>());
        }
        v
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
struct ShapeGroup {
    pub area: Vec<u64>, // data
    pub width: u64,
    pub height: u64,
    pub shapes: Vec<(usize, usize)>,
}

impl ShapeGroup {
    pub fn new() -> ShapeGroup {
        ShapeGroup {
            area: Vec::with_capacity(50),
            width: 0,
            height: 0,
            shapes: Vec::with_capacity(10),
        }
    }

    pub fn build_shape_pairs(&mut self, shapes: &[Shape]) {
        // try to arrange shape variants in a way that
        // provides most optimised space use ()

        let mut pairs: Vec<((usize, usize, usize, usize), ([u64; 50], usize, usize, u64))> =
            Vec::with_capacity(1000);

        let width = 3;
        let height = 3;

        for i in 0..shapes.len() {
            let shape = &shapes[i];
            for (shape_variant_id, shape_variant) in shape.get_vecu64_variants().iter().enumerate()
            {
                for j in i..shapes.len() {
                    let shape2 = &shapes[j];
                    for (shape_variant_id2, shape_variant2) in
                        shape2.get_vecu64_variants().iter().enumerate()
                    {
                        for x_offset in 0..=width {
                            for y_offset in 0..=height {
                                let r = ShapeGroup::try_combine(
                                    &shape_variant,
                                    &shape_variant2,
                                    x_offset,
                                    y_offset,
                                    width,
                                    height,
                                );
                                if let Ok(r) = r {
                                    pairs.push(((i, j, shape_variant_id, shape_variant_id2), r));
                                }
                            }
                        }
                    }
                }
            }
        }

        pairs.sort_by(|a, b| a.1 .3.cmp(&b.1 .3));

        for i in pairs.iter().filter(|x| x.1 .3 < 10) {
            println!("Pairs: {:?} -> {} {} {}", i.0, i.1 .1, i.1 .2, i.1 .3);
        }
    }

    pub fn try_combine(
        shape: &Vec<u64>,
        shape2: &Vec<u64>,
        offset_x: usize,
        offset_y: usize,
        limit_x: usize,
        limit_y: usize,
    ) -> std::result::Result<([u64; 50], usize, usize, u64), ()> {
        let mut area: [u64; 50] = [0; 50];

        shape.iter().enumerate().for_each(|(i, j)| area[i] = *j);

        for (row_num, row) in shape2.iter().enumerate() {
            let area_row = area[offset_y + row_num];
            if area_row & ((*row) << offset_x) > 0 {
                return Err(());
            }
        }
        for (row_num, row) in shape2.iter().enumerate() {
            area[offset_y + row_num] |= (*row) << offset_x;
        }

        let mut width = 0;
        let mut height = 0;
        let mut empty = 0;

        area.iter().enumerate().for_each(|(_i, j)| {
            let mut k = *j;
            let mut row_width = 0;

            if k > 0 {
                height += 1;
                while k > 0 {
                    row_width += 1;
                    k >>= 1;
                }
                width = width.max(row_width);
            }
        });

        for i in 0..height {
            for j in 0..width {
                if area[i] & (1 << j) == 0 {
                    empty += 1;
                }
            }
        }

        Ok((area, width, height, empty))
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    required_shapes: [usize; 6],
    put_shapes: [usize; 6],
    area: Area,
    shapes_on_area: VecDeque<(usize, usize, usize, usize)>,
}

impl Region {
    fn new(width: usize, height: usize, required_shapes: [usize; 6]) -> Region {
        Region {
            width,
            height,
            required_shapes,
            put_shapes: [0; 6],
            area: [0; AREA_HEIGHT],
            shapes_on_area: VecDeque::with_capacity(600),
        }
    }
    fn try_fit(&mut self, shapes: &[Shape]) -> bool {
        println!("Shape 4: {:?}", shapes[4]);
        self.try_put_next(shapes)
    }

    fn try_put_next(&mut self, shapes: &[Shape]) -> bool {
        let mut done = true;
        for shape_id in 0..self.required_shapes.len() {
            if self.put_shapes[shape_id] < self.required_shapes[shape_id] {
                done = false;
                for shape_variant in 0..shapes[shape_id].variants.len() {
                    for offset_y in 0..=self.height - 3 {
                        for offset_x in 0..=self.width - 3 {
                            if self.try_put_shape(
                                &shapes[shape_id],
                                shape_variant,
                                offset_x,
                                offset_y,
                            ) {
                                self.put_shapes[shape_id] += 1;
                                if self.try_put_next(shapes) {
                                    return true;
                                } else {
                                    self.try_remove_last(shapes);
                                    self.put_shapes[shape_id] -= 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        if done {
            println!("Solution: {:#?}", self.shapes_on_area.len());
            self.render();
        }
        done
    }
    fn try_remove_last(&mut self, shapes: &[Shape]) {
        if let Some((offset_x, offset_y, shape_id, shape_variant)) = self.shapes_on_area.pop_back()
        {
            let shape_row = shapes[shape_id].variants[shape_variant];

            for (row_num, row) in shape_row.iter().enumerate() {
                self.area[offset_y + row_num] ^= (*row as u64) << offset_x;
            }
        }
    }

    fn try_put_shape(
        &mut self,
        shape: &Shape,
        shape_variant: usize,
        offset_x: usize,
        offset_y: usize,
    ) -> bool {
        let shape_row = shape.variants[shape_variant];

        for (row_num, row) in shape_row.iter().enumerate() {
            let area_row = &self.area[offset_y + row_num];
            if *area_row & ((*row as u64) << offset_x) > 0 {
                return false;
            }
        }

        for (row_num, row) in shape_row.iter().enumerate() {
            self.area[offset_y + row_num] |= (*row as u64) << offset_x;
        }

        self.shapes_on_area
            .push_back((offset_x, offset_y, shape.id, shape_variant));
        true
    }
    fn render(&self) {}
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

    let mut shape_group: ShapeGroup = ShapeGroup::new();

    shape_group.build_shape_pairs(&shapes);

    /*regions
    .iter_mut()
    .for_each(|r| sum += if r.try_fit(&shapes) { 1 } else { 0 });*/
    Ok(sum.to_string())
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;

#[test]

fn test_shape_build() {
    let shape = Shape::from((&vec!["###", "##.", "##."], 0));

    println!("Shape: {:#?}", shape)
}
