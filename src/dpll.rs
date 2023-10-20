use std::collections::HashSet;

use crate::ast;


////////////////////////////////////////////////////////////////////////////////

// Assumption: clause is normal
fn get_literal_when_unit(clause: &ast::Clause, asgmt: &ast::Asgmt) -> Option<ast::Literal> {
    let mut unit : Option<ast::Literal> = None;
    for literal in clause.iter() {
        match asgmt.get(&literal.atom()) {
            Some(phase) => {
                if phase == literal.phase() {
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

fn try_find_propagate_unit(clauses: &ast::Cnf, asgmt: &mut ast::Asgmt, log: bool) -> Option<ast::Atom> {
    for clause in clauses.iter() {
        if let Some(literal) = get_literal_when_unit(&clause, asgmt) {
            let atom = literal.atom();
            let phase = literal.phase();
            if log {
                println!("Unit propagating {}", literal);
            };
            asgmt.insert(atom, phase);
            return Some(atom)
        }
    }
    None
}

fn unit_propagate_all(clauses: &ast::Cnf, asgmt: &mut ast::Asgmt, log: bool) -> HashSet<ast::Atom> {
    let mut atoms: HashSet<ast::Atom> = HashSet::new();
    while let Some(atom) = try_find_propagate_unit(clauses, asgmt, log) {
        atoms.insert(atom);
    }
    atoms
}

// Assumption: atom is not bound
// Assumption: cnf is normal
fn purity(atom: ast::Atom, clauses: &ast::Cnf) -> Option<bool> {
    let mut occurs_pos = false;
    let mut occurs_neg = false;
    for clause in clauses.iter() {
        for literal in clause.iter() {
            if literal.atom() == atom {
                if literal.phase() {
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

fn try_find_eliminate_pure_literal(clauses: &ast::Cnf, asgmt: &mut ast::Asgmt, log: bool) -> Option<ast::Atom> {
    let free = clauses.free_atoms(asgmt);
    // println!("free: {:#?}", free);
    for atom in free {
        // println!("atom: {}", atom);
        if let Some(phase) = purity(atom, clauses) {
            if log {
                println!("atom: {} found to have purity: {}", atom, phase);
            };
            asgmt.insert(atom, phase);
            return Some(atom)
        }
    };
    None
}

fn pure_literal_elimination(clauses: &ast::Cnf, asgmt: &mut ast::Asgmt, log: bool) -> HashSet<ast::Atom> {
    let mut atoms: HashSet<ast::Atom> = HashSet::new();
    while let Some(atom) = try_find_eliminate_pure_literal(clauses, asgmt, log) {
        atoms.insert(atom);
    }
    atoms
}


fn bool_propagate(clauses: &ast::Cnf, asgmt: &mut ast::Asgmt, log: bool) -> HashSet<ast::Atom> {
    let mut atoms: HashSet<ast::Atom> = HashSet::new();
    loop {
        // TODO: remove atoms before returning None
        let new_atoms_unit = unit_propagate_all(clauses, asgmt, log);
        let new_atoms_pure = pure_literal_elimination(clauses, asgmt, log);
        if new_atoms_unit.is_empty() && new_atoms_pure.is_empty() {
            return atoms;
        }
        atoms.extend(new_atoms_unit.into_iter().chain(new_atoms_pure));
    }
}


// Assumption: There exists at least one literal in clauses
fn choose_literal(clauses: &ast::Cnf, asgmt: &mut ast::Asgmt) -> ast::Literal {
    // This is of course the spot to try heuristics. For now, we arbitrarily
    // choose the first literal we come across.
    let bound = asgmt.atoms();
    clauses.iter()
        .flat_map(|clause| clause.iter().filter(|literal| !bound.contains(&literal.atom())))
        .cloned()
        .next()
        .unwrap()
}


fn dpll(clauses: &ast::Cnf, asgmt: &mut ast::Asgmt, log: bool) -> bool {
    let mut stack: Vec<(ast::Literal, HashSet<ast::Atom>)> = Vec::new();

    bool_propagate(clauses, asgmt, log);

    loop {
        if let Some(phase) = clauses.eval_cnf(asgmt) {
            if phase {
                return true
            };
            match stack.pop() {
                None => return false,
                Some((assumed, consquences)) => {
                    if log {
                        println!("Assumption {} failed, assuming its inverse", assumed);
                    }
                    // Note: there is not a good way to print when an assumption
                    // fails in both directions, as this is implicit; the second
                    // assignment is treated as a consquence like the propagate
                    // variables.
                    for new in consquences {
                        asgmt.remove(&new);
                    };
                    let assumed_atom = assumed.atom();
                    asgmt.insert(assumed_atom, !assumed.phase());
                    if let Some((_, prev_consequences)) = stack.last_mut() {
                        prev_consequences.insert(assumed_atom);
                        prev_consequences.extend(bool_propagate(clauses, asgmt, log));
                    };
                    continue
                },
            }
        };

        let literal = choose_literal(clauses, asgmt);
        let atom = literal.atom();
        let phase = literal.phase();

        if log {
            println!("Adding assumption: {}", literal);
        };
        asgmt.insert(atom, phase);
        stack.push((literal, bool_propagate(clauses, asgmt, log)));
    }
}


pub fn sat(clauses: &ast::Cnf, log: bool) -> Option<ast::Asgmt> {
    let mut asgmt = ast::Asgmt::new();
    if dpll(clauses, &mut asgmt, log) {
        Some(asgmt)
    } else {
        None
    }
}