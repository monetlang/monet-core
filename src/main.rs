mod expr;
mod op;
mod parser;
mod ast;
mod compiler;
mod integration;

use std::io;
use std::collections::HashMap;
use combine::Parser;
use crate::parser::expression_parser;
use crate::ast::Function;
use crate::compiler::Compiler;
use inkwell::context::Context;
use crate::compiler::set_compiler_hook;

fn main() {
    loop {
        print!("monet> ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let expr = expression_parser().parse(input.as_str()).unwrap().0;
                let ctx = Context::create();
                let mut compiler = compiler::create_compiler!(&ctx, "tmp");
                set_compiler_hook(&mut compiler);
                let result = compiler.compile_expr(&expr).unwrap();
                println!("{:?}", result.to_string());
            },
            Err(err) => println!("Could not parse input: {}", err),
        }

    }
}
