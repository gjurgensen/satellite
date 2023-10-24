use clap::Parser;

pub mod ast;
pub mod dimacs;
pub mod dpll;


/// Satellite is a toy SAT solver
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    /// Verbosity, 0-4
    #[arg(short, long, default_value_t = 1)]
    verbosity: usize,

    /// DIMACS file
    file: std::path::PathBuf,
}


fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();

    let args = Args::parse();

    if let Err(err) = dimacs::read_dimacs_check_sat_and_print(args.file, args.verbosity) {
        log::error!("{}", err)
    }
}


////////////////////////////////////////////////////////////////////////////////

// TODO: tests won't have logging enabled; this needs to be done with the command:
//   env_logger::builder().filter_level(log::LevelFilter::Info).init();
// Is there a way to have some code run before every test? (Something like
// JUnit's @BeforeEach)


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
    let result = dpll::sat(&cnf, 2);
    if let Some(asgmt) = &result {
        println!("sat: {}", asgmt);
    } else {
        println!("unsat");
    }
    assert!(result.is_some());
}

#[test]
// Panics because we don't expect any singleton clauses to exist. This should be handled by normalization.
fn singleton_sat() {
    let atom = ast::Atom::new(0);
    let cnf: ast::Cnf = ast::Cnf::from(vec![vec![ast::Literal::new(true, atom)]]);
    let result = dpll::sat(&cnf, 2);
    if let Some(asgmt) = &result {
        println!("sat: {}", asgmt);
    } else {
        println!("unsat");
    }
    assert!(result.is_some());
}

#[test]
// Same panic as above
fn trivial_noncontradiction() {
    let atom = ast::Atom::new(0);
    let cnf: ast::Cnf = ast::Cnf::from(vec![
        vec![ast::Literal::new(true, atom)],
        vec![ast::Literal::new(false, atom)]
    ]);
    let result = dpll::sat(&cnf, 2);
    if let Some(asgmt) = &result {
        println!("sat: {}", asgmt);
    } else {
        println!("unsat");
    }
    assert!(result == None);
}

fn dimacs_test<P>(path: P, sat: bool, verbosity: usize) where P: AsRef<std::path::Path> {
    // env_logger::builder().filter_level(log::LevelFilter::Info).init();

    let result = dimacs::read_dimacs_check_sat_and_print(path, verbosity);
    if let Err(err) = &result {
        eprintln!("Error: {}", err);
    }
    assert!(result.is_ok());
    let result = result.unwrap();
    if sat {
        assert!(result.is_some())
    } else {
        assert!(result == None)
    }
}

#[test]
fn dimacs_mini() {
    dimacs_test("tests/mini.cnf", true, 2)
}

#[test]
fn dimacs_mini2() {
    dimacs_test("tests/mini2.cnf", true, 2)
}

#[test]
fn dimacs_mini3() {
    dimacs_test("tests/mini3.cnf", true, 2)
}

#[test]
fn dimacs_uf20_01000() {
    dimacs_test("tests/uf20-01000.cnf", true, 2)
}

#[test]
fn dimacs_uf100_01() {
    dimacs_test("tests/uf100-430/uf100-01.cnf", true, 1)
}

#[test]
fn dimacs_uuf100_01() {
    dimacs_test("tests/uuf100-430/uuf100-01.cnf", false, 1)
}

#[test]
fn dimacs_uf250_01() {
    dimacs_test("tests/uf250/uf250-01.cnf", true, 0)
}