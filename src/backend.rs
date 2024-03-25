mod wasm {
  use crate::ast::Expr;

  macro_rules! resolve_binop {
    ($op:tt, $lhs:expr, $rhs:expr) => {
      build_expr_to_rust($lhs) $op build_expr_to_rust($rhs)
    }
  }

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