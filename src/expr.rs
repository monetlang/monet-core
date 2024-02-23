use combine::parser;
use combine::attempt;
use combine::{not_followed_by, optional};
use combine::parser::char::{spaces, digit, char, letter};
use combine::{between, choice, many1, sep_by, ParseError, Parser};
use combine::stream::Stream;

#[derive(Debug, PartialEq)]
pub enum Expr {
  Id(String),
  Decimal(f64),
  Integer(usize),
  Array(Vec<Expr>),
  Pair(Box<Expr>, Box<Expr>),
}

parser!{
    fn expr[I]()(I) -> Expr
    where [I: Stream<Token = char>]
  {
    expr_()
  }
}

fn integer<I>() -> impl Parser<I, Output = usize>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  spaces()
    .with(many1(digit()))
    .skip(not_followed_by(char('.')))
    .map(|string: String| string.parse::<usize>().unwrap())
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

fn expr_<'a, I>() -> impl Parser<I, Output = Expr>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  let word = many1(letter());
  let skip_spaces = || spaces().silent();
  let lex_char = |c| char(c).skip(skip_spaces());

  let comma_list = sep_by(expr(), lex_char(','));
  let array = between(lex_char('['), lex_char(']'), comma_list);

  let pair = (lex_char('('),
              expr(),
              lex_char(','),
              expr(),
              lex_char(')'))
    .map(|(_, fst, _, snd, _)| Expr::Pair(Box::new(fst), Box::new(snd)));

  choice((
    attempt(integer().map(Expr::Integer)),
    decimal().map(Expr::Decimal),
    word.map(Expr::Id),
    array.map(Expr::Array),
    pair,
  ))
      .skip(skip_spaces())
}

fn decode(input: &str) -> Result<Expr, String> {
  match expr().parse(input) {
    Ok((expr, _)) => Ok(expr),
    Err(err) => Err(format!("{} in `{}`", err, input)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_integer() {
    let result = integer().parse("123").unwrap().0;
    assert_eq!(result, 123);
    let result = integer().parse("0").unwrap().0;
    assert_eq!(result, 0);
    if let Err(e) = integer().parse("120.") {
      assert!(true, "{} should be returned", e);
    } else {
      assert!(false);
    }
  }

  #[test]
  fn test_integer_part() {
    let result = integer_part().parse("123").unwrap().0;
    assert_eq!(result, 123.0);
    let result = integer_part().parse("0").unwrap().0;
    assert_eq!(result, 0.0);
    if let Err(e) = integer().parse("120.") {
      assert!(true, "{} should be returned", e);
    } else {
      assert!(false);
    }
  }

  #[test]
  fn test_decimal_part() {
    let result = decimal_part().parse("14").unwrap().0;
    assert_eq!(result, 0.14);
    let result = decimal_part().parse("99").unwrap().0;
    assert_eq!(result, 0.99);
    let result = decimal_part().parse("21").unwrap().0;
    assert_eq!(result, 0.21);
    if let Err(e) = integer().parse(".12") {
      assert!(true, "{} should be returned", e);
    } else {
      assert!(false);
    }

  }

  #[test]
  fn test_decimal() {
    let result = decimal().parse("123.14").unwrap().0;
    assert_eq!(result, 123.14);
    let result = decimal().parse("50.8").unwrap().0;
    assert_eq!(result, 50.8);
    let result = decimal().parse("123.").unwrap().0;
    assert_eq!(result, 123.0);
    let result = decimal().parse(".99").unwrap().0;
    assert_eq!(result, 0.99);
    let result = decimal().parse("0.100").unwrap().0;
    assert_eq!(result, 0.1);
    let result = decimal().parse("50.").unwrap().0;
    assert_eq!(result, 50.0);
  }

  #[test]
  fn test_expr() {
    let e = expr().parse("12").unwrap().0;
    assert_eq!(e, Expr::Integer(12));
    let e = expr().parse("123").unwrap().0;
    assert_eq!(e, Expr::Integer(123));
    let e = expr().parse(".43").unwrap().0;
    assert_eq!(e, Expr::Decimal(0.43));
    let e = expr().parse("50.").unwrap().0;
    assert_eq!(e, Expr::Decimal(50.0));
  }

  #[test]
  fn test_decode() {
    assert_eq!(decode("hello"), Ok(Expr::Id("hello".to_string())));
    assert_eq!(decode("123"), Ok(Expr::Integer(123)));
    assert_eq!(decode("1."), Ok(Expr::Decimal(1.0)));
    assert_eq!(decode(".123"), Ok(Expr::Decimal(0.123)));
    assert_eq!(decode(".99"), Ok(Expr::Decimal(0.99)));

    let result = decode("[[], (hello, world), 120, [rust, 3.14, .12]]").unwrap();
    let expr = Expr::Array(vec![
      Expr::Array(Vec::new()),
      Expr::Pair(
        Box::new(Expr::Id("hello".to_string())),
        Box::new(Expr::Id("world".to_string())),
      ),
      Expr::Integer(120),
      Expr::Array(vec![
        Expr::Id("rust".to_string()),
        Expr::Decimal(3.14),
        Expr::Decimal(0.12),
      ])
    ]);
    assert_eq!(result , expr);

    assert_eq!(decode("[hello, world]"), Ok(Expr::Array(vec![
      Expr::Id("hello".to_string()),
      Expr::Id("world".to_string()),
    ])));

    assert_eq!(decode("(hello, world)"), Ok(Expr::Pair(
      Box::new(Expr::Id("hello".to_string())),
      Box::new(Expr::Id("world".to_string())),
    )));

    assert_eq!(decode("(hello, [world, 123])"), Ok(Expr::Pair(
      Box::new(Expr::Id("hello".to_string())),
      Box::new(Expr::Array(vec![
        Expr::Id("world".to_string()),
        Expr::Integer(123),
      ])),
    )));
  }
}