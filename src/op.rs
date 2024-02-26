use std::thread::panicking;

use crate::expr::Expr;

pub(crate) struct Token {
  name: String,
  ticker: String,
  amount: usize
}

pub(crate) struct DealRequest {
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

impl From<&Expr> for DealRequest {
  fn from(expr: &Expr) -> Self {
    match expr {
      Expr::Dict(hm) => {
        let piece_cid = match hm.get("piece_cid") {
          Some(Expr::QuotedString(s)) => s.to_string(),
          _ => panic!("Missing piece_cid!")
        };
        let piece_size = match hm.get("piece_size") {
          Some(Expr::Integer(n)) => *n,
          _ => panic!("Missing piece_size!")
        };
        let verified_deal = match hm.get("verified_deal") {
          Some(Expr::Bool(b)) => *b,
          _ => panic!("Missing verified_deal!")
        };
        let label = match hm.get("label") {
          Some(Expr::QuotedString(s)) => s.to_string(),
          _ => panic!("Missing label!")
        };
        let start_epoch = match hm.get("start_epoch") {
          Some(Expr::Integer(n)) => *n,
          _ => panic!("Missing start_epoch!")
        };
        let end_epoch = match hm.get("end_epoch") {
          Some(Expr::Integer(n)) => *n,
          _ => panic!("Missing end_epoch!")
        };
        let storage_price_per_epoch = match hm.get("storage_price_per_epoch") {
          Some(Expr::Integer(n)) => *n,
          _ => panic!("Missing storage_price_per_epoch!")
        };
        let provider_collateral = match hm.get("provider_collateral") {
          Some(Expr::Integer(n)) => *n,
          _ => panic!("Missing provider_collateral!")
        };
        let extra_params_version = match hm.get("extra_params_version") {
          Some(Expr::Integer(n)) => *n,
          _ => panic!("Missing extra_params_version!")
        };
        DealRequest{
          piece_cid,
          piece_size: piece_size as u64,
          verified_deal,
          label,
          start_epoch: start_epoch as i64,
          end_epoch: end_epoch as i64,
          storage_price_per_epoch,
          provider_collateral,
          extra_params_version: extra_params_version as u64
        }
      },
      _ => panic!("Not a Expr::Dict!")
    }
  }
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

// Some dummy implementation for now.
pub(crate) fn propose(expr: Option<Expr>) {
  if let Some(Expr::Dict(hm)) = expr {
    if let Some(deal_expr) = hm.get("deal_request") {
      propose_inner(deal_expr.into());
    } else {
      panic!("Missing argument for propose!");
    }
  } else {
    panic!("Wrong argument type for propose!");
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

fn propose_inner(deal_request: DealRequest) {
  println!("Proposing deal to {}!", deal_request.piece_cid);
}