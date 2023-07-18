#[macro_use] extern crate scan_fmt;

pub mod reader;
pub mod types;

use reader::read_file_to_cnf;

fn main() {
    let cnf = read_file_to_cnf("tests/add4.cnf");
    println!("{:?}", cnf);
}
