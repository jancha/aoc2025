use aoc::Result;
#[cfg(feature="debug")]
use colored::Colorize;
use std::cell::RefCell;
use std::{collections::VecDeque, fs};
const AREA_HEIGHT: usize = 51;
type ShapeRow = [u8; 3];
type Area = [u64; AREA_HEIGHT];

#[derive(Debug)]
pub struct Shape {
    pub rows: ShapeRow,
    pub id: usize,
    pub variation: usize,
    pub id_variation: usize,
    pub size: usize,
}

impl Shape {
    fn new(rows: [u8; 3], id: usize, variation: usize) -> Shape {
        let mut size = 0;
        for i in rows {
            for j in 0..3 {
                if i & (1 << j) > 0 {
                    size += 1;
                }
            }
        }
        Shape {
            rows,
            id,
            variation,
            id_variation: id * 10 + variation,
            size,
        }
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
    fn build_variants(&mut self) -> Vec<Shape> {
        let mut shapes = Vec::new();
        let mut rows: Vec<ShapeRow> = Vec::new();

        for _j in 0..2 {
            for _i in 0..4 {
                if !rows.contains(&self.rows) {
                    rows.push(self.rows);
                }
                self.rotate();
            }
            self.flip_vertical();
        }
        rows.iter()
            .enumerate()
            .filter(|x| x.0 != 0)
            .for_each(|(i, x)| {
                shapes.push(Shape::new(*x, self.id, i));
            });
        shapes
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
    // function to render single shape
    #[allow(dead_code)]
    #[cfg(feature="debug")]
    fn render(&self) {
        let mut buf: [char; 9] = ['.'; 9]; // 6x6 max

        for (row, u) in self.rows.iter().enumerate() {
            for i in 0..3 {
                if (*u as u64) & (1 << i) > 0 {
                    buf[row * 3 + (3 - 1 - i)] = std::char::from_digit(self.id as u32, 10).unwrap();
                }
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                let c = &buf[(3 - j - 1) + i * 3];

                let c = match c {
                    '0' => c.to_string().truecolor(255, 255, 0),
                    '1' => c.to_string().truecolor(0, 255, 0),
                    '2' => c.to_string().truecolor(255, 128, 255),
                    '3' => c.to_string().truecolor(0, 128, 255),
                    '4' => c.to_string().truecolor(128, 128, 255),
                    '5' => c.to_string().truecolor(255, 128, 0),
                    '.' => c.to_string().truecolor(120, 120, 120),
                    _ => c.to_string().truecolor(80, 80, 80),
                };

                print!("{}", c);
            }
            println!("");
        }
    }
    #[cfg(feature="debug")]
    fn render_to_buf(&self, offset_x: usize, offset_y: usize, width: usize,  buf: &mut[char] ) {
        for (row, u) in self.rows.iter().enumerate() {
            for i in 0..3 {
                if (*u as u64) & (1 << i) > 0 {
                    buf[(row + offset_y) * width + (width - 1 - i - offset_x)] = std::char::from_digit(self.id as u32, 10).unwrap();
                }
            }
        }
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

        Shape::new(rows, shape.1, 0)
    }
}

#[derive(Debug)]
struct Region<'a> {
    width: usize,
    height: usize,
    required_shapes: [usize; 6],
    put_shapes: RefCell<[usize; 6]>,
    area: RefCell<Area>,
    shapes_on_area: RefCell<VecDeque<(usize, usize, usize)>>,
    shapes: Option<&'a [Shape]>,
}

impl<'a> Region<'a> {
    fn new(width: usize, height: usize, required_shapes: [usize; 6]) -> Region<'a> {
        Region {
            width,
            height,
            required_shapes,
            put_shapes: RefCell::new([0; 6]),
            area: RefCell::new([0; AREA_HEIGHT]),
            shapes_on_area: RefCell::new(VecDeque::with_capacity(600)),
            shapes: None,
        }
    }
    fn try_fit_new(&mut self) -> bool {
        if self.put_next_shape(0,0, 0) {
            #[cfg(feature="debug")]
            {
                println!("Solution found!");
                self.render_solution("");
            }
            true
        } else {
            #[cfg(feature="debug")]
            println!("Solution not found!");
            false
        }
    }
    fn put_next_shape(&self, x: usize, y: usize, depth: usize) -> bool {
        let mut tried: Vec<usize> = Vec::with_capacity(32);
        if self.is_done(y){
            return true;
        }
        let mut x = x;
        let mut y= y;

        let tried_max_len = if self.width  * self.height <= 60 {
            6 // test requires more thorough analsis due to smaller real estate
        } else {
            2 // we are okay with best fit strategy for larger fields
        };

        loop {
            // only use best next shape, ignore others 
            while let Some(id_variation) = self.try_put_next_best_shape(x, y, &mut tried) && tried.len() < tried_max_len {
                self.put_shape_on_area(id_variation, x,y);
                if self.is_done(y){
                    return true;
                }
                if let Ok((next_x,next_y)) = self.add_xy(x, y) {
                    if self.put_next_shape(next_x, next_y, depth + 1) {
                        return true;
                    } else {
                        self.remove_last_shape_from_area();
                    }
                } else {
                    return false; // assume solution does not exist
                }
            }

            if let Ok((next_x, next_y)) = self.add_xy(x,y) {
                x = next_x;
                y = next_y;
            } else {
                return false;
            }
        }
    }
    fn add_xy(&self, x:usize, y:usize) -> std::result::Result<(usize, usize), ()> {
        let mut x = x+1;
        let mut y = y;
        if x > self.width - 3 {
            x = 0;
            y += 1;
            let mut delta = 0;
            let put_shapes = self.put_shapes.borrow();
            self.required_shapes.iter().enumerate().for_each(|x|
                {
                    delta += (x.1 - put_shapes[x.0]) * self.shapes.unwrap()[x.0].size
                });
            if y > 2 {
                let reminder = self.width * (self.height - y - 2);
                if reminder < delta {
                    // theoretically impossible to solve
                    return Err(());
                } 
            }
        }
        if y >= self.height - 2 {
            Err(())
        } else {
            Ok((x,y))
        }
    }
    fn is_done(&self, _y: usize) -> bool {
        #[cfg(not(feature="debug"))]
        {
            // shortcut - if there is enough space we can assume that we can complete the puzzle,
            // no need to actualy solve it
            let put_shapes = self.put_shapes.borrow();
            let mut delta = 0;
            self.required_shapes.iter().enumerate().for_each(|x|
                {
                    delta += (x.1 - put_shapes[x.0]) * 9;
                });
            let reminder = (self.width - self.width % 3) * ((self.height - _y - 2) - (self.height - _y - 2) % 3);
            if reminder > delta { 
                return true;
            }
        }
        // check if all shapes have been put
        for (i,j) in self.put_shapes.borrow().iter().enumerate() {
            if self.required_shapes[i] != *j {
                return false;
            }
        }
        true
    }
    fn try_put_next_best_shape(
        &self,
        x: usize,
        y: usize,
        tried: &mut Vec<usize>,
    ) -> Option<usize> {
        // pick one of the shapes that best fit the x (least empty spaces on right and top)
        let mut candidates:Vec<(Option<usize>, usize, u8)> = Vec::with_capacity(50);
        let put_shapes = self.put_shapes.borrow();
        for shape in self.shapes.unwrap().iter().filter(|shape| {
            !tried.contains(&shape.id_variation)
                && put_shapes[shape.id] < self.required_shapes[shape.id]
        }) {
            if let Ok(score) = self.fit_score(shape, &x, &y) {
                candidates.push((Some(shape.id), shape.id_variation, score as u8));
            }
        }
        candidates.sort_by(|a, b| a.2.cmp(&b.2));
        candidates.reverse();
        if let Some(next_candidate) = candidates.first() && next_candidate.0.is_some() {
            tried.push(next_candidate.1);
            Some(next_candidate.1) // return shape variation id
        } else {
            None
        }
    }

