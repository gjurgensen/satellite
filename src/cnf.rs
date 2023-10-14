use std::collections::{HashMap, HashSet};

// Variable identifiers
// Invariant: nonnegative
// TODO: rename to "atom"?
#[derive(Hash,PartialEq,Eq,Clone,Copy)]
pub struct Var {
    val: i32,
}

impl Var {
    pub fn new(val: i32) -> Self {
        Self {
            val: if val > 0 {val} else {
                // Panic/warn?
                (-1)*val
            },
        }
    }
}

// A partial mapping from variables to values
pub type Asgmt = HashMap<Var, bool>;


// A positive or negative literal
#[derive(PartialEq,Eq,Clone,Copy)]
pub struct Literal {
    data: i32,
}

impl Literal {
    pub fn new(positive: bool, var: Var) -> Self {
        Self {
            data: if positive {var.val} else {(-1)*var.val},
        }
    }

    pub fn positive(&self) -> bool {
        self.data > 0
    }

    pub fn var(&self) -> Var {
        Var::new(self.data.abs())
    }
}


// A disjunction of literals
// TODO: add normalize and is_normal functions
pub type Clause = Vec<Literal>;

// A conjunction of clauses
pub type Cnf = Vec<Clause>;


////////////////////////////////////////////////////////////////////////////////

pub fn vars_in_clause(clause: &Clause) -> HashSet<Var> {
    clause.iter().map(|literal| literal.var()).collect()
}


pub fn all_vars_in_cnf(cnf: &Cnf) -> HashSet<Var> {
    cnf.iter()
        .flat_map(|clause| clause.iter().map(|literal| literal.var()))
        .collect()
}

pub fn bound_vars(asgmt: &Asgmt) -> HashSet<Var> {
    asgmt.keys().cloned().collect()
}

pub fn free_bound_vars_pair(cnf: &Cnf, asgmt: &Asgmt) -> (HashSet<Var>, HashSet<Var>) {
    let all = all_vars_in_cnf(cnf);
    let bound = bound_vars(asgmt);
    (all.difference(&bound).map(|&var| var).collect(), bound)
}

pub fn free_vars(cnf: &Cnf, asgmt: &Asgmt) -> HashSet<Var> {
    free_bound_vars_pair(cnf, asgmt).0
}


////////////////////////////////////////////////////////////////////////////////

// Evaluates clause when fully assigned
pub fn eval_clause(clause: &Clause, asgmt: &Asgmt) -> Option<bool> {
    for literal in clause {
        let pos = literal.positive();
        let val = *asgmt.get(&literal.var())?;
        if pos == val {
            return Some(true)
        }
    }
    Some(false)
}


// Evaluates cnf when sufficiently assigned (evaluates a fully assigned clause,
// then true if all true, false if exists false, undefined otherwise).
pub fn eval_cnf(cnf: &Cnf, asgmt: &Asgmt) -> Option<bool> {
    let mut under_assigned = false;
    for clause in cnf {
        if let Some(val) = eval_clause(clause, asgmt) {
            if !val {
                return Some(false)
            }
        } else {
            under_assigned = true;
        }
    }
    if under_assigned {
        None
    } else {
        Some(true)
    }
}