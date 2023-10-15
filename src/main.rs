use std::env;

pub mod cnf;
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
    let cnf = vec![];
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
    let var = cnf::Var::new(0);
    let cnf: cnf::Cnf = vec![vec![cnf::Literal::new(true, var)]];
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
    let var = cnf::Var::new(0);
    let cnf: cnf::Cnf = vec![
        vec![cnf::Literal::new(true, var)],
        vec![cnf::Literal::new(false, var)]
    ];
    let result = dpll::sat(&cnf);
    if let Some(asgmt) = &result {
        println!("sat: {:#?}", asgmt);
    } else {
        println!("unsat");
    }
    assert!(result == None);
}

#[test]
fn dimas_mini() {
    if let Err(err) = dimacs::read_dimacs_check_sat_and_print("tests/mini.cnf") {
        eprintln!("Error: {}", err);
    }
}
