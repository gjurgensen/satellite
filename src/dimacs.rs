use std::{fs, io, path};
use std::io::BufRead;

use itertools::Itertools;

use crate::clauses;
use crate::dpll;


// TODO: handle comments
pub fn read_dimacs<P>(path: P) -> Option<clauses::Cnf>
where P: AsRef<path::Path> {
    let file = fs::File::open(path).ok()?;
    let mut lines = io::BufReader::new(file).lines();
    let line = lines.next()?.ok()?;
    let (_num_var, mut num_clauses) = match line.split_whitespace().collect_tuple()? {
        ("p", "cnf", num_vars, num_clauses) =>
            (num_vars.parse::<u32>().ok()?,
             num_clauses.parse::<u32>().ok()?),
        _ => return None,
    };
    let mut clauses: clauses::Cnf = clauses::Cnf::new();
    for line in lines {
        // println!("Current clauses: {}", clauses);
        if num_clauses == 0 {
            return Some(clauses)
        }
        num_clauses -= 1;
        let line = line.ok()?;
        let mut clause = clauses::Clause::new();
        for lit_str in line.split_whitespace() {
            let num = lit_str.parse::<i32>().ok()?;
            if num == 0 {
                break
            }
            let pos = 0 < num;
            let var = clauses::Atom::new((if pos {num} else {-num}) as u32);
            clause.add(clauses::Literal::new(pos, var));
            // println!("Adding literal to clause: {}", clause);
        }
        // println!("Adding clause: {}", clause);
        clauses.add(clause);
    }
    Some(clauses)
}


pub fn read_dimacs_and_check_sat<P>(path: P) -> Result<Option<clauses::Asgmt>, String>
where P: AsRef<path::Path> {
    let cnf: clauses::Cnf = read_dimacs(path).ok_or("Error parsing DIMACs file.")?;
    println!("Read CNF: {}", cnf);
    Ok(dpll::sat(&cnf))
}

pub fn read_dimacs_check_sat_and_print<P>(path: P) -> Result<Option<clauses::Asgmt>, String>
where P: AsRef<path::Path> {
    let asgmt: Option<clauses::Asgmt> = read_dimacs_and_check_sat(path)?;
    if let Some(asgmt) = &asgmt {
        println!("sat: {:#?}", asgmt);
    } else {
        println!("unsat");
    }
    Ok(asgmt)
}