use std::collections::HashMap;
use inkwell::context::Context;
use inkwell::builder::Builder;
use inkwell::module::Module;
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, FunctionValue, PointerValue, FloatValue};
use inkwell::FloatPredicate;

use crate::ast::{Expr, Function, Prototype};

pub struct Compiler<'a, 'ctx> {
  pub context: &'ctx Context,
  pub builder: &'a Builder<'ctx>,
  pub module: &'a Module<'ctx>,
  pub function: &'a Function,

  pub(crate) variables: HashMap<String, PointerValue<'ctx>>,
  pub(crate) fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {

  #[inline]
  fn fn_value(&self) -> FunctionValue<'ctx> {
    self.fn_value_opt.unwrap()
  }

  /// Creates a new stack allocation instruction in the entry block of the function.
  fn create_entry_block_alloca(&self, name: &str) -> PointerValue<'ctx> {
    let builder = self.context.create_builder();

    let entry = self.fn_value().get_first_basic_block().unwrap();

    match entry.get_first_instruction() {
        Some(first_instr) => builder.position_before(&first_instr),
        None => builder.position_at_end(entry),
    }

    builder.build_alloca(self.context.f64_type(), name).unwrap()
  }

  pub fn build_load(&self, ptr: PointerValue<'ctx>, name: &str) -> BasicValueEnum<'ctx> {
    self.builder.build_load(self.context.f64_type(), ptr, name).unwrap()
  }

  pub(crate) fn compile_fn(&mut self) -> Result<FunctionValue<'ctx>, &'static str> {
    let proto = &self.function.prototype;
    let function = self.compile_prototype(proto)?;

    // got external function, returning only compiled prototype
    if let Expr::None = self.function.body {
      return Ok(function);
    }

    // if self.function.body.is_none() {
    //   return Ok(function);
    // }

    let entry = self.context.append_basic_block(function, "entry");

    self.builder.position_at_end(entry);

    // update fn field
    self.fn_value_opt = Some(function);

    // build variables map
    self.variables.reserve(proto.args.len());

    for (i, arg) in function.get_param_iter().enumerate() {
      let arg_name = proto.args[i].as_str();
      let alloca = self.create_entry_block_alloca(arg_name);

      self.builder.build_store(alloca, arg).unwrap();

      self.variables.insert(proto.args[i].clone(), alloca);
    }

    // compile body
    let body = self.compile_expr(&self.function.body).unwrap();

    self.builder.build_return(Some(&body)).unwrap();

    // return the whole thing after verification and optimization
    if function.verify(true) {
      Ok(function)
    } else {
      unsafe {
        function.delete();
      }

      Err("Invalid generated function.")
    }
  }

  pub(crate) fn compile_prototype(&self, proto: &Prototype) -> Result<FunctionValue<'ctx>, &'static str> {
    let ret_type = self.context.f64_type();
    let args_types = std::iter::repeat(ret_type)
        .take(proto.args.len())
        .map(|f| f.into())
        .collect::<Vec<BasicMetadataTypeEnum>>();
    let args_types = args_types.as_slice();

    let fn_type = self.context.f64_type().fn_type(args_types, false);
    let fn_val = self.module.add_function(proto.name.as_str(), fn_type, None);

    // set arguments names
    for (i, arg) in fn_val.get_param_iter().enumerate() {
        arg.into_float_value().set_name(proto.args[i].as_str());
    }

    // finally return built prototype
    Ok(fn_val)
  }

  pub(crate) fn compile_expr(&self, expr: &Expr) -> Result<FloatValue<'ctx>, &'static String> {
    match expr {
      Expr::Number(n) => {
        let f64_type = self.context.f64_type();
        let llvm_num = f64_type.const_float(*n);
        Ok(llvm_num)
      },
      Expr::Variable(ref name) => match self.variables.get(name.as_str()) {
        Some(var) => Ok(self.build_load(*var, &name).into_float_value()),
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
              Some(val) => Ok(val.into_float_value()),
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
        // let fn_type = self.context.f64_type().fn_type(&[], false);
        // let function = self.module.add_function("my_function", fn_type, None);
        // let basic_block = self.context.append_basic_block(function, "entry");
        // self.builder.position_at_end(basic_block);

        let lhs = self.compile_expr(lhs.as_ref())?;
        let rhs = self.compile_expr(rhs.as_ref())?;
        let plusop = self.builder.build_float_add(lhs, rhs, "subtmp").unwrap();
        let minusop = self.builder.build_float_sub(lhs, rhs, "subtmp").unwrap();
        let multop = self.builder.build_float_mul(lhs, rhs, "multmp").unwrap();
        let divop = self.builder.build_float_div(lhs, rhs, "divtmp").unwrap();
        let ltop_ = self.builder.build_float_compare(FloatPredicate::ULT, lhs, rhs, "cmptmp").unwrap();
        let ltop = self.builder
          .build_unsigned_int_to_float(ltop_, self.context.f64_type(), "booltmp").unwrap();
        match op {
          '+' => Ok(plusop),
          '-' => Ok(minusop),
          '*' => Ok(multop),
          '/' => Ok(divop),
          '<' => Ok(ltop),
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

pub(crate) fn set_compiler_hook(c: &mut Compiler) {
  let fn_type = c.context.f64_type().fn_type(&[], false);
  let function = c.module.add_function("my_function", fn_type, None);
  let basic_block = c.context.append_basic_block(function, "entry");
  c.builder.position_at_end(basic_block);
}

// #[macro_export]
macro_rules! create_compiler {
  ($c:expr, $s:expr) => {
    Compiler {
      context: $c,
      builder: &$c.create_builder(),
      module: &$c.create_module($s),
      variables: HashMap::new(),
      function: &Function::default(),
      fn_value_opt: None,
    }
  }
}

pub(crate) use create_compiler;

#[cfg(test)]
mod tests {
  use core::f64;

use super::*;
  use inkwell::{context::Context, types::BasicMetadataTypeEnum, AddressSpace};
  use inkwell::values::InstructionOpcode;

  #[test]
  fn test_compile_number() {
    let ctx = Context::create();
    let compiler = create_compiler!(&ctx, "tmp");
    let result = compiler.compile_expr(&Expr::Number(1.0)).unwrap();
    assert_eq!(result.to_string(), "\"double 1.000000e+00\"".to_string());
  }

  #[test]
  fn test_compile_variable() {
    let ctx = Context::create();
    let mut compiler = create_compiler!(&ctx, "temp");
    let f64_type = ctx.f64_type();
    let float_val: FloatValue = f64_type.const_float(1.2);

    let fn_type = f64_type.fn_type(&[], false);
    let function = compiler.module.add_function("my_function", fn_type, None);
    let basic_block = compiler.context.append_basic_block(function, "entry");
    compiler.builder.position_at_end(basic_block);

    let alloca = compiler.builder.build_alloca(f64_type, "float_ptr").unwrap();
    let inst = compiler.builder.build_store(alloca, float_val).unwrap();
    let op = inst.get_opcode();
    assert!(op == InstructionOpcode::Store);
    let operand = inst.get_operand(0).unwrap().left().unwrap().into_float_value();
    assert!(operand.to_string() == "\"double 1.200000e+00\"".to_string());

    compiler.variables.insert("x".to_string(), alloca);
    let result = compiler.compile_expr(&Expr::Variable("x".to_string())).unwrap();
    assert_eq!(result.get_type(), f64_type);
  }

  #[test]
  fn test_compile_function() {
    let ctx = Context::create();
    let mut compiler = create_compiler!(&ctx, "tmp");
    let function = &Function::new(
      Prototype::new("fadd".to_string(), vec!["x".to_string(), "y".to_string()]),
      Expr::BinOp {
        op: '+',
        lhs: Box::new(Expr::Variable("x".to_string())),
        rhs: Box::new(Expr::Variable("y".to_string())),
      }
    );
    compiler.function = function;
    let f64_type: BasicMetadataTypeEnum = ctx.f64_type().into();
    let fn_type = ctx.f64_type().fn_type(&[f64_type, f64_type], false);
    let function1 = compiler.module.add_function("fadd", fn_type, None);
    let basic_block = ctx.append_basic_block(function1, "fadd");
    compiler.builder.position_at_end(basic_block);
    let result = compiler.compile_fn().expect("Failed to compile function");
    assert_eq!(result.get_name().to_str().unwrap(), "fadd.1");
    assert_eq!(result.get_first_param().unwrap().to_string(), "\"double %x\"");
    assert_eq!(result.get_last_param().unwrap().to_string(), "\"double %y\"");
  }

  #[test]
  fn test_compile_call() {
    let ctx = Context::create();
    let compiler = create_compiler!(&ctx, "tmp");

    let f64_type: BasicMetadataTypeEnum = ctx.f64_type().into();
    let fn_type = ctx.f64_type().fn_type(&[f64_type, f64_type], false);
    let function1 = compiler.module.add_function("fadd", fn_type, None);
    let basic_block = ctx.append_basic_block(function1, "fadd");
    compiler.builder.position_at_end(basic_block);

    let expr = Expr::Call {
      callee: "fadd".to_string(),
      args: vec![Expr::Number(1.0), Expr::Number(3.0)]
    };

    let result = compiler.compile_expr(&expr).unwrap();
    assert_eq!(result.to_string(), "\"  %tmp = call double @fadd(double 1.000000e+00, double 3.000000e+00)\"".to_string());
  }

  #[test]
  fn test_compile_binop() {
    let ctx = Context::create();
    let compiler = create_compiler!(&ctx, "tmp");

    let void_type = ctx.void_type();
    let fn_type = void_type.fn_type(&[], false);
    let function1 = compiler.module.add_function("do_nothing", fn_type, None);
    let basic_block = ctx.append_basic_block(function1, "entry");
    compiler.builder.position_at_end(basic_block);

    let expr = Expr::BinOp {
      op: '+',
      lhs: Box::new(Expr::Number(1.0)),
      rhs: Box::new(Expr::Number(3.0)),
    };

    let result = compiler.compile_expr(&expr).unwrap();
    assert_eq!(result.to_string(), "\"double 4.000000e+00\"".to_string());
  }

  #[test]
  fn test_compile_prototype() {
    let ctx = Context::create();
    let compiler = create_compiler!(&ctx, "tmp");
    let proto = Prototype::new("foo".to_string(), vec!["x".to_string(), "y".to_string()]);
    let result = compiler.compile_prototype(&proto).unwrap();
    let expect = "\"declare double @foo(double, double)\\n\"".to_string();
    assert_eq!(result.to_string(), expect);
  }
}
