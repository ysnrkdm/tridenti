use crate::eval::eval_clauses;
use crate::sat_solver::FixedVars;
use crate::sat_solver::Solution;
use crate::sat_solver::Solution::Satisfiable;
use crate::sat_solver::Solution::Unsatisfiable;
use dimacs::Clause;
use std::error::Error;

pub fn solve(
    num_vars: usize,
    clauses: &Box<[Clause]>,
    _fixed_vars: Vec<FixedVars>,
) -> Result<Solution, Box<(dyn Error)>> {
    dfs(num_vars, clauses, vec![])
}

fn dfs(
    num_vars: usize,
    clauses: &Box<[Clause]>,
    sofar: Vec<bool>,
) -> Result<Solution, Box<(dyn Error)>> {
    if num_vars == sofar.len() {
        // Done, eval
        let res = eval_clauses(clauses, &sofar);
        // println!("===== {:?} {:?}", sofar, res);
        if res {
            return Ok(Satisfiable { vars: sofar });
        } else {
            return Ok(Unsatisfiable {});
        }
    }

    let mut true_a = sofar.clone();
    true_a.extend_from_slice(&[true]);
    match dfs(num_vars, clauses, true_a) {
        Ok(r) => match r {
            Satisfiable { vars } => {
                return Ok(Satisfiable { vars });
            }
            _ => {}
        },
        _ => {}
    };

    let mut false_a = sofar.clone();
    false_a.extend_from_slice(&[false]);
    match dfs(num_vars, clauses, false_a) {
        Ok(r) => Ok(r),
        Err(f) => Err(f),
    }
}
