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
  use crate::ast::{Expr, Function, Prototype};
  use inkwell::types::BasicMetadataTypeEnum;
  use Expr::*;
  use crate::parser::{parse_number_expr, expression_parser, parse_definition};
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
  fn test_fn() {
    let result = parse_definition().parse("def foo(x y) x + foo(y 4.0)").unwrap().0;
    let expected = Function::new(
      Prototype::new("foo".to_string(), vec!["x".to_string(), "y".to_string()]),
      BinOp {
        op: '+',
        lhs: Box::new(Variable("x".to_string())),
        rhs: Box::new(Call {
          callee: "foo".to_string(),
          args: vec![Variable("y".to_string()), Number(4.0)],
        }),
      }
    );
    assert_eq!(result, expected);
    let ctx = Context::create();
    let mut compiler = create_compiler!(&ctx, "tmp");
    compiler.function = &result;
    let result = compiler.compile_fn().expect("Failed to compile function");
    assert_eq!(result.get_name().to_str().unwrap(), "foo");
    assert_eq!(result.get_first_param().unwrap().to_string(), "\"double %x\"");
    assert_eq!(result.get_last_param().unwrap().to_string(), "\"double %y\"");
  }

  #[test]
  fn test_op() {
    let s: String = "6.0 < 4.0 * 20.0".to_string();
    let expr = expression_parser().parse(s.as_str()).unwrap().0;
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