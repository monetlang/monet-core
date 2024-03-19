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
  use crate::parser::parse_number_expr;
  use crate::compiler::Compiler;
  use crate::compiler::create_compiler;
  use inkwell::context::Context;

  #[test]
  fn test_decimal() {
    let expr = parse_number_expr().parse("3.14").unwrap().0;
    assert_eq!(expr, Expr::Number(3.14));
    let ctx = Context::create();
    let compiler = create_compiler!(&ctx, "tmp");
    let result = compiler.compile_expr(&expr).unwrap();
    assert_eq!(result.to_string(), "\"double 3.140000e+00\"".to_string());
  }
}