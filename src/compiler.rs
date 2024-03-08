use inkwell::context::Context;
use inkwell::values::FloatValue;
use crate::ast::Expr;

pub struct Compiler<'ctx> {
  context: &'ctx Context,
}

impl<'ctx> Compiler<'ctx> {
  pub(crate) fn compile_expr(&mut self, expr: &Expr) -> Result<FloatValue<'ctx>, &'static String> {
    match *expr {
      Expr::Number(n) => Ok(self.context.f64_type().const_float(n)),
      _ => todo!("compile_expr: {:?}", expr),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use inkwell::context::Context;

  #[test]
  fn test_compile_number() {
    let mut compiler = Compiler {
      context: &Context::create(),
    };
    let expr = Expr::Number(1.0);
    let result = compiler.compile_expr(&expr).unwrap();
    let s = result.to_string();
    assert_eq!(s, "\"double 1.000000e+00\"".to_string());
  }
}
