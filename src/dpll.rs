use std::collections::HashSet;

use crate::clauses;


////////////////////////////////////////////////////////////////////////////////

// Assumption: clause is normal
fn get_literal_when_unit(clause: &clauses::Clause, asgmt: &clauses::Asgmt) -> Option<clauses::Literal> {
    let mut unit : Option<clauses::Literal> = None;
    for literal in clause.iter() {
        match asgmt.get(&literal.atom()) {
            Some(val) => {
                if val == literal.positive() {
                    return None;
                }
            },
            None => match unit {
                Some(_) => return None,
                None => unit = Some(literal.clone()),
            },
        }
    };
    unit
}

fn try_find_propagate_unit(clauses: &clauses::Cnf, asgmt: &mut clauses::Asgmt) -> Option<clauses::Atom> {
    for clause in clauses.iter() {
        if let Some(literal) = get_literal_when_unit(&clause, asgmt) {
            let atom = literal.atom();
            let positive = literal.positive();
            println!("Unit propagating {}", literal);
            asgmt.insert(atom, positive);
            return Some(atom)
        }
    }
    None
}

fn unit_propagate_all(clauses: &clauses::Cnf, asgmt: &mut clauses::Asgmt) -> HashSet<clauses::Atom> {
    let mut atoms: HashSet<clauses::Atom> = HashSet::new();
    while let Some(atom) = try_find_propagate_unit(clauses, asgmt) {
        atoms.insert(atom);
    }
    atoms
}

// Assumption: atom is not bound
// Assumption: cnf is normal
fn purity(atom: clauses::Atom, clauses: &clauses::Cnf) -> Option<bool> {
    let mut occurs_pos = false;
    let mut occurs_neg = false;
    for clause in clauses.iter() {
        for literal in clause.iter() {
            if literal.atom() == atom {
                if literal.positive() {
                    if occurs_neg {
                        return None
                    }
                    occurs_pos = true;
                } else {
                    if occurs_pos {
                        return None
                    }
                    occurs_neg = true;
                }
                break;
            }
        }
    }
    if occurs_pos {
        Some(true)
    } else if occurs_neg {
        Some(false)
    } else {
        // If we want to add the assumption that atom is bound, then this should
        // instead panic (due to assumption violation).
        None
    }
}

fn try_find_eliminate_pure_literal(clauses: &clauses::Cnf, asgmt: &mut clauses::Asgmt) -> Option<clauses::Atom> {
    let free = clauses.free_atoms(asgmt);
    // println!("free: {:#?}", free);
    for atom in free {
        // println!("atom: {:#?}", atom);
        if let Some(val) = purity(atom, clauses) {
            println!("atom: {} found to have purity: {}", atom, val);
            asgmt.insert(atom, val);
            return Some(atom)
        }
    };
    None
}

fn pure_literal_elimination(clauses: &clauses::Cnf, asgmt: &mut clauses::Asgmt) -> HashSet<clauses::Atom> {
    let mut atoms: HashSet<clauses::Atom> = HashSet::new();
    while let Some(atom) = try_find_eliminate_pure_literal(clauses, asgmt) {
        atoms.insert(atom);
    }
    atoms
}


fn bool_propagate(clauses: &clauses::Cnf, asgmt: &mut clauses::Asgmt) -> HashSet<clauses::Atom> {
    let mut atoms: HashSet<clauses::Atom> = HashSet::new();
    loop {
        // TODO: remove atoms before returning None
        let new_atoms_unit = unit_propagate_all(clauses, asgmt);
        let new_atoms_pure = pure_literal_elimination(clauses, asgmt);
        if new_atoms_unit.is_empty() && new_atoms_pure.is_empty() {
            return atoms;
        }
        atoms.extend(new_atoms_unit.into_iter().chain(new_atoms_pure));
    }
}

// use clauses::Clause::IntoIterator;

// Assumption: There exists at least one literal in clauses
fn choose_literal(clauses: &clauses::Cnf, asgmt: &mut clauses::Asgmt) -> clauses::Literal {
    // This is of course the spot to try heuristics. For now, we arbitrarily
    // choose the first literal we come across.
    let bound = asgmt.atoms();
    clauses.iter()
        .flat_map(|clause| clause.iter().filter(|literal| !bound.contains(&literal.atom())))
        .cloned()
        .next()
        .unwrap()
}


// TODO: rewrite recursion into a loop
fn dpll(clauses: &clauses::Cnf, asgmt: &mut clauses::Asgmt) -> bool {
    let new_atoms = bool_propagate(clauses, asgmt);

    if let Some(val) = clauses.eval_cnf(asgmt) {
        if !val {
            for new in new_atoms {
                asgmt.remove(&new);
            }
        }
        return val
    }

    let literal = choose_literal(clauses, asgmt);
    let atom = literal.atom();
    let val = literal.positive();

    println!("Adding assumption: {}", literal);
    asgmt.insert(atom, val);
    if dpll(clauses, asgmt) {
        return true
    }
    println!("Assumption {} failed, assuming its inverse", literal);
    asgmt.insert(atom, !val);
    if dpll(clauses, asgmt) {
        return true
    }
    println!("Any assignment of {} yield UNSAT; backtracking", literal.atom());
    asgmt.remove(&atom);
    for new in new_atoms {
        asgmt.remove(&new);
    }
    false
}


pub fn sat(clauses: &clauses::Cnf) -> Option<clauses::Asgmt> {
    let mut asgmt = clauses::Asgmt::new();
    if dpll(clauses, &mut asgmt) {
        Some(asgmt)
    } else {
        None
    }
}