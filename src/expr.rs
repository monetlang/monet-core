use std::collections::HashMap;
use combine::parser;
use combine::attempt;
use combine::parser::char::string;
use combine::parser::choice::or;
use combine::{not_followed_by, optional};
use combine::parser::char::{spaces, newline, digit, char, letter};
use combine::{between, choice, many1, sep_by, ParseError, Parser};
use combine::stream::Stream;

use crate::op::*;

// #[derive(Debug, PartialEq)]
// pub struct Token(String, String, usize);

// #[derive(Debug, PartialEq)]
// pub enum Contract {
//   Stmt(Vec<(Vec<Event>, Vec<Op>)>),
//   Close,
// }

// #[derive(Debug, PartialEq)]
// pub struct Op {
//   f: fn(Option<Box<Expr>>) -> (),
//   arg: Option<Expr>,
// }

// #[derive(Debug, PartialEq)]
// pub enum Event {
//   Deposit {
//     from: String,
//     token: Token,
//   },
//   Pay {
//     to: String,
//     token: Token,
//   },
//   DealProposalCreated,
//   DealPublished,
//   DealActivated,
//   DealTerminated,
// }

#[derive(Debug, PartialEq)]
pub struct EventOp {
  name: String,
  event: Expr,
}

pub type Ops = Vec<Op>;

#[derive(Debug, PartialEq)]
pub struct Op {
  f: fn(Option<Expr>) -> (),
  arg: Option<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
  Id(String),
  Decimal(f64),
  Integer(usize),
  QuotedString(String),
  Atom(String),
  Dict(HashMap<String, Expr>),
  Array(Vec<Expr>),
  Pair(Box<Expr>, Box<Expr>),
  Event{
    name: String,
    args: HashMap<String, Expr>
  },
  Token{
    name: String,
    ticker: String,
    amount: usize
  }
}

parser!{
    fn expr[I]()(I) -> Expr
    where [I: Stream<Token = char>]
  {
    expr_()
  }
}

// fn close<I>() -> impl Parser<I, Output = Contract>
//   where I: Stream<Token = char>,
//         I::Error: ParseError<I::Token, I::Range, I::Position>,
// {
//   spaces().with(string("close")).map(|_| Contract::Close)
// }

fn op<T>() -> impl Parser<T, Output = Op>
  where T: Stream<Token = char>,
        T::Error: ParseError<T::Token, T::Range, T::Position>,
{
  let kw = choice(
    (
      string("pay"),
    )
  );

  (kw, optional(spaces()), dict()).map(|(_op, _, args)|
      Op{ f: pay, arg: Some(Expr::Dict(args)) })
  // spaces()
  //   .with(string("pay"))
  //   .and(spaces().with(expr()))
  //   .map(|(_op_name, expr)| Op{
  //       f: pay,
  //       arg: Some(expr),
  //     }
  //   )
}

fn when<T>() -> impl Parser<T, Output = EventOp>
  where T: Stream<Token = char>,
        T::Error: ParseError<T::Token, T::Range, T::Position>,
{
  spaces()
    .with(string("when"))
    .and(spaces().with(event()))
    .map(|(name, event)| EventOp{
      name: name.to_string(),
      event,
    })
}

fn event<I>() -> impl Parser<I, Output = Expr>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  let ps = choice(
    (
      string("Deposit"),
      string("Pay"),
      string("DealProposalCreated"),
      string("DealPublished"),
      string("DealActivated"),
      string("DealTerminated"),
    )
  );

  (ps, optional(spaces()), dict()).map(|(evt, _, args)|
      Expr::Event{ name: evt.to_string(), args })
}

fn atom<I>() -> impl Parser<I, Output = Expr>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  char(':').with(many1(or(letter(), digit())).map(Expr::Atom))
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

fn quoted_string<I>() -> impl Parser<I, Output = String>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  between(char('"'), char('"'), many1(or(letter(), digit())))
    .map(|chars: Vec<char>| chars.into_iter().collect())
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

fn dict<I>() -> impl Parser<I, Output = HashMap<String, Expr>>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  let skip_spaces = || spaces().silent();
  let lex_char = |c| char(c).skip(skip_spaces());
  let pair =
    or(word(), quoted_string())
      .skip(spaces().with(lex_char(':')))
      .and(spaces().with(expr()))
      .map(|(key, value): (String, Expr)| (key, value));

  let comma_list =
    sep_by(pair, lex_char(','));


  let dict = between(
    char('{').skip(spaces()),
    spaces().with(char('}')),
    comma_list
  );
  dict.map(|pairs: Vec<(String, Expr)>| {
    let mut dict: HashMap<String, Expr> = HashMap::new();
    for (key, value) in pairs {
      dict.insert(key, value);
    }
    dict
  })
}

fn word<I>() -> impl Parser<I, Output = String>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  many1(choice((letter(), digit(), char('_')))).map(|chars: String| chars)
}

fn expr_<'a, I>() -> impl Parser<I, Output = Expr>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{

  let skip_spaces = || spaces().silent();
  let lex_char = |c| char(c).skip(skip_spaces());

  let comma_list =
    sep_by(expr(), lex_char(','));

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
    event(),
    word().map(Expr::Id),
    dict().map(Expr::Dict),
    quoted_string().map(Expr::QuotedString),
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
  use std::hash::Hash;

