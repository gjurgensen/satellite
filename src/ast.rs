use core::panic;
use std::collections::{HashMap, HashSet};
use std::fmt;

use crate::util;


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

// A partial mapping from atoms to phases
#[derive(PartialEq, Eq, Clone, Debug)]

// TODO: if we ever need to regularly inspect which variables are free, Asgmt
// should maintain a list which it modifies on each insert/remove.
pub struct Asgmt (HashMap<Atom, bool>);

impl Asgmt {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, atom: &Atom) -> Option<bool> {
        self.0.get(atom).map(|&b| b)
    }

    pub fn insert(&mut self, atom: Atom, phase: bool) -> Option<bool> {
        self.0.insert(atom, phase)
    }

    pub fn remove(&mut self, atom: &Atom) -> Option<bool> {
        self.0.remove(atom)
    }

    pub fn atoms(&self) -> HashSet<Atom> {
        self.0.keys().cloned().collect()
    }
}

impl fmt::Display for Asgmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}}}", itertools::join(self.0.iter().map(|(&atom, &pos)| Literal::new(pos, atom)), ", "))
    }
}


////////////////////////////////////////////////////////////////////////////////

// A positive or negative atom
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
// The MSB represents the sign, and the rest represent the atom
pub struct Literal (u32);

const LITERAL_MASK : u32 = 1 << 31;

impl Literal {
    pub fn new(phase: bool, atom: Atom) -> Self {
        Self (
            if phase {atom.0} else {atom.0 | LITERAL_MASK}
        )
    }

    pub fn phase(&self) -> bool {
        (self.0 & LITERAL_MASK) == 0
    }

    pub fn atom(&self) -> Atom {
        Atom::new(self.0 & !LITERAL_MASK)
    }

    pub fn inversion(&self) -> Self {
        Self::new(!self.phase(), self.atom())
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.phase() { "" } else { "!" }, self.atom())
    }
}


////////////////////////////////////////////////////////////////////////////////

// A disjunction of literals
// TODO: add normalize and is_normal functions
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Clause {
    literals: Vec<Literal>,
}

impl Clause {
    pub fn new() -> Self {
        Self {
            literals: Vec::new(),
        }
    }

    // pub fn shrink_to_fit(&mut self) {
    //     self.literals.shrink_to_fit()
    // }

    pub fn add(&mut self, literal: Literal) {
        self.literals.push(literal)
    }

    pub fn literals(&self) -> impl Iterator<Item = &Literal> {
        self.literals.iter()
    }

    pub fn unassigned_literals<'a, 'b, 'c>(&'a self, asgmt: &'b Asgmt) -> impl Iterator<Item = &'c Literal>
        where 'a: 'c, 'b: 'c
    {
        self.literals()
            .filter(|lit| asgmt.get(&lit.atom()).is_none())
    }

    // - Removes duplicate literals.
    // - Returns Err if a clause is trivially unsatisfiable (empty, or includes
    //   two literals of the same atom with a different phase).
    // - If trivial unit clause, returns the literal
    // TODO: shrink here, remove public interface?
    pub fn normalize(&mut self) -> Result<Option<Literal>, ()> {
        let asgmt = util::fold_option(self.literals.iter(), Asgmt::new(), |mut asgmt, literal|
            match asgmt.get(&literal.atom()) {
                Some(phase) => {
                    if phase == literal.phase() {
                        Some(asgmt)
                    } else {
                        None
                    }
                },
                None => {
                    asgmt.insert(literal.atom(), literal.phase());
                    Some(asgmt)
                },
            }
        ).ok_or(())?;
        self.literals = asgmt.0.into_iter().map(|(atom, phase)| Literal::new(phase, atom)).collect();
        self.literals.shrink_to_fit();
        let len = self.literals.len();
        match len {
            0 => Err(()),
            1 => Ok(Some(self.literals.get(0).unwrap().clone())),
            _ => Ok(None)
        }
    }

