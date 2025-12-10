use aoc::Result;
use good_lp::{
    constraint, solvers::coin_cbc::coin_cbc, variable, variables, Expression, ProblemVariables,
    Solution, SolverModel, Variable,
};
use std::fs;

pub fn solver(file_path: &str) -> Result<String> {
    let input = fs::read_to_string(file_path).expect("Could not read input file");
    let mut toggles: Vec<Vec<Vec<isize>>> = Vec::new();
    let mut joltages: Vec<Vec<isize>> = Vec::new();
    let mut machine_count = 0;
    input.lines().enumerate().for_each(|(line_num, line)| {
        let s = line.split_whitespace().collect::<Vec<&str>>();
        let mut machine_toggles = Vec::new();
        s.into_iter().skip(1).for_each(|x| match &x[0..1] {
            "(" => {
                let toggle = x[1..x.len() - 1].split(",").collect::<Vec<&str>>();
                let toggle = toggle
                    .iter()
                    .map(|char| char.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>();
                machine_toggles.push(toggle);
            }
            "{" => {
                let joltage = x[1..x.len() - 1].split(",").collect::<Vec<&str>>();
                let joltage = joltage
                    .iter()
                    .map(|char| char.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>();
                joltages.push(joltage);
            }
            _ => {}
        });

        toggles.push(machine_toggles);
        machine_count = line_num + 1;
    });
    let mut sum = 0;
    for i in 0..machine_count {
        let steps = find_min_toggle_count_linear(&mut joltages[i], &toggles[i]);
        sum += steps;
    }
    Ok(sum.to_string())
}

fn find_min_toggle_count_linear(joltages: &mut [isize], toggles: &[Vec<isize>]) -> isize {
    // merge toggles with the joltages
    let mut constraints: Vec<Vec<isize>> = Vec::new();
    for (jt, _jv_val) in joltages.iter().enumerate() {
        let mut jvec: Vec<isize> = Vec::new(); // which toggles affect the digit
        for (toggle_id, toggle) in toggles.iter().enumerate() {
            for i in toggle {
                if jt as isize == *i {
                    jvec.push(toggle_id as isize);
                }
            }
        }
        constraints.push(jvec);
    }
    let variable_count = toggles.len();
    // create vars - every toggle is a variable
    let mut vars: ProblemVariables = variables!();
    let mut x: Vec<Variable> = Vec::with_capacity(variable_count);
    (0..variable_count).for_each(|_i| {
        let v = vars.add(variable().integer().min(0.0));
        x.push(v);
    });
    // set objective
    let mut objective: Expression = 0.0.into();
    for v in &x {
        objective += *v;
    }
    // build model with the objective
    let mut model = vars.minimise(objective).using(coin_cbc);
    model.set_parameter("logLevel", "0");
    // set constraints
    for (joltage_id, joltage) in joltages.iter().enumerate() {
        let mut expr: Expression = 0.0.into();
        for i in constraints[joltage_id].clone() {
            expr += x[i as usize];
        }
        model = model.with(constraint!(expr == *joltage as f64));
    }
    // solve
    let solution = model.solve().expect("Could not solve");
    x.iter()
        .map(|v| solution.value(*v).round() as isize)
        .sum::<isize>()
}

fn main() {
    aoc::main(solver)
}

#[cfg(test)]
mod test;
