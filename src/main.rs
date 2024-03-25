mod expr;
mod op;
mod parser;
mod ast;
mod compiler;
mod integration;
mod backend;

use std::fs::File;
use std::path::Path;
use std::{env, fs, io};
use std::collections::HashMap;
use combine::Parser;
use crate::parser::expression_parser;
use crate::ast::Function;
use crate::compiler::Compiler;
use inkwell::context::Context;
use crate::compiler::set_compiler_hook;

fn build_llvm_to_wasm_module(input: String, output: String) {
    let ctx = Context::create();
    let mut compiler = compiler::create_compiler!(&ctx, "tmp");
    set_compiler_hook(&mut compiler);
    let expr = expression_parser().parse("fn add(a: i32, b: i32) -> i32 { a + b }").unwrap().0;
    let result = compiler.compile_expr(&expr).unwrap();
    println!("{:?}", result.to_string());
}


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("len: {}", args.len());
    println!("args[0]: {}", args[0]);
    println!("args[1]: {}", args[1]);
    println!("args[2]: {}", args[2]);
    println!("args[3]: {}", args[3]);
    if args.len() > 3 && vec!["-c", "--compile"].contains(&args[1].as_str()) {
        let input = args[2].to_string();
        let output = args[3].as_str();

        let txt = fs::read_to_string(input)
            .expect("Should have been able to read the file");

        let expr = expression_parser().parse(txt.as_str()).unwrap().0;
        let ctx = Context::create();
        let mut compiler = compiler::create_compiler!(&ctx, "tmp");
        set_compiler_hook(&mut compiler);
        let _result = compiler.compile_expr(&expr).unwrap();

        // let _output_file = File::create(output.as_str())
        //     .expect("Should be able to create file");

        compiler.module.print_to_file(Path::new(output)).unwrap();
        // compiler.module.write_bitcode_to_file(&output_file, true, true);
    } else {
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
}
