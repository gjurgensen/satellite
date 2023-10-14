use std::collections::HashMap;
use std::collections::HashSet;

pub mod cnf;


////////////////////////////////////////////////////////////////////////////////

// TODO: incrementalize count


////////////////////////////////////////////////////////////////////////////////

enum UpdateResult {
    Added,
    Redundant,
    Contradictory,
}

// Does not update when doing so would be contradictory
// TODO (OPTIMIZATION): avoid double lookup
fn try_add_asgmt(var: cnf::Var, val: bool, asgmt: &mut cnf::Asgmt) -> UpdateResult {
    match asgmt.get(&var) {
        Some(current_val) => {
            if val == *current_val {
                UpdateResult::Redundant
            } else {
                UpdateResult::Contradictory
            }
        },
        None => {
            asgmt.insert(var, val);
            UpdateResult::Added
        },
    }
}


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
            let positive = literal.positive();
            let var = literal.var();
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
    let bound = cnf::bound_vars(asgmt);
    for var in bound {
        if let Some(val) = purity(var, clauses) {
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


// TODO
fn all_clauses_true(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> bool {
    todo!()
}


// TODO
fn exists_false_clause(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> bool {
    todo!()
}

// TODO
fn choose_literal(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> cnf::Literal {
    todo!()
}


fn dpll(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> bool {
    // TODO: condense
    let new_vars = bool_propagate(clauses, asgmt);
    if all_clauses_true(clauses, asgmt) {
        return true
    }
    if exists_false_clause(clauses, asgmt) {
        // Remove new_vars
        for new in new_vars {
            asgmt.remove(&new);
        }
        return false
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


fn main() {
    println!("Hello, world!");
}
