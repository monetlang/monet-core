use std::collections::HashMap;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, PointerValue};

use crate::ast::Expr;

pub struct Compiler<'a, 'ctx> {
  pub context: &'ctx Context,
  pub builder: &'a Builder<'ctx>,
  pub module: &'a Module<'ctx>,
  // pub function: &'a Function,

  variables: HashMap<String, BasicValueEnum<'ctx>>,
  // fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
  pub fn build_load(&self, ptr: PointerValue<'ctx>, name: &str) -> BasicValueEnum<'ctx> {
    self.builder.build_load(self.context.f64_type(), ptr, name).unwrap()
  }

  pub(crate) fn compile_expr(&self, expr: &Expr) -> Result<BasicValueEnum<'ctx>, &'static String> {
    match expr {
      Expr::Number(n) => Ok(BasicValueEnum::FloatValue(self.context.f64_type().const_float(*n))),
      Expr::Variable(ref name) => match self.variables.get(name.as_str()) {
        Some(var) => Ok(*var),
        None => todo!("what!"),
      },
      Expr::Call { ref callee, ref args } => match self.module.get_function(callee.as_str()) {
        Some(callee) => {
          let mut compiled_args = Vec::with_capacity(args.len());

          for args in args {
            compiled_args.push(self.compile_expr(args)?);
          }

          let argsv: Vec<BasicMetadataValueEnum> =
            compiled_args.iter().by_ref().map(|&val| val.into()).collect();

          match self
            .builder
            .build_call(callee, argsv.as_slice(), "tmp")
            .unwrap()
            .try_as_basic_value()
            .left()
            {
              Some(val) => Ok(val),
              None => todo!("Invalid call produced."),
            }
        }
        None => todo!("Unknown function"),
      },
      Expr::BinOp {
        op,
        ref lhs,
        ref rhs
      } => {
        let lhs = self.compile_expr(lhs.as_ref())?;
        let rhs = self.compile_expr(rhs.as_ref())?;
        let l = lhs.into_float_value();
        let r = rhs.into_float_value();
        let plusop = self.builder.build_float_add(l, r, "subtmp").unwrap();
        let minusop = self.builder.build_float_sub(lhs.into_float_value(), rhs.into_float_value(), "subtmp").unwrap();
        let multop = self.builder.build_float_mul(lhs.into_float_value(), rhs.into_float_value(), "multmp").unwrap();
        let divop = self.builder.build_float_div(lhs.into_float_value(), rhs.into_float_value(), "divtmp").unwrap();
        match op {
          '+' => Ok(BasicValueEnum::FloatValue(plusop)),
          '-' => Ok(BasicValueEnum::FloatValue(minusop)),
          '*' => Ok(BasicValueEnum::FloatValue(multop)),
          '/' => Ok(BasicValueEnum::FloatValue(divop)),
          _ => todo!("compile_expr: {:?}", expr),
        }
      },
      _ => todo!("compile_expr: {:?}", expr),
    }
  }

//     /// Compiles the specified `Prototype` into an extern LLVM `FunctionValue`.
//     fn compile_prototype(&self, proto: &Prototype) -> Result<FunctionValue<'ctx>, &'static str> {
//       let ret_type = self.context.f64_type();
//       let args_types = std::iter::repeat(ret_type)
//           .take(proto.args.len())
//           .map(|f| f.into())
//           .collect::<Vec<BasicMetadataTypeEnum>>();
//       let args_types = args_types.as_slice();

//       let fn_type = self.context.f64_type().fn_type(args_types, false);
//       let fn_val = self.module.add_function(proto.name.as_str(), fn_type, None);

//       // set arguments names
//       for (i, arg) in fn_val.get_param_iter().enumerate() {
//           arg.into_float_value().set_name(proto.args[i].as_str());
//       }

//       // finally return built prototype
//       Ok(fn_val)
//   }

//   /// Compiles the specified `Function` into an LLVM `FunctionValue`.
//   fn compile_fn(&mut self) -> Result<FunctionValue<'ctx>, &'static str> {
//     let proto = &self.function.prototype;
//     let function = self.compile_prototype(proto)?;

//     // got external function, returning only compiled prototype
//     if self.function.body.is_none() {
//         return Ok(function);
//     }

//     let entry = self.context.append_basic_block(function, "entry");

//     self.builder.position_at_end(entry);

//     // update fn field
//     self.fn_value_opt = Some(function);

//     // build variables map
//     self.variables.reserve(proto.args.len());

//     for (i, arg) in function.get_param_iter().enumerate() {
//         let arg_name = proto.args[i].as_str();
//         let alloca = self.create_entry_block_alloca(arg_name);

//         self.builder.build_store(alloca, arg).unwrap();

//         self.variables.insert(proto.args[i].clone(), alloca);
//     }

//     // compile body
//     let body = self.compile_expr(self.function.body.as_ref().unwrap())?;

//     self.builder.build_return(Some(&body)).unwrap();

//     // return the whole thing after verification and optimization
//     if function.verify(true) {
//         Ok(function)
//     } else {
//         unsafe {
//             function.delete();
//         }

//         Err("Invalid generated function.")
//     }
//   }

// /// Compiles the specified `Function` in the given `Context` and using the specified `Builder` and `Module`.
//   pub fn compile(
//     context: &'ctx Context,
//     builder: &'a Builder<'ctx>,
//     module: &'a Module<'ctx>,
//     function: &Function,
// ) -> Result<FunctionValue<'ctx>, &'static str> {
//     let mut compiler = Compiler {
//         context,
//         builder,
//         module,
//         function,
//         fn_value_opt: None,
//         variables: HashMap::new(),
//     };
//     compiler.compile_fn()
//   }
// }
}

#[cfg(test)]
mod tests {
  use super::*;
  use inkwell::{context::Context, types::BasicMetadataTypeEnum};

