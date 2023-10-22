use std::collections::{HashSet, HashMap};

use crate::ast;


////////////////////////////////////////////////////////////////////////////////

enum EvalResult {
    Sat,
    Unsat,
    Unknown,
}

// Assumption: clause is normal
fn get_literal_when_unit(clause: &ast::Clause, asgmt: &ast::Asgmt) -> Result<ast::Literal, EvalResult> {
    let mut unit : Option<ast::Literal> = None;
    for literal in clause.iter() {
        match asgmt.get(&literal.atom()) {
            Some(phase) => {
                if phase == literal.phase() {
                    return Err(EvalResult::Sat);
                }
            },
            None => match unit {
                Some(_) => return Err(EvalResult::Unknown),
                None => unit = Some(literal.clone()),
            },
        }
    };
    unit.ok_or(EvalResult::Unsat)
}

fn try_find_propagate_unit(cnf: &ast::Cnf, asgmt: &mut ast::Asgmt, verbosity: usize) -> Option<ast::Atom> {
    for clause in cnf.iter() {
        if let Ok(literal) = get_literal_when_unit(&clause, asgmt) {
            let atom = literal.atom();
            let phase = literal.phase();
            if 0 < verbosity {
                log::info!("Unit propagating {}", literal);
            };
            asgmt.insert(atom, phase);
            return Some(atom)
        }
    }
    None
}

fn unit_propagate_all(cnf: &ast::Cnf, asgmt: &mut ast::Asgmt, verbosity: usize) -> HashSet<ast::Atom> {
    let mut atoms: HashSet<ast::Atom> = HashSet::new();
    while let Some(atom) = try_find_propagate_unit(cnf, asgmt, verbosity) {
        atoms.insert(atom);
    }
    atoms
}

