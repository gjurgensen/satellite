use std::collections::{HashMap, HashSet};

use crate::cnf;


////////////////////////////////////////////////////////////////////////////////

// Assumption: clause is normal
fn get_literal_when_unit(clause: &cnf::Clause, asgmt: &cnf::Asgmt) -> Option<cnf::Literal> {
    let mut unit : Option<cnf::Literal> = None;
    for literal in clause {
        if !asgmt.contains_key(&literal.var()) {
            match unit {
                Some(_) => return None,
                None => unit = Some(literal.clone())
            }
        }
    };
    unit
}

fn try_find_propagate_unit(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> Option<cnf::Var> {
    for clause in clauses {
        if let Some(literal) = get_literal_when_unit(clause, asgmt) {
            let var = literal.var();
            let positive = literal.positive();
            println!("Unit propagating {}", literal);
            asgmt.insert(var, positive);
            return Some(var)
        }
    }
    None
}

fn unit_propagate_all(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> HashSet<cnf::Var> {
    let mut vars: HashSet<cnf::Var> = HashSet::new();
    while let Some(var) = try_find_propagate_unit(clauses, asgmt) {
        vars.insert(var);
    }
    vars
}

// Assumption: var is not bound
// Assumption: cnf is normal
fn purity(var: cnf::Var, clauses: &cnf::Cnf) -> Option<bool> {
    let mut occurs_pos = false;
    let mut occurs_neg = false;
    for clause in clauses {
        for literal in clause {
            if literal.var() == var {
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
        // If we want to add the assumption that var is bound, then this should
        // instead panic (due to assumption violation).
        None
    }
}

fn try_find_eliminate_pure_literal(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> Option<cnf::Var> {
    let free = cnf::free_vars(clauses, asgmt);
    // println!("free: {:#?}", free);
    for var in free {
        // println!("var: {:#?}", var);
        if let Some(val) = purity(var, clauses) {
            println!("var: {} found to have purity: {}", var, val);
            asgmt.insert(var, val);
            return Some(var)
        }
    };
    None
}

fn pure_literal_elimination(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> HashSet<cnf::Var> {
    let mut vars: HashSet<cnf::Var> = HashSet::new();
    while let Some(var) = try_find_eliminate_pure_literal(clauses, asgmt) {
        vars.insert(var);
    }
    vars
}


fn bool_propagate(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> HashSet<cnf::Var> {
    let mut vars: HashSet<cnf::Var> = HashSet::new();
    loop {
        // TODO: remove vars before returning None
        let new_vars_unit = unit_propagate_all(clauses, asgmt);
        let new_vars_pure = pure_literal_elimination(clauses, asgmt);
        if new_vars_unit.is_empty() && new_vars_pure.is_empty() {
            return vars;
        }
        vars.extend(new_vars_unit.into_iter().chain(new_vars_pure));
    }
}


// Assumption: There exists at least one literal in clauses
fn choose_literal(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> cnf::Literal {
    // This is of course the spot to try heuristics. For now, we arbitrarily
    // choose the first literal we come across.
    let bound = cnf::bound_vars(asgmt);
    clauses.iter()
        .flatten()
        .filter(|literal| bound.contains(&literal.var()))
        .cloned()
        .next()
        .unwrap()
}


fn dpll(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> bool {
    // TODO: condense
    println!("asgmt (start of round): {:?}", asgmt);
    let new_vars = bool_propagate(clauses, asgmt);
    println!("asgmt (post-propagate): {:?}", asgmt);

    if let Some(val) = cnf::eval_cnf(clauses, asgmt) {
        if !val {
            for new in new_vars {
                asgmt.remove(&new);
            }
        }
        return val
    }

    let literal = choose_literal(clauses, asgmt);
    let var = literal.var();
    let val = literal.positive();

    asgmt.insert(var, val);
    if dpll(clauses, asgmt) {
        return true
    }
    asgmt.insert(var, !val);
    if dpll(clauses, asgmt) {
        return true
    }
    asgmt.remove(&var);
    for new in new_vars {
        asgmt.remove(&new);
    }
    false
}


pub fn sat(clauses: &cnf::Cnf) -> Option<cnf::Asgmt> {
    let mut asgmt = HashMap::new();
    if dpll(clauses, &mut asgmt) {
        Some(asgmt)
    } else {
        None
    }
}