  #[test]
  fn test_compile_number() {
    let context = &Context::create();
    let builder = &context.create_builder();
    let module = &context.create_module("tmp");
    let variables = HashMap::new();

    let mut compiler = Compiler {
      context,
      builder,
      module,
      variables,
    };

    let expr = Expr::Number(1.0);
    let result = compiler.compile_expr(&expr).unwrap();
    let s = result.to_string();
    assert_eq!(s, "\"double 1.000000e+00\"".to_string());
  }

  #[test]
  fn test_compile_variable() {
    let context = &Context::create();
    let builder = &context.create_builder();
    let module = &context.create_module("tmp");
    let mut variables = HashMap::new();
    let f64_type = context.f64_type();
    let a = BasicValueEnum::FloatValue(f64_type.const_float(1.2));
    variables.insert("x".to_string(), a);

    let mut compiler = Compiler {
      context,
      builder,
      module,
      variables,
    };

    let expr = Expr::Variable("x".to_string());
    let result = compiler.compile_expr(&expr).unwrap();
    let s = result.to_string();
    assert_eq!(s, "\"double 1.200000e+00\"".to_string());
  }

  #[test]
  fn test_compile_call() {
    let context = &Context::create();
    let builder = &context.create_builder();
    let module = &context.create_module("tmp");
    let void_type = context.void_type();

    let f64_type: BasicMetadataTypeEnum = context.f64_type().into();
    let fn_type = context.f64_type().fn_type(&[f64_type, f64_type], false);
    let basic_block = context.append_basic_block(function1, "fadd");
    builder.position_at_end(basic_block);

    let variables = HashMap::new();

    let compiler = Compiler {
      context,
      builder,
      module,
      variables,
    };

    let expr = Expr::Call {
      callee: "fadd".to_string(),
      args: vec![Expr::Number(1.0), Expr::Number(3.0)]
    };

    let result = compiler.compile_expr(&expr).unwrap();
    let s = result.to_string();
    assert_eq!(s, "\"  %tmp = call double @fadd(double 1.000000e+00, double 3.000000e+00)\"".to_string());
  }

  #[test]
  fn test_compile_binop() {
    let context = &Context::create();
    let builder = &context.create_builder();
    let module = &context.create_module("my_module");
    let void_type = context.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function1 = module.add_function("do_nothing", fn_type, None);
    let basic_block = context.append_basic_block(function1, "entry");
    builder.position_at_end(basic_block);

    let variables = HashMap::new();

    let compiler = Compiler {
      context,
      builder,
      module,
      variables,
    };

    let expr = Expr::BinOp {
      op: '+',
      lhs: Box::new(Expr::Number(1.0)),
      rhs: Box::new(Expr::Number(3.0)),
    };

    let result = compiler.compile_expr(&expr).unwrap();
    let s = result.to_string();
    assert_eq!(s, "\"double 4.000000e+00\"".to_string());
  }
}
