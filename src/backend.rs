mod wasm {
  use crate::ast::{Expr, Function};
  use std::collections::HashMap;

  pub struct Transpiler {
    pub(crate) variables: HashMap<String, Expr>,
    pub(crate) functions: HashMap<String, Function>,
  }

  impl Transpiler {
    pub fn new() -> Self {
      Self {
        variables: HashMap::new(),
        functions: HashMap::new(),
      }
    }

    pub fn lookup_variable(&self, name: &str) -> Option<&Expr> {
      self.variables.get(name)
    }

    pub fn lookup_function(&self, name: &str) -> Option<&Function> {
      self.functions.get(name)
    }

    pub fn build_expression(&self, expr: Expr) -> f64 {
      build_expr_to_rust(expr)
    }

    pub fn transpile(&mut self, input: Expr) -> String {
      let mut output = String::new();
      output.push_str("fn main() -> f64 {\n");
      output.push_str(&format!("  {}", build_expr_to_rust(input)));
      output.push_str("\n}");
      output
    }
  }

  macro_rules! resolve_binop {
    ($op:tt, $lhs:expr, $rhs:expr) => {
      build_expr_to_rust($lhs) $op build_expr_to_rust($rhs)
    }
  }

  // macro_rules! resolve_call {
  //   ($callee:expr, $($args:expr)*) => {{
  //     let resolve_args = vec![$(build_expr_to_rust($args)),*];

  //     $callee(resolve_args)
  //   }}
  // }

  pub(crate) fn build_expr_to_rust(input: Expr) -> f64 {
    match input {
      Expr::Number(n) => n,
      Expr::BinOp { op, lhs, rhs } => {
        match op {
          '+' => resolve_binop!(+, *lhs, *rhs),
          '-' => resolve_binop!(-, *lhs, *rhs),
          '*' => resolve_binop!(*, *lhs, *rhs),
          '/' => resolve_binop!(/, *lhs, *rhs),
          _ => panic!("Unknown operator: {}", op),
        }
      },
      // Expr::Call { callee, args } => {
      //   resolve_call!(callee, args[0])
      // },
      _ => todo!(),

    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use wasm::*;
  use crate::ast::Expr;

  #[test]
  fn test_transpiler() {
    let mut transpiler = Transpiler::new();
    let input = Expr::BinOp {
      op: '+',
      lhs: Box::new(Expr::Number(3.0)),
      rhs: Box::new(Expr::BinOp {
        op: '*',
        lhs: Box::new(Expr::Number(4.0)),
        rhs: Box::new(Expr::Number(5.0)),
      }),
    };
    let output = transpiler.transpile(input);
    println!("{}", output);
  }

  #[test]
  fn test_build_expr_to_rust() {

    use Expr::*;

    let expr = Number(3.14);
    let res = build_expr_to_rust(expr);
    assert_eq!(res, 3.14);
    let expr = BinOp {
      op: '+',
      lhs: Box::new(Number(3.0)),
      rhs: Box::new(BinOp {
        op: '*',
        lhs: Box::new(Number(4.0)),
        rhs: Box::new(Number(5.0)),
      }),
    };
    let res = build_expr_to_rust(expr);
    assert_eq!(res, 23.0);
  }
}