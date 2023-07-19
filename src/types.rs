/// A literal is a number that represents a variable or its negation.
pub type Literal = i32;

/// A clause is a disjunction of literals.
pub type Clause = Vec<Literal>;

/// A problem is a conjunction of clauses.
#[derive(Debug)]
pub struct Problem {
    pub num_vars: i32,
    pub clauses: Vec<Clause>,
}

/// An assignment is a set of literals or their negation.
pub type Assignment = Vec<Literal>;

#[derive(Debug)]
pub enum Res {
    Satisfiable(Assignment),
    Unsatisfiable,
}