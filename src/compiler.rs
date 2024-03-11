use std::collections::HashMap;

use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FloatValue, PointerValue};
use inkwell::types::AsTypeRef;
use crate::ast::Expr;

pub struct Compiler<'a, 'ctx> {
  pub context: &'ctx Context,
  pub builder: &'a Builder<'ctx>,
  // pub module: &'a Module<'ctx>,
  // pub function: &'a Function,
  // variables: HashMap<String, PointerValue<'ctx>>,
  variables: HashMap<String, BasicValueEnum<'ctx>>,
  // fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
  // #[llvm_versions(4.0..=14.0)]
  // pub fn build_load(&self, ptr: PointerValue<'ctx>, name: &str) -> BasicValueEnum<'ctx> {
  //     self.builder.build_load(ptr, name).unwrap()
  // }

  // #[llvm_versions(15.0..=latest)]
  // pub fn build_load(&self, ptr: PointerValue<'ctx>, name: &str) -> BasicValueEnum<'ctx> {
  //     self.builder.build_load(self.context.f64_type(), ptr, name).unwrap()
  // }

  pub fn build_load(&self, ptr: PointerValue<'ctx>, name: &str) -> BasicValueEnum<'ctx> {
    self.builder.build_load(self.context.f64_type(), ptr, name).unwrap()
}

  pub(crate) fn compile_expr(&self, expr: &Expr) -> Result<BasicValueEnum<'ctx>, &'static String> {
    match *expr {
      // Expr::Number(n) => Ok(self.context.f64_type().const_float(n)),
      Expr::Number(n) => Ok(BasicValueEnum::FloatValue(self.context.f64_type().const_float(n))),
      Expr::Variable(ref name) => match self.variables.get(name.as_str()) {
        // Some(var) => Ok(self.build_load(*var, name)),
        Some(var) => Ok(*var),
        None => todo!("what!"),
      },
      _ => todo!("compile_expr: {:?}", expr),
    }
  }

  ///// Compiles the specified `Function` into an LLVM `FunctionValue`.
  // fn compile_fn(&mut self) -> Result<FunctionValue<'ctx>, &'static str> {
  //   let proto = &self.function.prototype;
  //   let function = self.compile_prototype(proto)?;

  //   // got external function, returning only compiled prototype
  //   if self.function.body.is_none() {
  //       return Ok(function);
  //   }

  //   let entry = self.context.append_basic_block(function, "entry");

  //   self.builder.position_at_end(entry);

  //   // update fn field
  //   self.fn_value_opt = Some(function);

  //   // build variables map
  //   self.variables.reserve(proto.args.len());

  //   for (i, arg) in function.get_param_iter().enumerate() {
  //       let arg_name = proto.args[i].as_str();
  //       let alloca = self.create_entry_block_alloca(arg_name);

  //       self.builder.build_store(alloca, arg).unwrap();

  //       self.variables.insert(proto.args[i].clone(), alloca);
  //   }

  //   // compile body
  //   let body = self.compile_expr(self.function.body.as_ref().unwrap())?;

  //   self.builder.build_return(Some(&body)).unwrap();

  //   // return the whole thing after verification and optimization
  //   if function.verify(true) {
  //       Ok(function)
  //   } else {
  //       unsafe {
  //           function.delete();
  //       }

  //       Err("Invalid generated function.")
  //   }
  // }

///// Compiles the specified `Function` in the given `Context` and using the specified `Builder` and `Module`.
//   pub fn compile(
//     context: &'ctx Context,
//     builder: &'a Builder<'ctx>,
//     module: &'a Module<'ctx>,
//     function: &Function,
// ) -> Result<FunctionValue<'ctx>, &'static str> {
//     let mut compiler = Compiler {
//         context,
//         builder,
//         // module,
//         // function,
//         // fn_value_opt: None,
//         variables: HashMap::new(),
//     };

//     compiler.compile_fn()
//   }
}

#[cfg(test)]
mod tests {
  use std::{collections::HashMap, result};

  use crate::ast::Function;

  use super::*;
  use combine::parser::token::Value;
  use inkwell::context::Context;
  use inkwell::builder::Builder;
  use inkwell::llvm_sys::LLVMValue;
  use inkwell::values::AsValueRef;

  #[test]
  fn test_compile_number() {
    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module("tmp");

    let compiler = Compiler {
      context: &context,
      builder: &builder,
      variables: HashMap::new(),
    };

    let expr = Expr::Number(1.0);
    let result = compiler.compile_expr(&expr).unwrap();
    let s = result.to_string();
    assert_eq!(s, "\"double 1.000000e+00\"".to_string());
  }

  #[test]
  fn test_compile_variable() {
    let context = Context::create();
    let builder = context.create_builder();
    let mut variables = HashMap::new();
    let f64_type = context.f64_type();
    let a = BasicValueEnum::FloatValue(f64_type.const_float(1.2));
    variables.insert("x".to_string(), a);
    let compiler = Compiler {
      context: &context,
      builder: &builder,
      variables,
    };

    let expr = Expr::Variable("x".to_string());
    let result = compiler.compile_expr(&expr).unwrap();
    let s = result.to_string();
    assert_eq!(s, "\"double 1.200000e+00\"".to_string());
  }
}
