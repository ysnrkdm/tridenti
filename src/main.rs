use getopts::Matches;
use getopts::Options;
use std::env;
use tridenti::sat_solver::solve_cnf_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("i", "input", "Input file", "FILE");

    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    let file_name = match matches.opt_str("input") {
        Some(s) => s,
        None => {
            panic!("Input file must be supplied!");
        }
    };

    println!("{:?}", file_name);

    let solved = solve_cnf_file(&file_name);
    println!("{:?}", solved);
}