// Assumption: atom is not bound
// Assumption: cnf is normal
fn purity(atom: ast::Atom, cnf: &ast::Cnf) -> Option<bool> {
    let mut occurs_pos = false;
    let mut occurs_neg = false;
    for clause in cnf.iter() {
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

fn try_find_eliminate_pure_literal(cnf: &ast::Cnf, asgmt: &mut ast::Asgmt, verbosity: usize) -> Option<ast::Atom> {
    let free = cnf.free_atoms(asgmt);
    for atom in free {
        if let Some(phase) = purity(atom, cnf) {
            if 0 < verbosity {
                log::info!("Atom: {} found to have purity: {}", atom, phase);
            };
            asgmt.insert(atom, phase);
            return Some(atom)
        }
    };
    None
}

fn pure_literal_elimination(cnf: &ast::Cnf, asgmt: &mut ast::Asgmt, verbosity: usize) -> HashSet<ast::Atom> {
    let mut atoms: HashSet<ast::Atom> = HashSet::new();
    while let Some(atom) = try_find_eliminate_pure_literal(cnf, asgmt, verbosity) {
        atoms.insert(atom);
    }
    atoms
}


fn bool_propagate(cnf: &ast::Cnf, asgmt: &mut ast::Asgmt, verbosity: usize) -> HashSet<ast::Atom> {
    unit_propagate_all(cnf, asgmt, verbosity)
}

fn preprocess(cnf: &ast::Cnf, asgmt: &mut ast::Asgmt, verbosity: usize) {
    pure_literal_elimination(cnf, asgmt, verbosity);
}

// Assumption: There exists at least one literal in cnf
fn choose_literal(cnf: &ast::Cnf, asgmt: &mut ast::Asgmt) -> ast::Literal {
    // This is of course the spot to try heuristics. For now, we arbitrarily
    // choose the first literal we come across.
    let bound = asgmt.atoms();
    cnf.iter()
        .flat_map(|clause| clause.iter().filter(|literal| !bound.contains(&literal.atom())))
        .cloned()
        .next()
        .unwrap()
}

// Assumption: clause has at least one unassigned literal
fn _get_unassigned_literal(clause: &ast::Clause, asgmt: &ast::Asgmt) -> ast::Literal {
    clause.unassigned_literals(asgmt).next().unwrap().clone()
}


struct Watchers<'a> {
    clauses: HashMap<ast::Literal, HashSet<&'a ast::Clause>>,
    // Invariant: watchers map is total
    watchers: HashMap<&'a ast::Clause, (ast::Literal, ast::Literal)>,
}

// Assumption: clause has at least two unassigned literals
fn choose_watched_literals(clause: &ast::Clause, asgmt: &ast::Asgmt) -> (ast::Literal, ast::Literal) {
    let mut iter = clause.unassigned_literals(asgmt);
    let lit1 = iter.next().unwrap().inversion();
    let lit2 = iter.next().unwrap().inversion();
    (lit1, lit2)
}

impl<'a> Watchers<'a> {
    fn new(cnf: &'a ast::Cnf, asgmt: &ast::Asgmt, verbosity: usize) -> Self {
        let mut watchers = Self {
            clauses: HashMap::new(),
            watchers: HashMap::new(),
        };
        for clause in cnf.iter() {
            let (lit1, lit2) = choose_watched_literals(clause, asgmt);
            watchers.set(lit1, lit2, clause);
            if 2 < verbosity {
                log::info!("Watching literal {} and {} for clause {}", lit1, lit2, clause);
            }
        };
        watchers
    }

    fn clauses(&self, literal: ast::Literal) -> Option<&HashSet<&'a ast::Clause>> {
        self.clauses.get(&literal)
    }

    // Dangerous, could be used to violate invariant
    fn _clauses_mut(&mut self, literal: ast::Literal) -> Option<&mut HashSet<&'a ast::Clause>> {
        self.clauses.get_mut(&literal)
    }

    fn watchers(&self, clause: &'a ast::Clause) -> (ast::Literal, ast::Literal) {
        self.watchers.get(&clause).unwrap().clone()
    }

    // Dangerous, could be used to violate invariant
    fn _watchers_mut(&mut self, clause: &'a ast::Clause) -> &mut (ast::Literal, ast::Literal) {
        self.watchers.get_mut(&clause).unwrap()
    }

    fn set(&mut self, lit1: ast::Literal, lit2: ast::Literal, clause: &'a ast::Clause) {
        match self.clauses.get_mut(&lit1) {
            Some(clauses) => {
                clauses.insert(clause);
            },
            None => {
                self.clauses.insert(lit1, HashSet::from([clause]));
            },
        };
        match self.clauses.get_mut(&lit2) {
            Some(clauses) => {
                clauses.insert(clause);
            },
            None => {
                self.clauses.insert(lit2, HashSet::from([clause]));
            },
        };
        match self.watchers.get_mut(clause) {
            Some(literals) => {
                *literals = (lit1, lit2);
            },
            None => {
                self.watchers.insert(clause, (lit1, lit2));
            },
        }
    }

    fn replace(&mut self, current: ast::Literal, new: ast::Literal, clause: &'a ast::Clause) {
        if let Some(clauses) = self.clauses.get_mut(&current) {
            clauses.remove(clause);
        };
        match self.clauses.get_mut(&new) {
            Some(clauses) => {
                clauses.insert(clause);
            },
            None => {
                self.clauses.insert(new, HashSet::from([clause]));
            },
        };
        match self.watchers.get_mut(clause) {
            Some((lit1, lit2)) => {
                if *lit1 == current {
                    *lit1 = new;
                } else if *lit2 == current {
                    *lit2 = new;
                } else {
                    panic!()
                }
            },
            None => {
                panic!()
            },
        }
    }
}


