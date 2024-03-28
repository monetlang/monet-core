use inkwell::values::FloatValue;

#[derive(Debug, PartialEq)]
pub struct Prototype {
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

impl Default for Prototype {
  fn default() -> Self {
    Self {
      name: "tmp".to_string(),
      args: Vec::new(),
    }
  }

}

#[derive(Debug, Default, PartialEq)]
pub struct Function {
  pub prototype: Prototype,
  pub body: Expr,
}

impl Function {
  pub fn new(prototype: Prototype, body: Expr) -> Self {
    Self { prototype, body }
  }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
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
  Nothing,
}

impl Default for Expr {
  fn default() -> Self {
    Self::Nothing
  }
}