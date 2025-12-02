use std::env;
use std::fs;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum AOCError {
    String(String),
}
impl std::fmt::Display for AOCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AOCError {}

pub fn main<T: Fn(&str) -> Result<String>>(solver: T) {
    let args: Vec<_> = env::args().collect();

    let default_input = "input.txt";

    let input = if let Some(x) = args.iter().find(|x| x.starts_with("input")) {
        let s = x.split("=").collect::<Vec<&str>>();
        s.last().expect("please provide puzzle input").to_string()
    } else {
        default_input.to_string()
    };

    if let Some(x) = args.iter().find(|x| x.starts_with("bench")) {
        let s = x.split("=").collect::<Vec<&str>>();
        let i = s.last().unwrap().parse::<usize>().unwrap_or(100);

        bench::bench(i, || {
            solver(&input).expect("Failed to compute the result");
        });
    } else {
        let r = solver(&input).expect("Failed to compute the result");
        println!("{r}");
    }
}

pub fn test_case<T: Fn(&str) -> Result<String>>(input: &str, output: &str, solver: T) {
    assert_eq!(
        solver(input).expect("Failed to compute the result"),
        fs::read_to_string(output)
            .expect("Failed to read output file")
            .trim()
    );
}
pub fn auto_test<T: Fn(&str) -> Result<String>>(solver: T) {
    let mut cases: Vec<(String, String)> = Vec::new();
    let prefix = "input_test";
    let output_prefix = "output_test";

    let mut tests: Vec<String> = Vec::new();
    for i in std::fs::read_dir(".").unwrap().flatten() {
        let path = i.path();
        if path.is_file() {
            let path_str = path.display().to_string();
            if path_str.contains(prefix) {
                tests.push(path_str);
            }
        }
    }
    tests.sort();

    for path_str in tests {
        let s = path_str.split(prefix).collect::<Vec<&str>>();

        let case = s.last().unwrap();
        cases.push((
            format!("{prefix}{}", &case),
            format!("{output_prefix}{}", case),
        ));
    }

    for (i, o) in cases {
        println!("Testing {i}");
        test_case(&i, &o, &solver);
    }
}
