use std::fmt::Debug;
use crate::ast::{Program,Value,Error,Expression};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};

pub struct JITProgram {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: JITModule,
}

impl JITProgram {
   pub fn compile_fn<S: Clone + Debug>(&mut self) {
      //functions will not be compiled until referenced
   }
   pub fn compile_expr<S: Clone + Debug>(&mut self, e: &Expression<S>) {
      match e {
         Expression::UnaryIntroduction(_ui,_span) => unimplemented!("compile expression: UnaryIntroduction"),
         Expression::LiteralIntroduction(_li,_span) => unimplemented!("compile expression: LiteralIntroduction"),
         Expression::TupleIntroduction(_ti,_span) => unimplemented!("compile expression: TupleIntroduction"),
         Expression::VariableReference(_vi,_span) => unimplemented!("compile expression: VariableReference"),
         Expression::FunctionReference(_vi,_span) => unimplemented!("compile expression: FunctionReference"),
         Expression::FunctionApplication(_fi,_args,_span) => unimplemented!("compile expression: FunctionApplication"),
         Expression::PatternMatch(_pe,_lrs,_span) => unimplemented!("compile expression: PatternMatch"),
         Expression::Failure(_span) => unimplemented!("compile expression: Failure"),
      }
   }
   pub fn compile<S: Clone + Debug>(&mut self, p: &Program<S>) {
      for pe in p.expressions.iter() {
         self.compile_expr(pe);
      }
   }
}

pub fn jsweep<S: Clone + Debug>(p: Program<S>) -> JITProgram {
   let builder = JITBuilder::new(cranelift_module::default_libcall_names());
   let module = JITModule::new(builder.unwrap());
   let mut jp = JITProgram {
      builder_context: FunctionBuilderContext::new(),
      ctx: module.make_context(),
      data_ctx: DataContext::new(),
      module,
   };
   jp.compile(&p);
   jp
}

impl JITProgram {
   pub fn eval(&self) -> Result<Value,Error<String>> {
      Ok(Value::unary(b"00"))
   }
}
