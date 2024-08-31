#![allow(unused)]
#![allow(dead_code)]

mod expr;
mod op;
mod parser;
mod ast;
// mod wasm;
// mod compiler;
// mod integration;
mod backend;

use std::fs::File;
use std::path::Path;
use std::{env, fs, io};
use std::collections::HashMap;
use combine::Parser;

// use inkwell::llvm_sys::{target, target_machine};
// use inkwell::targets::{CodeModel, InitializationConfig, RelocMode, Target, TargetTriple, TargetMachine, FileType};
// use inkwell::values::GenericValue;
// use inkwell::{data_layout, OptimizationLevel};
use crate::parser::{expression_parser, parse_definition};
use crate::ast::Function;
// use crate::compiler::Compiler;
// use inkwell::context::Context;
// use inkwell::passes::{PassManager, PassManagerSubType};
// use crate::compiler::set_compiler_hook;

// fn print_llvm_add_module() {
//     let context = Context::create();
//     let module = context.create_module("add_module");

//     let builder = context.create_builder();

//     let i32_type = context.i32_type();

//     let function_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
//     let function = module.add_function("addition", function_type, None);

//     let entry_block = context.append_basic_block(function, "entry");
//     let recursive_block = context.append_basic_block(function, "recursive");
//     let base_case_block = context.append_basic_block(function, "base_case");
//     let after_block = context.append_basic_block(function, "after");

//     builder.position_at_end(entry_block);

//     let a = function.get_first_param().unwrap().into_int_value();
//     let b = function.get_nth_param(1).unwrap().into_int_value();

//     let result = builder.build_int_add(a, b, "result");

//     builder.build_return(Some(&result.unwrap()));

//     builder.position_at_end(after_block);

//     module.print_to_stderr();

//     // Create an execution engine to run the function (if desired)
//     let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

//     // Run the function
//     let mut n = 100;
//     let mut m = 20;
//     let ng = i32_type.create_generic_value(n as u64, true);
//     let mg = i32_type.create_generic_value(m as u64, true);

//     let result = unsafe {
//         let faddr = execution_engine
//             .get_function_address("addition")
//             .unwrap();
//         let function: extern "C" fn(i32, i32) -> i32 = std::mem::transmute(faddr);
//         function(n, m)
//     };

//     println!("Add({}, {}) = {}", n, m, result);
//     // println!("Add({}, {}) = {}", n, m, result.as_int(true));
// }

// fn print_llvm_add_one_module() {
//     let context = Context::create();
//     let module = context.create_module("add_one_module");

//     let builder = context.create_builder();

//     let i32_type = context.i32_type();

//     let function_type = i32_type.fn_type(&[i32_type.into()], false);
//     let function = module.add_function("add_one", function_type, None);

//     let entry_block = context.append_basic_block(function, "entry");
//     let after_block = context.append_basic_block(function, "after");

//     builder.position_at_end(entry_block);

//     let a = function.get_first_param().unwrap().into_int_value();

//     let result = builder.build_int_add(a, i32_type.const_int(1, false), "result");

//     builder.build_return(Some(&result.unwrap()));

//     builder.position_at_end(after_block);

//     module.print_to_stderr();

//     // Create an execution engine to run the function (if desired)
//     let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

//     // Run the function
//     let mut n = 100;

//     let result = unsafe {
//         let faddr = execution_engine
//             .get_function_address("add_one")
//             .unwrap();
//         let function: extern "C" fn(i32) -> i32 = std::mem::transmute(faddr);
//         function(n)
//     };

//     println!("AddOne({}) = {}", n, result);
// }

// fn build_add_one_wasm() {
//     // Create a context
//     let context = Context::create();
//     let module = context.create_module("wasm_module");
//     let builder = context.create_builder();

//     // Define the i32 type for our function signature
//     let i32_type = context.i32_type();

//     // Create the function signature (i32, i32) -> i32
//     let fn_type = i32_type.fn_type(&[i32_type.into()], false);

//     // Add the function to the module
//     let function = module.add_function("add_one", fn_type, None);

//     // Create a basic block and set the builder position
//     let basic_block = context.append_basic_block(function, "entry");
//     builder.position_at_end(basic_block);

//     // Get the function parameters
//     let x = function.get_nth_param(0).unwrap().into_int_value();

//     // Add the parameters and return the result
//     let sum = builder.build_int_add(x, i32_type.const_int(1, false), "sum");
//     builder.build_return(Some(&sum.unwrap()));

//     // Print the module's IR for debugging
//     module.print_to_stderr();

