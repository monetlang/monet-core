use std::collections::HashMap;
use std::str::Bytes;
use combine::many;
use combine::parser;
use combine::attempt;
use combine::parser::char::string;
use combine::parser::choice::or;
use combine::{not_followed_by, optional};
use combine::parser::char::{spaces, newline, digit, char, letter};
use combine::{between, choice, many1, sep_by, ParseError, Parser};
use combine::stream::Stream;

use crate::ast::Expr;

fn identifier<I>() -> impl Parser<I, Output = Expr>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  (
    many1(choice((letter(), digit(), char('_')))),
    optional(spaces()),
    optional(
      between(
        char('('), 
        char(')'), sep_by(expr(), spaces())
      )
    )
  )
    .map(|(id, _, args)| {
      match args {
        Some(args) => Expr::Call {
          callee: id,
          args,
        },
        None => Expr::Variable(id),
      }
    })
}

fn paren_expr<I>() -> impl Parser<I, Output = Expr>
where
    I: Stream<Token = char>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  between(char('('), char(')'), expr())
}

fn integer_part<I>() -> impl Parser<I, Output = f64>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  many1(digit()).map(|string: String| string.parse::<f64>().unwrap())
}

fn decimal_part<I>() -> impl Parser<I, Output = f64>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  many1(digit()).map(|digits: String| {
    let mut decimal = "0.".to_owned();
    decimal.push_str(&digits);
    decimal.parse::<f64>().unwrap()
  })
}

fn decimal<I>() -> impl Parser<I, Output = f64>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  optional(integer_part())
    .skip(char('.'))
    .and(optional(decimal_part()))
    .map(|(integer, decimal)| integer.unwrap_or(0.0) + decimal.unwrap_or(0.0))
}


fn number<I>() -> impl Parser<I, Output = Expr>
where
    I: Stream<Token = char>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  decimal().map(Expr::Number)
}

fn expr_<'a, I>() -> impl Parser<I, Output = Expr>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  let skip_spaces = || spaces().silent();

  choice((
    number(),
    paren_expr(),
    identifier(),
  )).skip(skip_spaces())
}

parser!{
    fn expr[I]()(I) -> Expr
    where [I: Stream<Token = char>]
  {
    expr_()
  }
}


#[cfg(test)]
mod tests {
  use combine::Parser;
  use super::*;

  #[test]
  fn test_number() {
    let result = number().parse("3.14").unwrap().0;
    assert_eq!(result, Expr::Number(3.14));
  }

  #[test]
  fn test_paren_expr() {
    let result = paren_expr().parse("(3.14)").unwrap().0;
    assert_eq!(result, Expr::Number(3.14));
  }

  #[test]
  fn test_identifier() {
    let result = identifier().parse("foo").unwrap().0;
    assert_eq!(result, Expr::Variable("foo".to_string()));

    let result = identifier().parse("foo (bar 3.14)").unwrap().0;
    assert_eq!(result, Expr::Call {
      callee: "foo".to_string(),
      args: vec![Expr::Variable("bar".to_string()), Expr::Number(3.14)],
    });
  }
}