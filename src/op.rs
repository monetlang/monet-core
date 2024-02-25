use std::thread::panicking;

use crate::expr::Expr;

pub(crate) struct Token {
  name: String,
  ticker: String,
  amount: usize
}

impl From<&Expr> for Token {
  fn from(expr: &Expr) -> Self {
    match expr {
      Expr::Token{name, ticker, amount} => {
        Token{
          name: name.to_string(),
          ticker: ticker.to_string(),
          amount: *amount
        }
      },
      _ => panic!("Not a Expr::Token!")
    }
  }
}

pub(crate) fn pay(expr: Option<Expr>) {
  if let Some(Expr::Dict(hm)) = expr {
    if let (
      Some(Expr::QuotedString(to)),
      Some(token_expr)
    )  = (hm.get("to"), hm.get("token")) {
      pay_inner(to.to_string(), token_expr.into());
    } else {
      panic!("Missing argument for pay!");
    }
  } else {
    panic!("Wrong argument type for pay!");
  }
}

// Just print stuff for now.
fn pay_inner(to: String, token: Token) {
    println!("Sending token {} to {}!", token.name, to);
}