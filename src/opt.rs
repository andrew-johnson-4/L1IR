use std::fmt::Debug;
use crate::ast::{Program,Value,Error,Expression};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};

pub struct JProgram {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: JITModule,
}

pub struct JType {
   pub name: String,
   pub jtype: types::Type,
}

impl JProgram {
   //functions will not be compiled until referenced
   pub fn compile_fn<S: Clone + Debug>(&mut self, p: &Program<S>, fi: usize, args: Vec<JType>) -> JType {
      unimplemented!("compile function: f#{}", fi)
   }
   pub fn compile_expr<S: Clone + Debug>(&mut self, p: &Program<S>, e: &Expression<S>) -> JType {
      match e {
         Expression::UnaryIntroduction(_ui,_span) => {
            JType {
               name: "Unary".to_string(),
               jtype: types::I64,
            }
         },
         Expression::LiteralIntroduction(_li,_span) => unimplemented!("compile expression: LiteralIntroduction"),
         Expression::TupleIntroduction(_ti,_span) => unimplemented!("compile expression: TupleIntroduction"),
         Expression::VariableReference(_vi,_span) => unimplemented!("compile expression: VariableReference"),
         Expression::FunctionReference(_vi,_span) => unimplemented!("compile expression: FunctionReference"),
         Expression::FunctionApplication(fi,args,_span) => {
            let mut arg_types = Vec::new();
            for a in args.iter() {
               let jt = self.compile_expr(p, a);
               arg_types.push(jt);
            }
            self.compile_fn(p, *fi, arg_types)
         },
         Expression::PatternMatch(_pe,_lrs,_span) => unimplemented!("compile expression: PatternMatch"),
         Expression::Failure(_span) => unimplemented!("compile expression: Failure"),
      }
   }
   pub fn compile<S: Clone + Debug>(&mut self, p: &Program<S>) {
      for pe in p.expressions.iter() {
         self.compile_expr(p, pe);
      }
   }
}

pub fn jsweep<S: Clone + Debug>(p: Program<S>) -> JProgram {
   let builder = JITBuilder::new(cranelift_module::default_libcall_names());
   let module = JITModule::new(builder.unwrap());
   let mut jp = JProgram {
      builder_context: FunctionBuilderContext::new(),
      ctx: module.make_context(),
      data_ctx: DataContext::new(),
      module,
   };
   jp.compile(&p);
   jp
}

impl JProgram {
   pub fn eval(&self) -> Result<Value,Error<String>> {
      Ok(Value::unary(b"00"))
   }
}
