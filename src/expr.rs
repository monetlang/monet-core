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

pub type Stmt = (EventOp, Ops);

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
  Bool(bool),
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
  },
  DealRequest {
    piece_cid: String,
    piece_size: u64,
    verified_deal: bool,
    label: String,
    start_epoch: i64,
    end_epoch: i64,
    storage_price_per_epoch: usize,
    provider_collateral: usize,
    extra_params_version: u64,
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
fn ops<I>() -> impl Parser<I, Output = Ops>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  many(((
    spaces(),
    optional(string("then")),
    spaces(),
    op(),
  )).map(|(_, _, _, op)| op))
}

fn stmt<T>() -> impl Parser<T, Output = Stmt>
  where T: Stream<Token = char>,
        T::Error: ParseError<T::Token, T::Range, T::Position>,
{
  (attempt(when()), ops())
}

fn op<T>() -> impl Parser<T, Output = Op>
  where T: Stream<Token = char>,
        T::Error: ParseError<T::Token, T::Range, T::Position>,
{
  let kw = choice((
    attempt(string("pay")),
    attempt(string("propose")),
  ));

  (kw, optional(spaces()), dict())
    .map(|(op, _, args)| {
      match op {
        "pay" => Op{ f: pay, arg: Some(Expr::Dict(args)) },
        "propose" => Op{ f: propose, arg: Some(Expr::Dict(args)) },
        _ => todo!("Don't know what to do"),
      }
    })
}