    fn remove_last_shape_from_area(&self) {
        if let Some((offset_x, offset_y, id_variation)) = self.shapes_on_area.borrow_mut().pop_back()
        {
            let shape = self.get_shape(id_variation).unwrap();
            self.put_shapes.borrow_mut()[shape.id] -= 1;
            let shape_row = shape.rows;
            let mut area = self.area.borrow_mut();
            for (row_num, row) in shape_row.iter().enumerate() {
                area[offset_y + row_num] ^= (*row as u64) << offset_x;
            }
        }
    }

    fn put_shape_on_area(
        &self,
        id_variation: usize,
        offset_x: usize,
        offset_y: usize,
    ) -> bool {
        let shape = self.get_shape(id_variation).unwrap();
        let shape_row = shape.rows;
        let mut area = self.area.borrow_mut();
        for (row_num, row) in shape_row.iter().enumerate() {
            area[offset_y + row_num] |= (*row as u64) << offset_x;
        }
        self.put_shapes.borrow_mut()[shape.id] += 1;
        self.shapes_on_area.borrow_mut()
            .push_back((offset_x, offset_y, shape.id_variation));
        true
    }

    fn get_shape(&self, id_variation: usize) -> Option<&Shape> {
        if let Some(shapes) = self.shapes {
            shapes.iter().find(|x| x.id_variation == id_variation)
        } else {
            None
        }
    }

