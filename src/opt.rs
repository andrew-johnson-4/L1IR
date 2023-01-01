use std::fmt::Debug;
use crate::ast::{Program,Value,Error};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};

pub struct JITProgram {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: JITModule,
}

pub fn jsweep<S: Clone + Debug>(p: Program<S>) -> JITProgram {
   let builder = JITBuilder::new(cranelift_module::default_libcall_names());
   let module = JITModule::new(builder.unwrap());
   JITProgram {
      builder_context: FunctionBuilderContext::new(),
      ctx: module.make_context(),
      data_ctx: DataContext::new(),
      module,
   }
}

impl JITProgram {
   pub fn eval(&self) -> Result<Value,Error<String>> {
      Ok(Value::unary(b"00"))
   }
}
