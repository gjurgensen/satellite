pub mod cnf;
pub mod dpll;

fn main() {
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