    fn fit_score(&self, shape: &Shape, x: &usize, y: &usize) -> std::result::Result<usize, ()> {
        let mut score = 0;
        let mut row_score: u64 = 9;
        let area = self.area.borrow();
        for (row_num, row) in shape.rows.iter().enumerate() {
            let area_row = area[y + row_num];
            let mut digits_in_row = 0;
            for i in 0..3 {
                let bit = if (*row as u64) & (1 << i) > 0 { 1 } else { 0 };
                if bit == 1 {
                    if area_row & (1 << (x + i as usize)) > 0 {
                        // overlap, terminate
                        return Err(());
                    } else {
                        // the more density towards top, the higher score
                        digits_in_row += 1;
                        score += row_score * digits_in_row;
                    }
                }
            }
            row_score /= 2;
        }

        Ok(score as usize)
    }

    // function to render actual solution
    #[cfg(feature="debug")]
    fn render_solution(&self, padding: &str) {
        let mut buf:Vec<char> = vec!['.'; self.width * self.height];
        println!("{padding} Shapes: {:?}", self.shapes_on_area.borrow());
        for i in self.shapes_on_area.borrow().iter() {
            self.get_shape(i.2).unwrap().render_to_buf(i.0, i.1, self.width, &mut buf);
        }
        for i in 0..self.height {
            for j in 0..self.width {
                let c = &buf[j + i * self.width];

                let c = match c {
                    '0' => c.to_string().truecolor(255, 255, 0),
                    '1' => c.to_string().truecolor(0, 255, 0),
                    '2' => c.to_string().truecolor(255, 128, 255),
                    '3' => c.to_string().truecolor(0, 128, 255),
                    '4' => c.to_string().truecolor(128, 128, 255),
                    '5' => c.to_string().truecolor(255, 128, 0),
                    '.' => c.to_string().truecolor(120, 120, 120),
                    _ => c.to_string().truecolor(80, 80, 80),
                };

                print!("{}", c);
            }
            print!("\n");
        }
        println!("");

    }
}

impl<'a> From<&str> for Region<'a> {
    fn from(region: &str) -> Region<'a> {
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
    for i in 0..shapes.len() {
        let mut v = shapes.get_mut(i).unwrap().build_variants();
        shapes.append(&mut v);
    }
    let mut sum = 0;
    regions.iter_mut().for_each(|r| {
        r.shapes = Some(&shapes);
        sum += if r.try_fit_new() { 1 } else { 0 }
    });
    Ok(sum.to_string())
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
