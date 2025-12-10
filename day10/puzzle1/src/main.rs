use aoc::Result;
use std::fs;
pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");

    // read

    let mut light_diagrams: [isize; 200] = [0; 200];

    let mut toggles: [Vec<isize>; 200] = [const { Vec::new() }; 200];

    //let mut joltage: Vec<Vec<u16>> = Vec::new();

    let mut machine_count = 0;
    input.lines().enumerate().for_each(|(line_num, line)| {
        let s = line.split_whitespace().collect::<Vec<&str>>();
        let light_diagram = s.first().unwrap();
        let light_diagram = (*light_diagram).chars();

        let light_diagram = light_diagram
            .filter(|x| *x == '.' || *x == '#')
            .enumerate()
            .map(|(pos, x)| {
                if x == '#' {
                    2_isize.pow(pos.try_into().unwrap())
                } else {
                    0
                }
            })
            .sum::<isize>();
        light_diagrams[line_num] = light_diagram;

        let mut light_toggles: Vec<isize> = Vec::new();
        s.into_iter().skip(1).for_each(|x| {
            if &x[0..1] == "(" {
                let toggle = x[1..x.len() - 1].split(",").collect::<Vec<&str>>();

                let toggle = toggle
                    .iter()
                    .map(|char| 2_isize.pow(char.parse::<usize>().unwrap().try_into().unwrap()))
                    .sum::<isize>();
                light_toggles.push(toggle);
            }
        });
        light_toggles.sort();
        toggles[line_num] = light_toggles;
        machine_count = line_num + 1;
    });

    let mut sum = 0;
    for i in 0..machine_count {
        let steps = find_min_toggle_count(&light_diagrams[i], &toggles[i]);

        sum += steps;
    }

    Ok(sum.to_string())
}

use std::collections::BTreeMap;

fn find_min_toggle_count(light_diagram: &isize, toggles: &[isize]) -> isize {
    let mut cache: BTreeMap<isize, isize> = BTreeMap::new();
    let mut merged_toggles: BTreeMap<isize, (isize, isize, isize)> = BTreeMap::new();

    for (pos, i) in toggles.iter().enumerate() {
        merged_toggles.insert(*i, (1, pos as isize, pos as isize));
    }

    let merged_toggles = expand_toggles(merged_toggles, 1);

    let mut flat_toggles: Vec<(isize, isize)> = Vec::new();
    for (val, t) in &merged_toggles {
        flat_toggles.push((*val, t.0));
    }
    flat_toggles.sort();
    flat_toggles.reverse();

    let r = find_min(0, None, *light_diagram, &flat_toggles, &mut cache);
    if r == isize::MAX {
        panic!("Could not solve");
    }
    r
}

fn find_min(
    steps_before: isize,
    prev: Option<isize>,
    remainder: isize,
    toggles: &Vec<(isize, isize)>,
    cache: &mut BTreeMap<isize, isize>,
) -> isize {
    if let Some(v) = cache.get(&remainder) {
        return *v;
    }
    let mut steps = isize::MAX;
    if steps_before > 5 {
        return steps;
    }
    for (val, toggle_steps) in toggles.iter().filter(|(val, _t)| {
        if let Some(prev) = prev {
            *val != prev
        } else {
            true
        }
    }) {
        let row_steps = if remainder == *val {
            *toggle_steps
        } else if *toggle_steps < steps {
            let delta = remainder ^ val;
            let row_steps = find_min(
                steps_before + toggle_steps,
                Some(*val),
                delta,
                toggles,
                cache,
            );
            if row_steps == isize::MAX {
                // did not find solution skip
                continue;
            }
            row_steps + toggle_steps
        } else {
            continue;
        };

        steps = steps.min(row_steps);
        if steps == 1 {
            break;
        }
    }
    cache.entry(remainder).or_insert(steps);
    steps
}

fn expand_toggles(
    toggles: BTreeMap<isize, (isize, isize, isize)>,
    base: isize,
) -> BTreeMap<isize, (isize, isize, isize)> {
    let mut new_toggle_map: BTreeMap<isize, (isize, isize, isize)> = toggles.clone();

    for (val, t1) in toggles.iter().filter(|(val, t)| t.0 == 1) {
        for (val2, t2) in toggles
            .iter()
            .filter(|(t2val, t)| t.0 == base && *t2val != val)
        {
            let d = val ^ val2;
            new_toggle_map.entry(d).or_insert((t2.0 + 1, *val, *val2));
        }
    }
    new_toggle_map
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
