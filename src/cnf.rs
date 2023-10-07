use std::collections::HashMap;

// Variable identifiers
// Invariant: nonnegative
// TODO: rename to "atom"?
#[derive(Hash,PartialEq,Eq,Clone,Copy)]
pub struct Var {
  val: i32,
}

impl Var {
  pub fn new(val: i32) -> Self {
    Self {
      val: if val > 0 {val} else {
        // Panic/warn?
        (-1)*val
      },
    }
  }
}

// A partial mapping from variables to values
pub type Asgmt = HashMap<Var, bool>;


// A positive or negative literal
#[derive(PartialEq,Eq,Clone,Copy)]
pub struct Literal {
  data: i32,
}

impl Literal {
  pub fn new(positive: bool, var: Var) -> Self {
    Self {
      data: if positive {var.val} else {(-1)*var.val},
    }
  }

  pub fn positive(&self) -> bool {
    self.data > 0
  }

  pub fn var(&self) -> Var {
    Var::new(self.data.abs())
  }
}


// A disjunction of literals
pub type Clause = Vec<Literal>;

// A conjunction of clauses
pub type Cnf = Vec<Clause>;