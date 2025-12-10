use aoc::Result;
use std::fs;

type Point = (isize, isize);
type Line = ((isize, isize), (isize, isize));

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut vertices: Vec<Point> = Vec::new();
    let mut outer_perimeter: Vec<Line> = Vec::new();
    let mut width = 0;
    let mut height = 0;
    input.lines().for_each(|line| {
        let mut line = line.split(",");
        let x = line.next().unwrap().parse::<isize>().unwrap();
        let y = line.next().unwrap().parse::<isize>().unwrap();

        width = width.max(x);
        height = height.max(y);
        vertices.push((x, y));
    });
    width += 1;
    height += 1;
    let mut first_p: Option<Point> = None;
    let mut prev_p: Option<Point> = None;
    for p in &vertices {
        if let Some(prev_p) = prev_p {
            outer_perimeter.push((prev_p, *p));
        } else {
            first_p = Some(*p);
        }
        prev_p = Some(*p);
    }
    outer_perimeter.push((prev_p.unwrap(), first_p.unwrap()));
    let voids = flood_fill(&outer_perimeter, width, height);
    vertices.sort();
    Ok(find_max(&voids, &vertices).to_string())
}

fn flood_fill(outer_perimeter: &Vec<Line>, width: isize, height: isize) -> Vec<(Point, Point)> {
    let mut buf: Vec<char> = vec!['.'; (width * height) as usize];
    let mut voids: Vec<(Point, Point)> = Vec::new();
    for line in outer_perimeter {
        if line.0 .0 == line.1 .0 {
            let s = line.0 .1.min(line.1 .1);
            let e = line.0 .1.max(line.1 .1);
            for i in s..=e {
                let index = i * width + line.0 .0;
                buf[index as usize] = '@';
            }
            buf[(s * width + line.0 .0) as usize] = '#';
            buf[(e * width + line.1 .0) as usize] = '#';
        } else {
            let s = line.0 .0.min(line.1 .0);
            let e = line.0 .0.max(line.1 .0);
            for i in s..=e {
                let index = line.0 .1 * width + i;
                buf[index as usize] = '@';
            }
            buf[(line.0 .1 * width + s) as usize] = '#';
            buf[(line.0 .1 * width + e) as usize] = '#';
        }
    }
    let mut found = false;
    let limit = width as usize * height as usize;
    let mut void_start: Point = (0, 0);
    for i in 0..limit {
        if i % width as usize == 0 && i > 0 {
            found = false;
            if void_start != (-1, -1) {
                voids.push((
                    void_start,
                    ((i - 1) as isize % width, (i - 1) as isize / width),
                ));
            }
            void_start = (0, i as isize / width);
        }
        let c = buf[i];
        if c == '#' || c == '@' {
            if found {
                if i < limit - 1 && buf[i + 1] == '.' {
                    found = false;
                }
            } else {
                if void_start != (-1, -1) {
                    voids.push((
                        void_start,
                        ((i as isize - 1) % width, (i as isize - 1) / width),
                    ));
                    void_start = (-1, -1);
                }
                if i < limit - 1 && buf[i + 1] != '.' {
                    continue;
                } else {
                    found = true;
                }
            }
        } else if !found && void_start == (-1, -1) {
            void_start = ((i as isize) % width, (i as isize) / width);
        }
    }
    if void_start != (-1, -1) {
        voids.push((
            void_start,
            ((limit - 1) as isize % width, (limit - 1) as isize / width),
        ));
    }
    voids
}

fn find_max(voids: &Vec<(Point, Point)>, vertices: &Vec<Point>) -> usize {
    let len = vertices.len();

    let mut candidates: Vec<(Point, Point, isize)> = Vec::new();
    for i in 0..len - 1 {
        for j in i + 1..len {
            let p1 = vertices[i];
            let p2 = vertices[j];
            if p1.0 == p2.0 || p1.1 == p2.1 {
                continue;
            }
            let s = (((p2.1 - p1.1).abs() + 1) * ((p2.0 - p1.0).abs() + 1)).abs();
            candidates.push((p1, p2, s));
        }
    }
    candidates.sort_by_key(|(_p1, _p2, s)| *s);
    candidates.reverse();
    'outer: for (p1, p2, s) in candidates {
        // check one from above, one from bottom

        let mut dx = p2.0 - p1.0;
        let mut dy = p2.1 - p1.1;

        let x0 = if dx > 0 {
            p1.0
        } else {
            dx *= -1;
            p2.0
        };
        let y0 = if dy > 0 {
            p1.1
        } else {
            dy *= -1;
            p2.1
        };
        let lines: Vec<Line> = vec![
            ((x0, y0), (x0 + dx, y0)),
            ((x0, y0), (x0, y0 + dy)),
            ((x0, y0 + dy), (x0 + dx, y0 + dy)),
            ((x0 + dx, y0), (x0 + dx, y0 + dy)),
        ];
        for (l0, l1) in lines {
            for (v0, v1) in voids {
                // horizontal
                if l0.1 == l1.1 {
                    if l0.1 != v0.1 {
                        // not crossing
                        continue;
                    } else {
                        if l0.0 >= v0.0 && l0.0 <= v1.0 {
                            continue 'outer;
                        }
                        if l1.0 >= v0.0 && l1.0 <= v1.0 {
                            continue 'outer;
                        }
                    }
                }
                // vertical
                if l0.0 == l1.0 && l0.0 >= v0.0 && l0.0 <= v1.0 && v0.1 >= l0.1 && v0.1 <= l1.1 {
                    continue 'outer;
                }
            }
        }
        return s as usize;
    }
    0
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
