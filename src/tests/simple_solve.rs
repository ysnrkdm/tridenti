use crate::sat_solver::solve_cnf_file;
use crate::sat_solver::solve_cnf_string;
use crate::sat_solver::Solution;
use crate::tests::simple_solve::Solution::Satisfiable;
use crate::tests::simple_solve::Solution::Unsatisfiable;

#[test]
fn test_simple_solve() {
    // s UNSATISFIABLE
    assert_eq!(
        Unsatisfiable {},
        solve_cnf_string(
            r#"c This is example of unsatisifiable form
c see. https://en.wikipedia.org/wiki/DPLL_algorithm
c Author rkx1209
p cnf 3 4
1 2 -3 0
-2 1 0
-1 0
3 2 0
    "#,
        )
        .unwrap()
    );

    // s SATISFIABLE
    // v 1 2 3 -4 5 0
    assert_eq!(
        Satisfiable {
            vars: vec![true, true, true, false, true]
        },
        solve_cnf_string(
            r#"c This is simple CNF example
c Author rkx1209
p cnf 5 3
1 -5 4 0
-1 5 3 4 0
-3 -4 0
    "#,
        )
        .unwrap()
    );

    // s SATISFIABLE
    // v 1 -2 3 -4 5 6 0
    assert_eq!(
        Satisfiable {
            vars: vec![true, true, false, true, false, true]
        },
        solve_cnf_string(
            r#"c This is example of satisifiable form
c see. https://en.wikipedia.org/wiki/DPLL_algorithm
c Author rkx1209
p cnf 6 5
4 5 0
-4 -5 0
1 6 0
-1 6 0
-2 -3 0
    "#,
        )
        .unwrap()
    );
}

#[test]
fn test_file_solve() {
    assert_eq!(
        Unsatisfiable {},
        solve_cnf_file("resources/problems/ph2.cnf").unwrap()
    );

    assert_eq!(
        Unsatisfiable {},
        solve_cnf_file("resources/problems/ph3.cnf").unwrap()
    );

    assert_eq!(
        Unsatisfiable {},
        solve_cnf_file("resources/problems/diamond1.cnf").unwrap()
    );

    assert_eq!(
        Unsatisfiable {},
        solve_cnf_file("resources/problems/diamond2.cnf").unwrap()
    );

    assert_eq!(
        Unsatisfiable {},
        solve_cnf_file("resources/problems/diamond3.cnf").unwrap()
    );
}

#[test]
fn test_file_solve_medium() {
    assert_eq!(
        Satisfiable {
            vars: vec![
                true, false, true, false, false, false, true, false, false, false, false, true,
                false, true, false, false, true, true, true, true
            ]
        },
        solve_cnf_file("resources/problems/prime4.cnf").unwrap()
    );
}

// #[test]
// fn test_file_solve_large() {
//     assert_eq!(
//         Unsatisfiable {},
//         solve_cnf_file("resources/problems/add4.cnf").unwrap()
//     );
// }
