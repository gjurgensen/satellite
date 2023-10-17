use std::{fs, io, path};
use std::io::BufRead;

use itertools::Itertools;

use crate::clauses;
use crate::dpll;


pub fn read_dimacs<P>(path: P, log: bool) -> Option<clauses::Cnf>
where P: AsRef<path::Path> {
    let file = fs::File::open(path).ok()?;
    let mut lines = io::BufReader::new(file)
        .lines()
        .flatten()
        .filter(|str| str.chars().next().is_some_and(|c| c != 'c'));
    // let line = lines.next()?.ok()?;
    let line = lines.next()?;
    if log {
        println!("Reading first line.");
    };
    let (num_vars, mut num_clauses) = match line.split_whitespace().collect_tuple()? {
        ("p", "cnf", num_vars, num_clauses) =>
            (num_vars.parse::<u32>().ok()?,
             num_clauses.parse::<u32>().ok()?),
        _ => return None,
    };
    if log {
        println!("num_vars: {}, num_clauses: {}", num_vars, num_clauses);
    }
    let mut clauses: clauses::Cnf = clauses::Cnf::new();
    for line in lines {
        if num_clauses == 0 {
            return Some(clauses)
        }
        num_clauses -= 1;
        // let line = line.ok()?;
        let mut clause = clauses::Clause::new();
        for lit_str in line.split_whitespace() {
            let num = lit_str.parse::<i32>().ok()?;
            if num == 0 {
                break
            }
            let pos = 0 < num;
            let var = clauses::Atom::new((if pos {num} else {-num}) as u32);
            let literal = clauses::Literal::new(pos, var);
            if log {
                println!("Adding literal {}", literal);
            };
            clause.add(literal);
        }
        if log {
            println!("Adding clause: {}", clause);
        };
        clauses.add(clause);
    }
    Some(clauses)
}


pub fn read_dimacs_and_check_sat<P>(path: P, log: bool) -> Result<Option<clauses::Asgmt>, String>
where P: AsRef<path::Path> {
    let cnf: clauses::Cnf = read_dimacs(path, false).ok_or("Error parsing DIMACs file.")?;
    if log {
        println!("Read CNF: {}", cnf);
    };
    Ok(dpll::sat(&cnf, log))
}

pub fn read_dimacs_check_sat_and_print<P>(path: P, log: bool) -> Result<Option<clauses::Asgmt>, String>
where P: AsRef<path::Path> {
    let asgmt: Option<clauses::Asgmt> = read_dimacs_and_check_sat(path, log)?;
    if let Some(asgmt) = &asgmt {
        println!("sat: {}", asgmt);
    } else {
        println!("unsat");
    }
    Ok(asgmt)
}