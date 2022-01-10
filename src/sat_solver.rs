use crate::brute_force_solver::solve;
use crate::cnf::eval_clauses;
use crate::sat_solver::Solution::Satisfiable;
use crate::sat_solver::Solution::Unsatisfiable;
use dimacs::Instance::Cnf;
use std::fs::read_to_string;

#[derive(Debug, Eq, PartialEq)]
pub enum Solution {
    Satisfiable { vars: Vec<bool> },
    Unsatisfiable {},
}

pub enum FixedVars {
    True,
    False,
    Unset,
}

pub fn solve_cnf_file(file_name: &str) -> Option<Solution> {
    println!("{}", file_name);
    match read_to_string(file_name) {
        Ok(s) => solve_cnf_string(&s),
        Err(f) => {
            panic!("{}", f.to_string())
        }
    }
}

pub fn solve_cnf_string(str: &str) -> Option<Solution> {
    let cnf = match dimacs::parse_dimacs(str) {
        Ok(c) => c,
        Err(f) => {
            panic!("{:?}", f);
        }
    };

    // println!("{:?}", cnf);
    match cnf {
        Cnf { num_vars, clauses } => {
            let solved = solve(num_vars as usize, &clauses, vec![]);
            println!("solved: {:?}", solved);
            match solved {
                Ok(m) => match m {
                    Satisfiable { vars } => {
                        println!("s {:?}", vars);
                        println!("c double check: {:?}", eval_clauses(&clauses, &vars));
                        Some(Satisfiable { vars })
                    }
                    _ => {
                        println!("s UNSATISFIABLE");
                        Some(Unsatisfiable {})
                    }
                },
                _ => {
                    panic!("");
                }
            }
        }
        _ => {
            panic!("");
        }
    }
}
