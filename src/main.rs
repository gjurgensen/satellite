use std::env;

pub mod ast;
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
    let phase = true;
    let atom = ast::Atom::new(42);
    let literal = ast::Literal::new(phase, atom);
    println!("Literal::new({}, {}) = {}", phase, atom, literal);
    assert_eq!(phase, literal.phase());
    assert_eq!(atom, literal.atom());

    let phase = false;
    let literal = ast::Literal::new(phase, atom);
    println!("Literal::new({}, {}) = {}", phase, atom, literal);
    assert_eq!(phase, literal.phase());
    assert_eq!(atom, literal.atom());
}

#[test]
fn empty_sat() {
    let cnf = ast::Cnf::new();
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
    let atom = ast::Atom::new(0);
    let cnf: ast::Cnf = ast::Cnf::from(vec![vec![ast::Literal::new(true, atom)]]);
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
    let atom = ast::Atom::new(0);
    let cnf: ast::Cnf = ast::Cnf::from(vec![
        vec![ast::Literal::new(true, atom)],
        vec![ast::Literal::new(false, atom)]
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