//     Target::initialize_webassembly(&InitializationConfig::default());

//     let target_triple = TargetTriple::create("wasm32-unknown-unknown");
//     module.set_triple(&target_triple);

//     // Create a target machine
//     let target = Target::from_triple(&target_triple).unwrap();
//     let target_machine = target
//         .create_target_machine(
//             &target_triple,
//             "generic",
//             TargetMachine::get_host_cpu_features().to_str().unwrap(),
//             OptimizationLevel::Default,
//             RelocMode::Default,
//             CodeModel::Default,
//         )
//         .unwrap();
    
//     // Optional: Save the module to a .wasm file
//     let output_file = "playground/src/lib/add_one_inkwell.wasm";

//     // Print module to string.
//     module.print_to_string();

//     // Emit the module as a WASM object file
//     target_machine.write_to_file(
//         &module,
//         inkwell::targets::FileType::Object,
//         Path::new(output_file),
//     ).unwrap();
//     // module.write_bitcode_to_path(std::path::Path::new(output_file));
//     // println!("WASM module written to: {}", output_file);
// }

// fn print_llvm_fib_module() {
//     let context = Context::create();
//     let module = context.create_module("fib_module");

//     // Add functions and IR here using Inkwell API
//     // ...

//     let builder = context.create_builder();

//     // let pass_manager_builder = PassManagerSubType::
//     // pass_manager_builder.set_optimization_level(OptimizationLevel::Default);

//     let pass_manager = PassManager::create(());
//     // pass_manager_builder.populate_module_pass_manager(&pass_manager);

//     pass_manager.run_on(&module);

//     // Define the i32 type (for our Fibonacci function)
//     let i32_type = context.i32_type();

//     // Create the Fibonacci function signature (i32 -> i32)
//     let function_type = i32_type.fn_type(&[i32_type.into()], false);
//     let function = module.add_function("fibonacci", function_type, None);

//     // Create basic blocks for entry, recursive case, and base case
//     let entry_block = context.append_basic_block(function, "entry");
//     let recursive_block = context.append_basic_block(function, "recursive");
//     let base_case_block = context.append_basic_block(function, "base_case");
//     let after_block = context.append_basic_block(function, "after");

//     builder.position_at_end(entry_block);

//     // Get the function parameter (n)
//     let n = function.get_first_param().unwrap().into_int_value();

//     // Check if n < 2 (base case)
//     let condition = builder.build_int_compare(inkwell::IntPredicate::ULT, n, i32_type.const_int(2, false), "n_less_than_two").unwrap();
//     builder.build_conditional_branch(condition, base_case_block, recursive_block);

//     // Base case: return n (fib(0) = 0, fib(1) = 1)
//     builder.position_at_end(base_case_block);
//     builder.build_return(Some(&n));

//     // Recursive case: return fib(n-1) + fib(n-2)
//     builder.position_at_end(recursive_block);
//     let n_minus_one = builder.build_int_sub(n, i32_type.const_int(1, false), "n_minus_one");
//     let n_minus_two = builder.build_int_sub(n, i32_type.const_int(2, false), "n_minus_two");

//     let fib_n_minus_one = builder.build_call(function, &[n_minus_one.unwrap().into()], "fib_n_minus_one").expect("").try_as_basic_value().left().unwrap().into_int_value();
//     let fib_n_minus_two = builder.build_call(function, &[n_minus_two.unwrap().into()], "fib_n_minus_two").expect("").try_as_basic_value().left().unwrap().into_int_value();
//     let result = builder.build_int_add(fib_n_minus_one, fib_n_minus_two, "result");

//     builder.build_return(Some(&result.unwrap()));
//     // builder.build_return(Some(&result.into()));

//     // Position at the end of the "after" block (which we don't use in this example)
//     builder.position_at_end(after_block);

//     // Optionally, print the LLVM IR
//     module.print_to_stderr();

//     // Create an execution engine to run the function (if desired)
//     let execution_engine = module.create_jit_execution_engine(OptimizationLevel::None).unwrap();

//     // Run the function
//     let n = 10;

//     let result = unsafe {
//         let faddr = execution_engine
//             .get_function_address("fibonacci")
//             .unwrap();
//         let function: extern "C" fn(i32) -> i32 = std::mem::transmute(faddr);
//         function(n)
//     };

//     println!("Fibonacci({}) = {}", n, result);

//     module.print_to_string();
// }

// fn build_wasm_module() {
//     Target::initialize_webassembly(&InitializationConfig::default());

//     let context = Context::create();
//     let module = context.create_module("add_one_module");

