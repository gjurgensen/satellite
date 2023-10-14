use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt;

// Variable identifiers
// Invariant: nonnegative
// TODO: rename to "atom"?
#[derive(Hash,PartialEq,Eq,Clone,Copy,Debug)]
pub struct Var {
    val: u32,
}

impl Var {
    pub fn new(val: u32) -> Self {
        Self {
            val: if 0 <= val {val} else {
                panic!("Value should be nonnegative");
            },
        }
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}


// A partial mapping from variables to values
pub type Asgmt = HashMap<Var, bool>;


// A positive or negative literal
#[derive(PartialEq,Eq,Clone,Copy,Debug)]
pub struct Literal {
    // The MSB represents the sign, and the rest represent the var
    data: u32,
}

const LITERAL_MASK : u32 = 1 << 31;

impl Literal {
    pub fn new(positive: bool, var: Var) -> Self {
        Self {
            data: if positive {var.val} else {var.val | LITERAL_MASK},
        }
    }

    pub fn positive(&self) -> bool {
        (self.data & LITERAL_MASK) == 0
    }

    pub fn var(&self) -> Var {
        Var::new(self.data & !LITERAL_MASK)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.positive() { "!" } else { "" }, self.var())
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
    println!("Evaluating clause {:?} under assignment {:?}", clause, asgmt);
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
    println!("Evaluating cnf {:?} under assignment {:?}", cnf, asgmt);
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