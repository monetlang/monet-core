use wasm_bindgen::prelude::*;


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
pub enum SimpleExpr {
  Number(i32),
  Add(Box<SimpleExpr>, Box<SimpleExpr>),
  Sub(Box<SimpleExpr>, Box<SimpleExpr>),
  Mul(Box<SimpleExpr>, Box<SimpleExpr>),
  Div(Box<SimpleExpr>, Box<SimpleExpr>),
}

impl SimpleExpr {
  pub fn eval(&self) -> i32 {
    match self {
      SimpleExpr::Number(n) => *n,
      SimpleExpr::Add(lhs, rhs) => lhs.eval() + rhs.eval(),
      SimpleExpr::Sub(lhs, rhs) => lhs.eval() - rhs.eval(),
      SimpleExpr::Mul(lhs, rhs) => lhs.eval() * rhs.eval(),
      SimpleExpr::Div(lhs, rhs) => lhs.eval() / rhs.eval(),
    }
  }
}

#[wasm_bindgen]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct ExprWrapper {
  variant_type: u8,
  number_value: Option<f64>,
  variable_name: Option<String>,
  op_value: Option<char>,
  lhs_value: Option<Box<ExprWrapper>>,
  rhs_value: Option<Box<ExprWrapper>>,
  callee_value: Option<String>,
  args_value: Option<Vec<ExprWrapper>>,
}

impl ToString for ExprWrapper {
  fn to_string(&self) -> String {
    match self.variant_type {
      0 => self.number_value.unwrap().to_string(),
      1 => self.variable_name.clone().unwrap(),
      2 => format!(
        "{} {} {}", 
        self.lhs_value.clone().unwrap().to_string(),
        self.op_value.clone().unwrap(),
        self.rhs_value.clone().unwrap().to_string()),
      3 => {
        let args = self.args_value.clone().unwrap().iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(" ");
        format!("{} {}", self.callee_value.clone().unwrap(), args)
      },
      4 => "".to_string(),
      _ => "".to_string(),
    }
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

impl Expr {
  pub fn wrap(&self) -> ExprWrapper {
    let default = ExprWrapper::default();
    match self {
      Expr::Number(n) => ExprWrapper {
        variant_type: 0,
        number_value: Some(*n),
        ..default
      },
      Expr::Variable(v) => ExprWrapper {
        variant_type: 1,
        variable_name: Some(v.clone()),
        ..default
      },
      Expr::BinOp { op, lhs, rhs } => ExprWrapper {
        variant_type: 2,
        op_value: Some(*op),
        lhs_value: Some(Box::new(lhs.wrap())),
        rhs_value: Some(Box::new(rhs.wrap())),
        ..default
      },
      Expr::Call { callee, args } => ExprWrapper {
        variant_type: 3,
        callee_value: Some(callee.clone()),
        args_value: Some(
          args.iter()
            .map(|arg| arg.wrap())
            .collect::<Vec<ExprWrapper>>()
            // .iter()
            // .map(|arg| arg.wrap())
            // .collect::<Vec<ExprWrapper>>()),
        ),
        ..default
      },
      Expr::Nothing => ExprWrapper {
        variant_type: 4,
        ..default
      },
    }
  }
}

impl ToString for Expr {
  fn to_string(&self) -> String {
    match self {
      Expr::Number(n) => n.to_string(),
      Expr::Variable(v) => v.clone(),
      Expr::BinOp { op, lhs, rhs } => format!("{} {} {}", lhs.to_string(), op, rhs.to_string()),
      Expr::Call { callee, args } => {
        let args = args.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(" ");
        format!("{} {}", callee, args)
      },
      Expr::Nothing => "".to_string(),
    }
  }
}

impl Default for Expr {
  fn default() -> Self {
    Self::Nothing
  }
}