// true means that unsat clause was found
fn propagate_with_watcher(_cnf: &ast::Cnf, asgmt: &mut ast::Asgmt, literal: ast::Literal, watchers: &mut Watchers, verbosity: usize)
    -> (HashSet<ast::Atom>, bool)
{
    let mut acc = HashSet::new();
    let mut new_lits: HashSet::<ast::Literal> = HashSet::from([literal]);

    loop {
        if new_lits.is_empty() {
            return (acc, false);
        }

        acc.extend(new_lits.iter().map(|lit| lit.atom()));
        let seen_clauses: Vec<_> = new_lits.into_iter()
            .flat_map(|lit|
                match watchers.clauses(lit.clone()) {
                    Some(clauses) => {
                        itertools::Either::Left(std::iter::zip(std::iter::repeat(lit), clauses.iter()))
                    },
                    None => itertools::Either::Right(std::iter::empty()),
                })
            .map(|(literal, &clause)| (literal, clause))
            .collect();
        new_lits = HashSet::new();

        for (watched_lit, clause) in seen_clauses {
            if 3 < verbosity {
                log::info!("Considering clause {}, watched by {}", clause, watched_lit);
            }
            match get_literal_when_unit(clause, asgmt) {
                Ok(lit) => {
                    let atom = lit.atom();
                    asgmt.insert(atom, lit.phase());
                    if 1 < verbosity {
                        log::info!("Unit propagating {} (by clause {})", lit, clause);
                    } else if 0 < verbosity {
                        log::info!("Unit propagating {}", lit);
                    };
                    new_lits.insert(lit);
                }
                Err(EvalResult::Sat) => continue,
                Err(EvalResult::Unsat) => {
                    if 3 < verbosity {
                        log::info!("Clause {} is false!", clause);
                        log::info!("Assignment: {}", asgmt);
                    }
                    acc.extend(new_lits.iter().map(|lit| lit.atom()));
                    return (acc, true)
                },
                // We always need two unassigned watchers, so since this watcher was
                // assigned but the clause was not a unit, we need to replace it
                // with a new watcher (one is guaranteed to exist since the clause
                // is not a unit and not unsat).
                Err(EvalResult::Unknown) => {
                    match clause.unassigned_literals(asgmt)
                        .map(|lit| lit.inversion())
                        .filter(|&lit| {
                            let (lit1, lit2) = watchers.watchers(clause);
                            lit != lit1 && lit != lit2
                        })
                        .next()
                    {
                        Some(new_watcher) => {
                            watchers.replace(watched_lit.clone(), new_watcher, clause);
                            if 2 < verbosity {
                                log::info!("Replacing watcher {} with {} (in clause {})", watched_lit, new_watcher, clause);
                            }
                        },
                        None => {
                            // Should be unreachable
                            panic!()
                        },
                    }
                },
            }
        };
    }
}



fn dpll(cnf: &ast::Cnf, asgmt: &mut ast::Asgmt, verbosity: usize) -> bool {
    let mut stack: Vec<(ast::Literal, HashSet<ast::Atom>)> = Vec::new();

    preprocess(cnf, asgmt, verbosity);
    bool_propagate(cnf, asgmt, verbosity);

    let mut watchers = Watchers::new(cnf, asgmt, verbosity);

    loop {
        if let Some(phase) = cnf.eval(asgmt) {
            if phase {
                return true
            };
            match stack.pop() {
                None => return false,
                Some((assumed, consquences)) => {
                    if 0 < verbosity {
                        log::info!("Assumption {} failed, assuming its inverse", assumed);
                    }
                    // Note: there is not a good way to log when an assumption
                    // fails in both directions, as this is implicit; the second
                    // assignment is treated as a consquence like the propagate
                    // variables.
                    for new in consquences {
                        if 3 < verbosity {
                            log::info!("Removing consequent {}", new);
                        };
                        asgmt.remove(&new);
                    };
                    if 3 < verbosity {
                        log::info!("Assignment after rolling back changes and assuming inverse: {}", asgmt);
                    }
                    let assumed_atom = assumed.atom();
                    asgmt.insert(assumed_atom, !assumed.phase());
                    if let Some((_, prev_consequences)) = stack.last_mut() {
                        prev_consequences.insert(assumed_atom);
                        let (prop_consequences, falsified) =
                            propagate_with_watcher(cnf, asgmt, assumed.inversion(), &mut watchers, verbosity);
                        prev_consequences.extend(prop_consequences);
                        if 3 < verbosity && falsified {
                            log::info!("Expecting immediate backtrack (after pop)")
                            // TODO: we know the cnf is false, we should deal with it here somehow
                        }
                    };
                    continue
                },
            }
        };

        let literal = choose_literal(cnf, asgmt);
        let atom = literal.atom();
        let phase = literal.phase();

        if 0 < verbosity {
            log::info!("Adding assumption: {}", literal);
        };
        asgmt.insert(atom, phase);
        let (prop_consequences, falsified) = propagate_with_watcher(cnf, asgmt, literal, &mut watchers, verbosity);
        stack.push((literal, prop_consequences));
        if 3 < verbosity && falsified {
            log::info!("Expecting immediate backtrack")
            // TODO: we know the cnf is false, we should deal with it here somehow
        }
    }
}


pub fn sat(cnf: &ast::Cnf, verbosity: usize) -> Option<ast::Asgmt> {
    let mut asgmt = ast::Asgmt::new();
    if dpll(cnf, &mut asgmt, verbosity) {
        Some(asgmt)
    } else {
        None
    }
}