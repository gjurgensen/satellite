use std::env;

pub mod clauses;
pub mod dimacs;
pub mod dpll;

fn main() {
    if let Some(arg) = env::args().next() {
        if let Err(err) = dimacs::read_dimacs_check_sat_and_print(arg) {
            eprintln!("Error: {}", err);
        }
    }
    println!("Expecting commandline argument specifying a DIMACs file.")
}


////////////////////////////////////////////////////////////////////////////////

#[test]
fn empty_sat() {
    let cnf = clauses::Cnf::new();
    let result = dpll::sat(&cnf);
    if let Some(asgmt) = &result {
        println!("sat: {:#?}", asgmt);
    } else {
        println!("unsat");
    }
    assert!(result != None);
}

#[test]
fn singleton_sat() {
    let atom = clauses::Atom::new(0);
    let cnf: clauses::Cnf = clauses::Cnf::from(vec![vec![clauses::Literal::new(true, atom)]]);
    let result = dpll::sat(&cnf);
    if let Some(asgmt) = &result {
        println!("sat: {:#?}", asgmt);
    } else {
        println!("unsat");
    }
    assert!(result != None);
}

#[test]
fn trivial_noncontradiction() {
    let atom = clauses::Atom::new(0);
    let cnf: clauses::Cnf = clauses::Cnf::from(vec![
        vec![clauses::Literal::new(true, atom)],
        vec![clauses::Literal::new(false, atom)]
    ]);
    let result = dpll::sat(&cnf);
    if let Some(asgmt) = &result {
        println!("sat: {:#?}", asgmt);
    } else {
        println!("unsat");
    }
    assert!(result == None);
}

#[test]
fn dimacs_mini() {
    if let Err(err) = dimacs::read_dimacs_check_sat_and_print("tests/mini.cnf") {
        eprintln!("Error: {}", err);
    }
}
