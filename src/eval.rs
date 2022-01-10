use dimacs::Clause;

pub fn eval_clauses(clauses: &Box<[Clause]>, vars: &Vec<bool>) -> bool {
    let mut ret = true;
    for clause in clauses.iter() {
        let mut clause_ret = false;
        for lit in clause.lits().iter() {
            let var_i = (lit.var().to_u64() - 1) as usize;
            clause_ret |= match lit.sign() {
                dimacs::Sign::Pos => vars[var_i],
                dimacs::Sign::Neg => !vars[var_i],
            };
        }
        ret &= clause_ret;
    }
    return ret;
}

pub fn eval_clauses_partial(clauses: &Box<[Clause]>, vars: &Vec<bool>) -> bool {
    let mut ret = true;
    for clause in clauses.iter() {
        let mut clause_ret = false;
        for lit in clause.lits().iter() {
            let var_i = (lit.var().to_u64() - 1) as usize;
            clause_ret |= match lit.sign() {
                dimacs::Sign::Pos => vars[var_i],
                dimacs::Sign::Neg => !vars[var_i],
            };
        }
        ret &= clause_ret;
    }
    return ret;
}
