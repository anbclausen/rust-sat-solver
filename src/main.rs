#[macro_use] extern crate scan_fmt;

pub mod reader;
pub mod types;
pub mod solver;

use reader::read_file_to_cnf;
use solver::solve;

fn main() {
    let cnf = read_file_to_cnf("tests/prime4.cnf");
    let result = solve(&cnf);
    print!("{:?}", result)
}

#[cfg(test)]
mod tests {
    use crate::reader::read_file_to_cnf;
    use crate::solver::solve;
    use crate::types::Res;

    #[test]
    fn trivial_should_be_satisfiable() {
        let cnf = read_file_to_cnf("tests/trivial.cnf");
        let result = solve(&cnf);
        assert!(matches!(result, Res::Satisfiable ( .. )))
    }

    #[test]
    fn false_should_be_unsatisfiable() {
        let cnf = read_file_to_cnf("tests/false.cnf");
        let result = solve(&cnf);
        assert!(matches!(result, Res::Unsatisfiable))
    }

    #[test]
    fn full4_should_be_unsatisfiable() {
        let cnf = read_file_to_cnf("tests/full4.cnf");
        let result = solve(&cnf);
        assert!(matches!(result, Res::Unsatisfiable))
    }

    #[test]
    fn ph2_should_be_unsatisfiable() {
        let cnf = read_file_to_cnf("tests/ph2.cnf");
        let result = solve(&cnf);
        assert!(matches!(result, Res::Unsatisfiable))
    }

    #[test]
    fn prime4_should_be_satisfiable() {
        let cnf = read_file_to_cnf("tests/prime4.cnf");
        let result = solve(&cnf);
        assert!(matches!(result, Res::Satisfiable ( .. )))
    }
}