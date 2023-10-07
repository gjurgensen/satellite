use std::collections::HashMap;
// use crate::cnf;

pub mod cnf;


fn get_literal_when_unit(clause: &cnf::Clause, asgmt: &cnf::Asgmt) -> Option<cnf::Literal> {
    let mut unit : Option<cnf::Literal> = None;
    for literal in clause {
        if !asgmt.contains_key(&literal.var()) {
            match unit {
                // NOTE: we assume no repeats of vars in a clause. If this is
                // not a safe assumption, we'd need to check the value of the
                // accumulator, and check if we contradict or if it is redundant
                // (which would still count as a unit).
                Some(_) => return None,
                None => unit = Some(literal.clone())
            }
        }
    };
    unit
}


fn unit_propagate_all(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> Vec<cnf::Var> {
    let mut vars: Vec<cnf::Var> = Vec::new();
    loop {
        for clause in clauses {
            if let Some(literal) = get_literal_when_unit(clause, asgmt) {
                let positive = literal.positive();
                let var = literal.var();
                // TODO: check for consistency here, or will that be done with
                // after all the units have been propagated?
                asgmt.insert(var, positive);
                // TODO: only push a var if it was not previously assigned
                // (and at the same time, might as well check if redundant or contradictory)
                vars.push(var);
                break;
            }
        };
        break vars
    }
}


fn dpll(clauses: &cnf::Cnf, asgmt: &mut cnf::Asgmt) -> bool {
    // What is mutation story? Need to be able to roll back on fn return
    // Plan is to explicitly track new assignments in order to delete them when necessary
    // This may motivate some other data structure. With easier deletes
    // (although I suspect much more time will be spent looking up assignments
    // than deleting)
    let new_vars = unit_propagate_all(clauses, asgmt);

    // ...
    false
}


fn main() {
    println!("Hello, world!");
}
