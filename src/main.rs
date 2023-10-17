use std::env;

pub mod clauses;
pub mod dimacs;
pub mod dpll;

fn main() {
    if let Some(path) = env::args().collect::<Vec<_>>().get(1) {
        if let Err(err) = dimacs::read_dimacs_check_sat_and_print(path, true) {
            eprintln!("Error: {}", err);
        }
    }
    eprintln!("Expecting commandline argument specifying a DIMACS file.");
}


////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_literal() {
    let pos = true;
    let atom = clauses::Atom::new(42);
    let literal = clauses::Literal::new(pos, atom);
    println!("Literal::new({}, {}) = {}", pos, atom, literal);
    assert_eq!(pos, literal.positive());
    assert_eq!(atom, literal.atom());

    let pos = false;
    let literal = clauses::Literal::new(pos, atom);
    println!("Literal::new({}, {}) = {}", pos, atom, literal);
    assert_eq!(pos, literal.positive());
    assert_eq!(atom, literal.atom());
}

#[test]
fn empty_sat() {
    let cnf = clauses::Cnf::new();
    let result = dpll::sat(&cnf, true);
    if let Some(asgmt) = &result {
        println!("sat: {}", asgmt);
    } else {
        println!("unsat");
    }
    assert!(result != None);
}

#[test]
fn singleton_sat() {
    let atom = clauses::Atom::new(0);
    let cnf: clauses::Cnf = clauses::Cnf::from(vec![vec![clauses::Literal::new(true, atom)]]);
    let result = dpll::sat(&cnf, true);
    if let Some(asgmt) = &result {
        println!("sat: {}", asgmt);
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
    let result = dpll::sat(&cnf, true);
    if let Some(asgmt) = &result {
        println!("sat: {}", asgmt);
    } else {
        println!("unsat");
    }
    assert!(result == None);
}

#[test]
fn dimacs_mini() {
    if let Err(err) = dimacs::read_dimacs_check_sat_and_print("tests/mini.cnf", true) {
        eprintln!("Error: {}", err);
    }
}

#[test]
fn dimacs_mini2() {
    if let Err(err) = dimacs::read_dimacs_check_sat_and_print("tests/mini2.cnf", true) {
        eprintln!("Error: {}", err);
    }
}

#[test]
fn dimacs_mini3() {
    if let Err(err) = dimacs::read_dimacs_check_sat_and_print("tests/mini3.cnf", true) {
        eprintln!("Error: {}", err);
    }
}

#[test]
fn dimacs_uf20_01000() {
    if let Err(err) = dimacs::read_dimacs_check_sat_and_print("tests/uf20-01000.cnf", true) {
        eprintln!("Error: {}", err);
    }
}

// Takes 59s
#[test]
fn dimacs_uf100_01() {
    if let Err(err) = dimacs::read_dimacs_check_sat_and_print("tests/uf100-430/uf100-01.cnf", false) {
        eprintln!("Error: {}", err);
    }
}

// Takes 187s
#[test]
fn dimacs_uuf100_01() {
    if let Err(err) = dimacs::read_dimacs_check_sat_and_print("tests/uuf100-430/uuf100-01.cnf", false) {
        eprintln!("Error: {}", err);
    }
}

#[test]
fn dimacs_uf250_01() {
    if let Err(err) = dimacs::read_dimacs_check_sat_and_print("tests/uf250/uf250-01.cnf", false) {
        eprintln!("Error: {}", err);
    }
}