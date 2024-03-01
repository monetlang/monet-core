use combine::error::Token;
use combine::parser::choice::or;
use combine::parser::combinator::Map;
use combine::{optional, token};
use combine::parser::char::{spaces, digit, char, letter};
use combine::{attempt, between, choice, many, many1, sep_by, ParseError, Parser};
use combine::parser::repeat::chainl1;
use combine::stream::Stream;
use combine::parser;

use crate::ast::Expr;

fn parse_identifier_expr<I>() -> impl Parser<I, Output = Expr>
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

fn parse_paren_expr<I>() -> impl Parser<I, Output = Expr>
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


fn parse_number_expr<I>() -> impl Parser<I, Output = Expr>
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
    parse_number_expr(),
    parse_paren_expr(),
    parse_identifier_expr(),
  )).skip(skip_spaces())
}

parser!{
  fn expr[I]()(I) -> Expr
  where [I: Stream<Token = char>]
  {
    expr_()
  }
}

parser!{
  fn expr_parser[I]()(I) -> Expr
  where [I: Stream<Token = char>]
  {
    expression_parser()
  }
}


fn parse_primary<I>() -> impl Parser<I, Output = Expr>
where
    I: Stream<Token = char>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  spaces()
    .with(
      choice((
        parse_number_expr(),
        parse_paren_expr(),
        parse_identifier_expr(),
      ))
    ).skip(spaces())
}

fn create_binop_node(c: char) -> impl Fn(Expr, Expr) -> Expr {
  move |l: Expr, r: Expr| Expr::BinOp {
      op: c,
      lhs: Box::new(l),
      rhs: Box::new(r),
  }
}

macro_rules! create_binop {
  ($c:expr) => {
    return create_binop_node($c)
  }
}

fn expression_parser<I>() -> impl Parser<I, Output = Expr>
where
    I: Stream<Token = char>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    let number_expr = parse_number_expr();
    let paren_expr = between(char('('), char(')'), expr_parser());
    let factor = spaces()
      .with(choice((paren_expr, number_expr)))
      .skip(spaces());

    let div = token::<I>('/').map(|c| create_binop!(c));
    let mul = token::<I>('*').map(|c| create_binop!(c));

    // let op1 = div.or(mul);

    let term = chainl1(chainl1(factor, div), mul);
    // let term = chainl1(factor, op1);

    let sub = token::<I>('-').map(|c| create_binop!(c));
    let add = token::<I>('+').map(|c| create_binop!(c));

    // let op2 = choice((sub, add));

    let expr = chainl1(chainl1(term, sub), add);
    // spaces().with(expr).skip(spaces())
    expr
}


#[cfg(test)]
mod tests {
  use combine::Parser;
  use super::*;

  #[test]
  fn test_sub_and_add_op_precedence() {
    use Expr::{Number, BinOp};

    let result = expression_parser().parse("3.0 + 4.0 - 2.0").unwrap().0;
    let expected = BinOp {
      op: '+',
      lhs: Box::new(Number(3.0)),
      rhs: Box::new(BinOp {
        op: '-',
        lhs: Box::new(Number(4.0)),
        rhs: Box::new(Number(2.0)),
      }),
    };

    assert_eq!(result, expected);
  }

  #[test]
  fn test_div_and_mul_op_precedence() {

    use Expr::{Number, BinOp};

    let result = expression_parser().parse("3.0 * 4.0 / 2.0").unwrap().0;
    let expected = BinOp {
      op: '*',
      lhs: Box::new(Number(3.0)),
      rhs: Box::new(BinOp {
        op: '/',
        lhs: Box::new(Number(4.0)),
        rhs: Box::new(Number(2.0)),
      }),

    };

    assert_eq!(result, expected);
  }

  #[test]
  fn test_all_op_precedence() {
    use Expr::{Number, BinOp};

    let result = expression_parser().parse("3.0 + 4.0 * 2.0 / 2.0 - 1.0").unwrap().0;
    let expected = BinOp {
      op: '+',
      lhs: Box::new(Number(3.0)),
      rhs: Box::new(BinOp {
        op: '-',
        lhs: Box::new(BinOp {
          op: '*',
          lhs: Box::new(Number(4.0)),
          rhs: Box::new(BinOp {
            op: '/',
            lhs: Box::new(Number(2.0)),
            rhs: Box::new(Number(2.0)),
          }),
        }),
        rhs: Box::new(Number(1.0)),
      })
    };
    assert_eq!(result, expected);
  }

  #[test]
  fn test_op_with_paren() {
    use Expr::{Number, BinOp};

    let result = expression_parser().parse("(3.0 + 4.0) * 2.0").unwrap().0;
    let expected = BinOp {
      op: '*',
      lhs: Box::new(BinOp {
        op: '+',
        lhs: Box::new(Number(3.0)),
        rhs: Box::new(Number(4.0)),
      }),
      rhs: Box::new(Number(2.0)),
    };
    assert_eq!(result, expected);
  }

  #[test]
  fn test_parse_primary() {
    let result = parse_primary().parse(" 3.14  ").unwrap().0;
    assert_eq!(result, Expr::Number(3.14));
  }

  #[test]
  fn test_number() {
    let result = parse_number_expr().parse("3.14").unwrap().0;
    assert_eq!(result, Expr::Number(3.14));
  }

  #[test]
  fn test_paren_expr() {
    let result = parse_paren_expr().parse("(3.14)").unwrap().0;
    assert_eq!(result, Expr::Number(3.14));
  }

  #[test]
  fn test_identifier() {
    let result = parse_identifier_expr().parse("foo").unwrap().0;
    assert_eq!(result, Expr::Variable("foo".to_string()));

    let result = parse_identifier_expr().parse("foo (bar 3.14)").unwrap().0;
    assert_eq!(result, Expr::Call {
      callee: "foo".to_string(),
      args: vec![Expr::Variable("bar".to_string()), Expr::Number(3.14)],
    });
  }
}