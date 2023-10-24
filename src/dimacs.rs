use std::{fs, io, path};
use std::io::BufRead;

use itertools::Itertools;

use crate::ast;
use crate::dpll;


pub fn read_dimacs<P>(path: P, verbosity: usize) -> Option<ast::Cnf>
where P: AsRef<path::Path> {
    let file = fs::File::open(path).ok()?;
    let mut lines = io::BufReader::new(file)
        .lines()
        .flatten()
        .filter(|str| str.chars().next().is_some_and(|c| c != 'c'));
    // let line = lines.next()?.ok()?;
    let line = lines.next()?;
    if 3 < verbosity {
        log::info!("Reading first line.");
    };
    let (num_vars, mut num_clauses) = match line.split_whitespace().collect_tuple()? {
        ("p", "cnf", num_vars, num_clauses) =>
            (num_vars.parse::<u32>().ok()?,
             num_clauses.parse::<u32>().ok()?),
        _ => return None,
    };
    if 3 < verbosity {
        log::info!("num_vars: {}, num_clauses: {}", num_vars, num_clauses);
    }
    let mut clauses: ast::Cnf = ast::Cnf::new();
    for line in lines {
        if num_clauses == 0 {
            return Some(clauses)
        }
        num_clauses -= 1;
        let mut clause = ast::Clause::new();
        for lit_str in line.split_whitespace() {
            let num = lit_str.parse::<i32>().ok()?;
            if num == 0 {
                break
            }
            let phase = 0 < num;
            let var = ast::Atom::new((if phase {num} else {-num}) as u32);
            let literal = ast::Literal::new(phase, var);
            if 3 < verbosity {
                log::info!("Adding literal {}", literal);
            };
            clause.add(literal);
        }
        clause.shrink_to_fit();
        if 3 < verbosity {
            log::info!("Adding clause: {}", clause);
        };
        clauses.add(clause);
    }
    Some(clauses)
}


pub fn read_dimacs_and_check_sat<P>(path: P, verbosity: usize) -> Result<Option<ast::Asgmt>, String>
where P: AsRef<path::Path> {
    let cnf: ast::Cnf = read_dimacs(path, verbosity).ok_or("Error parsing DIMACs file.")?;
    if 1 < verbosity {
        log::info!("Read CNF: {}", cnf);
    };
    Ok(dpll::sat(&cnf, verbosity))
}

pub fn read_dimacs_check_sat_and_print<P>(path: P, verbosity: usize) -> Result<Option<ast::Asgmt>, String>
where P: AsRef<path::Path> {
    let asgmt: Option<ast::Asgmt> = read_dimacs_and_check_sat(path, verbosity)?;
    if let Some(asgmt) = &asgmt {
        println!("SAT: {}", asgmt);
    } else {
        println!("UNSAT");
    }
    Ok(asgmt)
}