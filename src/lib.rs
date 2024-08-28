include!(concat!(env!("OUT_DIR"), "/gen.rs"));

mod ast;
mod backend;
pub mod parser;

use ast::ExprWrapper;
use wasm_bindgen::prelude::*;
use crate::ast::Expr;
use crate::parser::expression_parser;
use crate::backend::wasm::translate_to_rust;
use combine::Parser;
use std::env;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[wasm_bindgen]
pub fn run() {
  alert(&format!("hello from wasm"));
}

pub fn translate_to_str(e: Expr) -> String {
  translate_to_rust(e)
}

// #[wasm_bindgen]
pub fn read_monet_file(path: &str) -> Expr {
  let contents = std::fs::read_to_string(path)
    .expect("Something went wrong reading the file");

  let expr = expression_parser().parse(contents.trim()).unwrap().0;
  expr
}

#[macro_export]
macro_rules! read_monet {
  ($path:literal) => {
    read_monet_file($path)
  };
}

#[macro_export]
macro_rules! create_main {
    ($body:block) => {
        #[wasm_bindgen(start)]
        pub fn main() {
          alert("WASM hello!");
          alert($body);
        }
    };
}

create_main!({
  &gen_expr().to_string()
});

// #[wasm_bindgen(start)]
// pub fn main() {
//   alert("WASM test!");
//   alert(&DATA.to_string());
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_file() {
    let path = "src/expression.mt";
    let expr = read_monet_file(path);
    let expect = Expr::BinOp {
      op: '+',
      lhs: Box::new(Expr::Number(1200.21)),
      rhs: Box::new(Expr::Number(54.012)),
    };
    assert_eq!(expr, expect);
    let s = expr.wrap().to_string();
    assert_eq!(s, "1200.21 + 54.012");
  }

  #[test]
  fn test_translate_to_str() {
    let path = "src/expression.mt";
    let expr = read_monet_file(path).to_string();
    let expect = "1200.21 + 54.012";
    assert_eq!(expr, expect);
  }
}


// #[wasm_bindgen(start)]
// pub fn main() {
//   alert(&format!("hello from Rust!"));
//   let e = translate_to_str(Expr::BinOp {
//     op: '+',
//     lhs: Box::new(Expr::Number(42.0)),
//     rhs: Box::new(Expr::Number(3.14)),
//   });
//   alert(&e);
// }

