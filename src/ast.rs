use inkwell::values::FloatValue;

#[derive(Debug, PartialEq)]
pub(crate) struct Prototype {
  pub name: String,
  pub args: Vec<String>,
}

impl Prototype {
  pub fn new(name: String, args: Vec<String>) -> Self {
    Self { name, args }
  }
  pub fn name(&self) -> String {
    self.name.clone()
  }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Function {
  pub prototype: Prototype,
  pub body: Expr,
}

impl Function {
  pub fn new(prototype: Prototype, body: Expr) -> Self {
    Self { prototype, body }
  }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Expr {
  Number(f64),
  Variable(String),
  BinOp {
    op: char,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
  },
  Call {
    callee: String,
    args: Vec<Expr>,
  },
}