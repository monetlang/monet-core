mod ast;
mod backend;

use std::error::Error;
use wasm_bindgen::prelude::*;
use crate::ast::Expr;
use crate::backend::wasm::translate_to_rust;

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

macro_rules! create_main {
    ($body:block) => {
        #[wasm_bindgen(start)]
        pub fn main() {
          alert(&format!("hello from Rust!"));
          alert($body);
        }    
    };
}

create_main!({
  &translate_to_str(Expr::BinOp {
    op: '+',
    lhs: Box::new(Expr::Number(4.110)),
    rhs: Box::new(Expr::Number(3.1418)),
  })
});


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

