use crate::types::{Literal, Clause, Problem};
use std::fs::read_to_string;

fn read_file_to_string(path: &str) -> String {
    read_to_string(path).unwrap_or_else(|_| panic!("Could not read file '{path}'."))
}

pub fn read_file_to_cnf(path: &str) -> Problem {
    let contents = read_file_to_string(path);
    let lines: Vec<&str> = contents.lines().collect();
    let header = lines[0];
    let (num_vars, _) = scan_fmt!(header, "p cnf {} {}", Literal, usize)
        .unwrap_or_else(|_| panic!("Could not parse header of file '{path}'."));

    let clauses = lines[1..].iter().map(|line| {
        line.split_whitespace()
            .map(|s| s.parse::<Literal>().unwrap())
            .take_while(|&n| n != 0)
            .collect::<Clause>()
    }).collect::<Vec<Clause>>();

    Problem { num_vars, clauses }
}
