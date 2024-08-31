use std::env;
use std::fs;
use std::path::PathBuf;
use parser::parser::expression_parser;
use parser::ast::Expr;
use combine::Parser;

fn main() {
  let file_path = std::env::var("TARGET_FILE").expect("Missing TARGET_FILE env variable");
  let path = file_path.clone();

  /////////// Tell Rust to re-run this build script //////////////
  println!("cargo:rerun-if-changed={}", file_path);
  println!("cargo:rerun-if-env-changed=TARGET_FILE");
  ////////////////////////////////////////////////////////////////

  let out_dir = env::var("OUT_DIR").unwrap();
  let dest_path = PathBuf::from(out_dir).join("gen.rs");
  let content = fs::read_to_string(file_path).unwrap();
  let ast: Expr = expression_parser().parse(content.trim()).unwrap().0;
  fs::write(&dest_path, format!("
    const TARGET_FILE: &str = \"{}\";
    const EXPR: &str = \"{}\";
    pub fn gen_expr() -> Expr {{ {} }}
    ", path, content, ast)).unwrap();
}