fn when<T>() -> impl Parser<T, Output = EventOp>
  where T: Stream<Token = char>,
        T::Error: ParseError<T::Token, T::Range, T::Position>,
{
  (
    string("when"),
    spaces(),
    event(),
  ).map(|(name, _, event)| EventOp{ name: name.to_string(), event })
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

fn boolean<I>() -> impl Parser<I, Output = bool>
  where I: Stream<Token = char>,
        I::Error: ParseError<I::Token, I::Range, I::Position>,
{
  choice((
    string("true").map(|_| true),
    string("false").map(|_| false),
  ))
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
    attempt(boolean().map(Expr::Bool)),
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
  fn test_boolean() {
    let result = boolean().parse("true").unwrap().0;
    assert_eq!(result, true);
    let result = boolean().parse("false").unwrap().0;
    assert_eq!(result, false);
    if let Err(_) = boolean().parse("no") {
      assert!(true);
    } else {
      assert!(false);
    }
  }

  #[test]
  fn test_dict() {
    let result = dict().parse(r#"{ 
      hello: 123,
      "world" : 50,
      verified: true,
      fruits: ["apple", "orange", "banana"]
    }"#).unwrap().0;
    let mut expected = HashMap::new();
    expected.insert("hello".to_string(), Expr::Integer(123));
    expected.insert("world".to_string(), Expr::Integer(50));
    expected.insert("verified".to_string(), Expr::Bool(true));
    expected.insert("fruits".to_string(), Expr::Array(vec![
      Expr::QuotedString("apple".to_string()),
      Expr::QuotedString("orange".to_string()),
      Expr::QuotedString("banana".to_string()),
    
    ]));
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
        "nested": 100,
        ozone: 30,
        people: false

      }
    }"#).unwrap().0;
    let mut nested = HashMap::new();
    nested.insert("ozone".to_string(), Expr::Integer(30));
    nested.insert("people".to_string(), Expr::Bool(false));
    nested.insert("value".to_string(), Expr::Integer(50));
    nested.insert("nested".to_string(), Expr::Integer(100));
    let mut dict = HashMap::new();
    dict.insert("hello".to_string(), Expr::Integer(123));
    dict.insert("world".to_string(), Expr::Decimal(30.0));
    dict.insert("key".to_string(), Expr::Dict(nested));
    let expected = Expr::Dict(dict);

    assert_eq!(e, expected);

    let e = expr().parse(r#"{
      deal_request: {
        piece_cid: "QmX",
        piece_size: 123,
        verified_deal: true,
        label: "label",
        start_epoch: 123,
        end_epoch: 123,
        storage_price_per_epoch: 123,
        provider_collateral: 123,
        extra_params_version: 123
      }
    }"#).unwrap().0;

    let mut nested = HashMap::new();
    nested.insert("piece_cid".to_string(), Expr::QuotedString("QmX".to_string()));
    nested.insert("piece_size".to_string(), Expr::Integer(123));
    nested.insert("verified_deal".to_string(), Expr::Bool(true));
    nested.insert("label".to_string(), Expr::QuotedString("label".to_string()));
    nested.insert("start_epoch".to_string(), Expr::Integer(123));
    nested.insert("end_epoch".to_string(), Expr::Integer(123));
    nested.insert("storage_price_per_epoch".to_string(), Expr::Integer(123));
    nested.insert("provider_collateral".to_string(), Expr::Integer(123));
    nested.insert("extra_params_version".to_string(), Expr::Integer(123));

    let mut dict = HashMap::new();
    dict.insert("deal_request".to_string(), Expr::Dict(nested));
    let expected = Expr::Dict(dict);
    assert_eq!(e, expected);
  }

  #[test]
  fn test_pay_op() {
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
  fn test_propose_op() {
    let e = op().parse(r#"propose {
      deal_request: {
        piece_cid: "QmX",
        piece_size: 123,
        verified_deal: true,
        label: "label",
        start_epoch: 123,
        end_epoch: 123,
        storage_price_per_epoch: 123,
        provider_collateral: 123,
        extra_params_version: 123
      }
    }"#).unwrap().0;

    let mut deal_request = HashMap::new();
    deal_request.insert("piece_cid".to_string(), Expr::QuotedString("QmX".to_string()));
    deal_request.insert("piece_size".to_string(), Expr::Integer(123));
    deal_request.insert("verified_deal".to_string(), Expr::Bool(true));
    deal_request.insert("label".to_string(), Expr::QuotedString("label".to_string()));
    deal_request.insert("start_epoch".to_string(), Expr::Integer(123));
    deal_request.insert("end_epoch".to_string(), Expr::Integer(123));
    deal_request.insert("storage_price_per_epoch".to_string(), Expr::Integer(123));
    deal_request.insert("provider_collateral".to_string(), Expr::Integer(123));
    deal_request.insert("extra_params_version".to_string(), Expr::Integer(123));

    let mut inner = HashMap::new();
    inner.insert("deal_request".to_string(), Expr::Dict(deal_request));

    let arg = Expr::Dict(inner);
    assert_eq!(e, Op{ f: propose, arg: Some(arg) });
  }

  #[test]
  fn test_ops() {
    let e = ops().parse(r#"pay {
      to: "addressA",
      token: {
        name: "world",
        ticker: "WRLD",
        amount: 123
      }
    } then
    pay {
      to: "addressB",
      token: {
        name: "mars",
        ticker: "MARS",
        amount: 100
      }
    }
    pay {
      to: "addressC",
      token: {
        name: "jupiter",
        ticker: "JUP",
        amount: 20
      }
    }"#).unwrap().0;
    let mut inner = HashMap::new();
    inner.insert("to".to_string(), Expr::QuotedString("addressA".to_string()));
    let mut inner_token = HashMap::new();
    inner_token.insert("name".to_string(), Expr::QuotedString("world".to_string()));
    inner_token.insert("ticker".to_string(), Expr::QuotedString("WRLD".to_string()));
    inner_token.insert("amount".to_string(), Expr::Integer(123));
    inner.insert("token".to_string(), Expr::Dict(inner_token));

    // let arg = Expr::Dict(inner);
    let op1 = Op{ f: pay, arg: Some(Expr::Dict(inner)) };

    let mut inner = HashMap::new();
    inner.insert("to".to_string(), Expr::QuotedString("addressB".to_string()));
    let mut inner_token = HashMap::new();
    inner_token.insert("name".to_string(), Expr::QuotedString("mars".to_string()));
    inner_token.insert("ticker".to_string(), Expr::QuotedString("MARS".to_string()));
    inner_token.insert("amount".to_string(), Expr::Integer(100));
    inner.insert("token".to_string(), Expr::Dict(inner_token));

    let op2 = Op{ f: pay, arg: Some(Expr::Dict(inner)) };

    let mut inner = HashMap::new();
    inner.insert("to".to_string(), Expr::QuotedString("addressC".to_string()));
    let mut inner_token = HashMap::new();
    inner_token.insert("name".to_string(), Expr::QuotedString("jupiter".to_string()));
    inner_token.insert("ticker".to_string(), Expr::QuotedString("JUP".to_string()));
    inner_token.insert("amount".to_string(), Expr::Integer(20));
    inner.insert("token".to_string(), Expr::Dict(inner_token));

    let op3 = Op{ f: pay, arg: Some(Expr::Dict(inner)) };

    assert_eq!(e, vec![op1, op2, op3]);
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
  fn test_stmt() {
    let e = stmt().parse(r#"when Deposit {
      from: "addressA",
      token: {
        name: "world",
        ticker: "WRLD",
        amount: 123
      }
    } then
      pay {
        to: "addressB",
        token: {
          name: "mars",
          ticker: "MARS",
          amount: 100
        }
      } then
      pay {
        to: "addressC",
        token: {
          name: "jupiter",
          ticker: "JUP",
          amount: 20
        }
      } then
      propose {
        deal_request: {
          piece_cid: "Qmx",
          piece_size: 123,
          verified_deal: true,
          label: "label",
          start_epoch: 123,
          end_epoch: 123,
          storage_price_per_epoch: 123,
          provider_collateral: 123,
          extra_params_version: 123
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

    let mut pargs = HashMap::new();
    pargs.insert("to".to_string(), Expr::QuotedString("addressB".to_string()));
    let mut token = HashMap::new();
    token.insert("name".to_string(), Expr::QuotedString("mars".to_string()));
    token.insert("ticker".to_string(), Expr::QuotedString("MARS".to_string()));
    token.insert("amount".to_string(), Expr::Integer(100));

    pargs.insert("token".to_string(), Expr::Dict(token));

    let op1 = Op{ f: pay, arg: Some(Expr::Dict(pargs)) };

    let mut pargs = HashMap::new();
    pargs.insert("to".to_string(), Expr::QuotedString("addressC".to_string()));
    let mut token = HashMap::new();
    token.insert("name".to_string(), Expr::QuotedString("jupiter".to_string()));
    token.insert("ticker".to_string(), Expr::QuotedString("JUP".to_string()));
    token.insert("amount".to_string(), Expr::Integer(20));

    pargs.insert("token".to_string(), Expr::Dict(token));

    let op2 = Op{ f: pay, arg: Some(Expr::Dict(pargs)) };

    let mut pargs = HashMap::new();
    let mut deal_request = HashMap::new();
    deal_request.insert("piece_cid".to_string(), Expr::QuotedString("Qmx".to_string()));
    deal_request.insert("piece_size".to_string(), Expr::Integer(123));
    deal_request.insert("verified_deal".to_string(), Expr::Bool(true));
    deal_request.insert("label".to_string(), Expr::QuotedString("label".to_string()));
    deal_request.insert("start_epoch".to_string(), Expr::Integer(123));
    deal_request.insert("end_epoch".to_string(), Expr::Integer(123));
    deal_request.insert("storage_price_per_epoch".to_string(), Expr::Integer(123));
    deal_request.insert("provider_collateral".to_string(), Expr::Integer(123));
    deal_request.insert("extra_params_version".to_string(), Expr::Integer(123));
    pargs.insert("deal_request".to_string(), Expr::Dict(deal_request));

    let op3 = Op{ f: propose, arg: Some(Expr::Dict(pargs)) };

    let expected = (event_op, vec![op1, op2, op3]);
    assert_eq!(e, expected);
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