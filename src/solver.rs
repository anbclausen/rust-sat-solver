use crate::types::{Assignment, Clause, Problem, Res};

fn check(assignment: &Assignment, clauses: &Vec<Clause>) -> Res {
    for clause in clauses {
        let satisfied = clause.iter().any(|&l| assignment.contains(&l));
        if !satisfied {
            return Res::Unsatisfiable;
        }
    }
    Res::Satisfiable(assignment.clone())
}

fn naive(problem: &Problem) -> Res {
    fn naive_helper(problem: &Problem, assignment: Assignment, var: i32) -> Res {
        if var > problem.num_vars {
            return Res::Unsatisfiable;
        }

        let mut positive_assignment = assignment.to_vec();
        positive_assignment.push(var);

        let mut negative_assignment = assignment.to_vec();
        negative_assignment.push(-var);

        let positive_assignment_satisfiable = match check(&positive_assignment, &problem.clauses) {
            Res::Unsatisfiable => naive_helper(problem, positive_assignment, var + 1),
            satisfiable => satisfiable,
        };

        match positive_assignment_satisfiable {
            Res::Unsatisfiable => {
                match check(&negative_assignment, &problem.clauses) {
                    Res::Unsatisfiable => naive_helper(problem, negative_assignment, var + 1),
                    satisfiable => satisfiable,
                }
            },
            satisfiable => satisfiable,
        }


    }
    let assignment = Vec::new();
    naive_helper(problem, assignment, 1)
}

pub fn solve(problem: &Problem) -> Res {
    naive(problem)
}