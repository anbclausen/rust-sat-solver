#[macro_use] extern crate scan_fmt;

pub mod reader;
pub mod types;
pub mod solver;

use reader::read_file_to_cnf;
use solver::naive;

fn main() {
    let cnf = read_file_to_cnf("tests/prime4.cnf");
    let result = naive(&cnf);
    print!("{:?}", result)
}
