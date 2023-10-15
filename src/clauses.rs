use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt;


////////////////////////////////////////////////////////////////////////////////

// Invariant: nonnegative
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Atom (u32);

impl Atom {
    pub fn new(val: u32) -> Self {
        Self (
            // Assumes normal twos complement
            if 0 <= (val as i32) {val} else {
                panic!("Value should be nonnegative");
            }
        )
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


////////////////////////////////////////////////////////////////////////////////

// A partial mapping from atoms to values
pub type Asgmt = HashMap<Atom, bool>;


////////////////////////////////////////////////////////////////////////////////

// A positive or negative atom
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
// The MSB represents the sign, and the rest represent the atom
pub struct Literal (u32);

const LITERAL_MASK : u32 = 1 << 31;

impl Literal {
    pub fn new(positive: bool, atom: Atom) -> Self {
        Self (
            if positive {atom.0} else {atom.0 | LITERAL_MASK}
        )
    }

    pub fn positive(&self) -> bool {
        (self.0 & LITERAL_MASK) == 0
    }

    pub fn atom(&self) -> Atom {
        Atom::new(self.0 & !LITERAL_MASK)
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.positive() { "" } else { "!" }, self.atom())
    }
}


////////////////////////////////////////////////////////////////////////////////

// A disjunction of literals
// TODO: add normalize and is_normal functions
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Clause {
    literals: Vec<Literal>,
}

impl Clause {
    pub fn new() -> Self {
        Self {
            literals: Vec::new(),
        }
    }

    pub fn add(&mut self, literal: Literal) {
        self.literals.push(literal)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Literal> {
        self.literals.iter()
    }

    pub fn atoms(&self) -> HashSet<Atom> {
        self.literals.iter().map(|literal| literal.atom()).collect()
    }

    // Evaluates clause when fully assigned
    pub fn eval_clause(&self, asgmt: &Asgmt) -> Option<bool> {
        // println!("Evaluating clause {} under assignment {:?}", clause, asgmt);
        for literal in self.literals.iter() {
            let pos = literal.positive();
            let val = *asgmt.get(&literal.atom())?;
            if pos == val {
                return Some(true)
            }
        }
        Some(false)
    }
}

impl IntoIterator for Clause {
    type Item = Literal;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.literals.into_iter()
    }
}

impl std::convert::From<Vec<Literal>> for Clause {
    fn from(literals: Vec<Literal>) -> Self {
        Self {literals}
    }
}

impl fmt::Display for Clause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", itertools::join(self.literals.iter(), " "))
    }
}


////////////////////////////////////////////////////////////////////////////////

// A conjunction of clauses
// TODO: add in assignment
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Cnf {
    clauses: Vec<Clause>,
}

impl Cnf {
    pub fn new() -> Self {
        Self {
            clauses: Vec::new(),
        }
    }

    pub fn add(&mut self, clause: Clause) {
        self.clauses.push(clause)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Clause> {
        self.clauses.iter()
    }

    pub fn atoms(&self) -> HashSet<Atom> {
        self.clauses.iter()
            .flat_map(|clause|
                clause.literals.iter().map(|literal| literal.atom()))
            .collect()
    }

    pub fn bound_atoms(asgmt: &Asgmt) -> HashSet<Atom> {
        asgmt.keys().cloned().collect()
    }

    pub fn free_bound_atoms_pair(&self, asgmt: &Asgmt) -> (HashSet<Atom>, HashSet<Atom>) {
        let all = self.atoms();
        let bound = Self::bound_atoms(asgmt);
        (all.difference(&bound).map(|&atom| atom).collect(), bound)
    }

    pub fn free_atoms(&self, asgmt: &Asgmt) -> HashSet<Atom> {
        self.free_bound_atoms_pair(asgmt).0
    }

    // fn get_unbound_literal(&self, asgmt: &Asgmt) -> Literal {
    //     let bound = Self::bound_atoms(asgmt);
    //     self.iter()
    //         .flat_map(|clause| clause.iter().filter(|literal| !bound.contains(&literal.atom())))
    //         .cloned()
    //         .next()
    //         .unwrap()
    // }

    // Evaluates cnf when sufficiently assigned (evaluates a fully assigned clause,
    // then true if all true, false if exists false, undefined otherwise).
    pub fn eval_cnf(&self, asgmt: &Asgmt) -> Option<bool> {
        // println!("Evaluating cnf {} under assignment {:?}", cnf, asgmt);
        let mut under_assigned = false;
        for clause in self.clauses.iter() {
            if let Some(val) = clause.eval_clause(asgmt) {
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
}

impl IntoIterator for Cnf {
    type Item = Clause;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.clauses.into_iter()
    }
}

impl std::convert::From<Vec<Clause>> for Cnf {
    fn from(clauses: Vec<Clause>) -> Self {
        Self {clauses}
    }
}

impl std::convert::From<Vec<Vec<Literal>>> for Cnf {
    fn from(clauses: Vec<Vec<Literal>>) -> Self {
        Self {
            clauses: clauses.into_iter().map(Clause::from).collect()
        }
    }
}

impl fmt::Display for Cnf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", itertools::join(self.clauses.iter(), ""))
    }
}