//     let target_triple = TargetTriple::create("wasm32-unknown-unknown");
//     module.set_triple(&target_triple);

//     // Create a target machine
//     let target = Target::from_triple(&target_triple).unwrap();
//     let target_machine = target
//         .create_target_machine(
//             &target_triple,
//             "generic",
//             TargetMachine::get_host_cpu_features().to_str().unwrap(),
//             OptimizationLevel::Default,
//             RelocMode::Default,
//             CodeModel::Default,
//         )
//         .unwrap();

//     // Print module to string.
//     module.print_to_string();

//     // Emit the module as a WASM object file
//     target_machine.write_to_file(
//         &module,
//         inkwell::targets::FileType::Object,
//         Path::new("playground/src/lib/add_one_inkwell.wasm"),
//     ).unwrap();
// }


fn main() {
    let data_layout_str = "e-m:e-p:32:32-i64:64-n32:64-S128";
    // let target = Target::from_name("wasm32-unknown-unknown").unwrap();
    // Target::initialize_webassembly(&InitializationConfig::default());
    // let opt = OptimizationLevel::Default;
    // let reloc = RelocMode::Default;
    // let model = CodeModel::Default;

    // let target = Target::from_name("wasm32").unwrap();
    // println!("Target: {:?}", target.get_name());

    // let has_asm = target.has_asm_backend();
    // println!("Has ASM? {:?}", has_asm);

    // let desc = target.get_description().to_str().unwrap();
    // println!("Description: {:?}", desc);

    // let has_machine = target.has_target_machine();
    // println!("Has machine? {:?}", has_machine);

    // let target_machine = target.create_target_machine(
    //     &TargetTriple::create("wasm32-unknown-unknown-wasi"),
    //     "wasm32",
    //     "",
    //     opt,
    //     reloc,
    //     model,
    // ).unwrap();

    // println!("Target Machine {:?}", target_machine);

    // println!("CPU {:?}", target_machine.get_cpu());

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 && vec!["-tr", "--test-run"].contains(&args[1].as_str()) {
        // print_llvm_add_one_module();
        // build_wasm_module();
        // build_add_one_wasm();
    }

    if args.len() > 3 && vec!["-c", "--compile"].contains(&args[1].as_str()) {

        // let input = args[2].to_string();
        // let output = args[3].as_str();

        // let txt = fs::read_to_string(input)
        //     .expect("Should have been able to read the file");

        // let function = parse_definition().parse(txt.as_str()).unwrap().0;
        // let ctx = Context::create();
        // let mut compiler = compiler::create_compiler!(&ctx, "tmp");
        // // set_compiler_hook(&mut compiler);

        // let engine = compiler.module.create_jit_execution_engine(OptimizationLevel::None).unwrap();
        // let target_data = engine.get_target_data();
        // let data_layout = target_data.get_data_layout();
        // compiler.module.set_triple(&TargetTriple::create("wasm32-unknown-empscripten"));
        // compiler.module.set_data_layout(&data_layout);


        // compiler.function = &function;
        // let _result = compiler.compile_fn().unwrap();

        // let _output_file = File::create(output)
        //     .expect("Should be able to create file");

        // let _file = File::create(Path::new(output)).unwrap();
        // compiler.module.print_to_file(Path::new(output)).unwrap();
        // compiler.module.write_bitcode_to_file(&file, true, true);
        // let buffer = target_machine.write_to_memory_buffer(&compiler.module, FileType::Assembly).unwrap();
        // let f = target_machine.write_to_file(&compiler.module, FileType::Assembly, Path::new(output)).unwrap();

        // compiler.module.print_to_file(Path::new(output)).unwrap();
        // compiler.module.write_bitcode_to_file(&output_file, true, true);
    } else if args.len() == 2 && vec!["-ll", "--llvm-prompt"].contains(&args[1].as_str()) {
        loop {
            print!("monet-llvm> ");
            // io::Write::flush(&mut io::stdout()).expect("flush failed!");
            // let mut input = String::new();
            // match std::io::stdin().read_line(&mut input) {
            //     Ok(_) => {
            //         let expr = expression_parser().parse(input.as_str()).unwrap().0;
            //         let ctx = Context::create();
            //         let mut compiler = compiler::create_compiler!(&ctx, "tmp");
            //         set_compiler_hook(&mut compiler);
            //         let result = compiler.compile_expr(&expr).unwrap();
            //         println!("{:?}", result.to_string());
            //     },
            //     Err(err) => println!("Could not parse input: {}", err),
            // }
        }
    }
}