use super::*;
  use crate::op::pay;

  #[test]
  fn test_word() {
    let result = word().parse("hello").unwrap().0;
    assert_eq!(result, "hello".to_string());
    let result = word().parse("world_tour").unwrap().0;
    assert_eq!(result, "world_tour".to_string());
    let result = word().parse("hey12").unwrap().0;
    assert_eq!(result, "hey12".to_string());
  }

  #[test]
  fn test_atom() {
    let result = atom().parse(":hello").unwrap().0;
    assert_eq!(result, Expr::Atom("hello".to_string()));
    let result = atom().parse(":world").unwrap().0;
    assert_eq!(result, Expr::Atom("world".to_string()));
    let result = atom().parse(":hey12").unwrap().0;
    assert_eq!(result, Expr::Atom("hey12".to_string()));
  }

  #[test]
  fn test_dict() {
    let result = dict().parse("{ hello: 123, \"world\" : 50 }").unwrap().0;
    let mut expected = HashMap::new();
    expected.insert("hello".to_string(), Expr::Integer(123));
    expected.insert("world".to_string(), Expr::Integer(50));
    assert_eq!(result, expected);
    let result = dict().parse("{hello: 123, world: 50, hey_12: 0.12}").unwrap().0;
    let mut expected = HashMap::new();
    expected.insert("hello".to_string(), Expr::Integer(123));
    expected.insert("world".to_string(), Expr::Integer(50));
    expected.insert("hey_12".to_string(), Expr::Decimal(0.12));
    assert_eq!(result, expected);
  }

  #[test]
  fn test_multiline_dict() {
    let e = expr().parse(r#"{
      hello: 123,
      "world": 30.
    }"#).unwrap().0;
    let mut dict = HashMap::new();
    dict.insert("hello".to_string(), Expr::Integer(123));
    dict.insert("world".to_string(), Expr::Decimal(30.0));
    let expected = Expr::Dict(dict);
    assert_eq!(e, expected);
  }

  #[test]
  fn test_nested_dict() {
    let e = expr().parse(r#"{
      hello: 123,
      "world": 30.,
      key: {
        value: 50,
        "nested": 100
      }
    }"#).unwrap().0;
    let mut nested = HashMap::new();
    nested.insert("value".to_string(), Expr::Integer(50));
    nested.insert("nested".to_string(), Expr::Integer(100));
    let mut dict = HashMap::new();
    dict.insert("hello".to_string(), Expr::Integer(123));
    dict.insert("world".to_string(), Expr::Decimal(30.0));
    dict.insert("key".to_string(), Expr::Dict(nested));
    let expected = Expr::Dict(dict);
    assert_eq!(e, expected);
  }

  #[test]
  fn test_op() {
    let e = op().parse(r#"pay {
      to: "addressA",
      token: {
        name: "world",
        ticker: "WRLD",
        amount: 123
      }
    }"#).unwrap().0;
    let mut inner = HashMap::new();
    inner.insert("to".to_string(), Expr::QuotedString("addressA".to_string()));
    let mut inner_token = HashMap::new();
    inner_token.insert("name".to_string(), Expr::QuotedString("world".to_string()));
    inner_token.insert("ticker".to_string(), Expr::QuotedString("WRLD".to_string()));
    inner_token.insert("amount".to_string(), Expr::Integer(123));
    inner.insert("token".to_string(), Expr::Dict(inner_token));

    let arg = Expr::Dict(inner);
    assert_eq!(e, Op{ f: pay, arg: Some(arg) });
  }

  #[test]
  fn test_when() {
    let e = when().parse(r#"when Deposit {
      from: "addressA",
      token: {
        name: "world",
        ticker: "WRLD",
        amount: 123
      }
    }"#).unwrap().0;
    let mut args = HashMap::new();
    args.insert("from".to_string(), Expr::QuotedString("addressA".to_string()));
    let mut token = HashMap::new();
    token.insert("name".to_string(), Expr::QuotedString("world".to_string()));
    token.insert("ticker".to_string(), Expr::QuotedString("WRLD".to_string()));
    token.insert("amount".to_string(), Expr::Integer(123));
    args.insert("token".to_string(), Expr::Dict(token));
    let event = Expr::Event{ name: "Deposit".to_string(), args };
    let event_op = EventOp{ name: "when".to_string(), event };
    assert_eq!(e, event_op);
  }

  #[test]
  fn test_event() {
    let e = event().parse(r#"Deposit {
      from: "addressA",
      token: {
        name: "world",
        ticker: "WRLD",
        amount: 123
      }
    }"#).unwrap().0;
    let mut args = HashMap::new();
    args.insert("from".to_string(), Expr::QuotedString("addressA".to_string()));
    let mut token = HashMap::new();
    token.insert("name".to_string(), Expr::QuotedString("world".to_string()));
    token.insert("ticker".to_string(), Expr::QuotedString("WRLD".to_string()));
    token.insert("amount".to_string(), Expr::Integer(123));
    args.insert("token".to_string(), Expr::Dict(token));
    let expected = Expr::Event{ name: "Deposit".to_string(), args };
    assert_eq!(e, expected);
  }

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
  fn test_quoted_string() {
    let result = quoted_string().parse("\"hello\"").unwrap().0;
    assert_eq!(result, "hello");
    let result = quoted_string().parse("\"world\"").unwrap().0;
    assert_eq!(result, "world");
    let result = quoted_string().parse("\"hey12\"").unwrap().0;
    assert_eq!(result, "hey12");
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
    let e = expr().parse("foo").unwrap().0;
    assert_eq!(e, Expr::Id("foo".to_string()));
    let e = expr().parse("\"hello\"").unwrap().0;
    assert_eq!(e, Expr::QuotedString("hello".to_string()));
    let e = expr().parse(r#"{
      hello: 123,
      "world": 50.,
      key: "value"
    }"#).unwrap().0;
    let mut dict = HashMap::new();
    dict.insert("hello".to_string(), Expr::Integer(123));
    dict.insert("world".to_string(), Expr::Decimal(50.0));
    dict.insert("key".to_string(), Expr::QuotedString("value".to_string()));
    let expected = Expr::Dict(dict);
    assert_eq!(e, expected);
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