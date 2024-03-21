use std::collections::HashMap;
use combine::many;
use combine::parser;
use combine::attempt;
use combine::parser::char::string;
use combine::parser::choice::or;
use combine::{not_followed_by, optional};
use combine::parser::char::{spaces, digit, char, letter};
use combine::{between, choice, many1, sep_by, ParseError, Parser};
use combine::stream::Stream;

#[cfg(test)]
mod tests {
  use super::*;
  use crate::ast::{Expr, Function};
  use crate::parser::{parse_number_expr, expression_parser};
  use crate::compiler::Compiler;
  use crate::compiler::create_compiler;
  use inkwell::context::Context;

  #[test]
  fn test_decimal() {
    let expr = parse_number_expr().parse("30.14").unwrap().0;
    assert_eq!(expr, Expr::Number(30.14));
    let ctx = Context::create();
    let compiler = create_compiler!(&ctx, "tmp");
    let result = compiler.compile_expr(&expr).unwrap();
    assert_eq!(result.to_string(), "\"double 3.014000e+01\"".to_string());
  }

  #[test]
  fn test_op() {
    use Expr::{Number, BinOp};
    let expr = expression_parser().parse("6.0 < 4.0 * 20.0").unwrap().0;
    let expected = BinOp {
      op: '<',
      lhs: Box::new(Number(6.0)),
      rhs: Box::new(BinOp {
        op: '*',
        lhs: Box::new(Number(4.0)),
        rhs: Box::new(Number(20.0)),
      }),
    };

    assert_eq!(expr, expected);
    let ctx = Context::create();
    let compiler = create_compiler!(&ctx, "tmp");
    let fn_type = ctx.f64_type().fn_type(&[], false);
    let function = compiler.module.add_function("my_function", fn_type, None);
    let basic_block = compiler.context.append_basic_block(function, "entry");
    compiler.builder.position_at_end(basic_block);

    let result = compiler.compile_expr(&expr).unwrap();
    // true
    assert_eq!(result.to_string(), "\"double 1.000000e+00\"".to_string());
    let expr = expression_parser().parse("5.0 < 2.0").unwrap().0;
    let result = compiler.compile_expr(&expr).unwrap();
    // false 
    assert_eq!(result.to_string(), "\"double 0.000000e+00\"".to_string());
  }
}