    // Evaluates clause when fully assigned
    pub fn eval(&self, asgmt: &Asgmt) -> Option<bool> {
        // println!("Evaluating clause {} under assignment {:?}", clause, asgmt);
        for literal in self.literals.iter() {
            let pos = literal.phase();
            let phase = asgmt.get(&literal.atom())?;
            if pos == phase {
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
    fn from(mut literals: Vec<Literal>) -> Self {
        literals.shrink_to_fit();
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
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Cnf {
    clauses: Vec<Clause>,
    atoms: HashSet<Atom>,
}

impl Cnf {
    pub fn new() -> Self {
        Self {
            clauses: Vec::new(),
            atoms: HashSet::new()
        }
    }

    pub fn shrink_to_fit(&mut self) {
        self.clauses.shrink_to_fit();
        self.atoms.shrink_to_fit()
    }

    pub fn add(&mut self, clause: Clause) {
        self.atoms.extend(clause.literals().map(|literal| literal.atom()));
        self.clauses.push(clause)
    }

    pub fn clauses(&self) -> std::slice::Iter<'_, Clause> {
        self.clauses.iter()
    }

    pub fn atoms<'a>(&'a self) -> impl Iterator<Item = Atom> + 'a {
        self.atoms.iter().map(|&atom| atom)
    }

    pub fn free_bound_atoms_pair(&self, asgmt: &Asgmt) -> (HashSet<Atom>, HashSet<Atom>) {
        let bound = asgmt.atoms();
        let free = self.atoms().filter(|atom| !bound.contains(atom)).collect();
        (free, bound)
    }

    pub fn free_atoms(&self, asgmt: &Asgmt) -> HashSet<Atom> {
        self.free_bound_atoms_pair(asgmt).0
    }

    // - Removes duplicate literals in clauses.
    // - Returns None if a clause is trivially unsatisfiable (empty, or includes
    //   two literals of the same atom with a different phase).
    // - Removes trivial unit clauses and returns their value in an initial assignment
    pub fn normalize(&mut self) -> Option<Asgmt> {
        let (asgmt, clauses) = util::fold_option(std::mem::take(&mut self.clauses).into_iter(), (Asgmt::new(), Vec::new()),
            |(mut asgmt, mut vec), mut clause| {
                match clause.normalize() {
                    Ok(Some(literal)) => {
                        // asgmt.insert(literal.atom(), literal.phase());
                        match asgmt.get(&literal.atom()) {
                            Some(phase) => {
                                if phase != literal.phase() {
                                    return None
                                }
                            },
                            None => {
                                asgmt.insert(literal.atom(), literal.phase());
                            },
                        };
                        Some((asgmt, vec))
                    },
                    Ok(None) => {
                        vec.push(clause);
                        Some((asgmt, vec))
                    },
                    Err(()) => None,
                }
            }
        )?;
        self.clauses = clauses;
        Some(asgmt)
    }

    // Evaluates cnf when sufficiently assigned (evaluates a fully assigned clause,
    // then true if all true, false if exists false, undefined otherwise).
    pub fn eval(&self, asgmt: &Asgmt) -> Option<bool> {
        let mut under_assigned = false;
        for clause in self.clauses.iter() {
            if let Some(val) = clause.eval(asgmt) {
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
    fn from(mut clauses: Vec<Clause>) -> Self {
        clauses.shrink_to_fit();
        let atoms = clauses.iter()
            .flat_map(|clause| clause.literals.iter().map(|literal| literal.atom()))
            .collect();
        Self {clauses, atoms}
    }
}

impl std::convert::From<Vec<Vec<Literal>>> for Cnf {
    fn from(clauses: Vec<Vec<Literal>>) -> Self {
        let clauses: Vec<Clause> = clauses.into_iter().map(Clause::from).collect();
        Self::from(clauses)
    }
}

impl fmt::Display for Cnf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", itertools::join(self.clauses.iter(), ""))
    }
}