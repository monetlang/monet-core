use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = PathBuf::from(out_dir).join("gen.rs");
  // let content = fs::read_to_string("src/expression.mt").unwrap();
  // let ast = parser::expression_parser().parse(content.trim()).unwrap().0;
  let expr = "Expr::BinOp { op: '+', lhs: Box::new(Expr::Number(99.01)), rhs: Box::new(Expr::Number(3.14)) }";
  // fs::write(&dest_path, format!("const DATA: Expr = {};", generated))
  //   .expect("Unable to write file");
  fs::write(&dest_path, format!("pub fn gen_expr() -> Expr {{ {} }}", expr)).unwrap();
}