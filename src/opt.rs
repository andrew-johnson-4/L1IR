use std::fmt::Debug;
use crate::ast;
use crate::ast::{Program,Error,Expression};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Module};

pub struct JProgram {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    data_ctx: DataContext,
    module: JITModule,
}

pub struct JExpr {
   pub value: Value,
}

pub struct JType {
   pub name: String,
   pub jtype: types::Type,
}

pub fn compile_expr<'f,S: Clone + Debug>(p: &Program<S>, f: &mut FunctionBuilder<'f>, e: &Expression<S>) -> (JExpr,JType) {
   match e {
      Expression::UnaryIntroduction(_ui,_span) => {
         (JExpr {
            value: f.ins().iconst(types::I64, i64::from(0))
         }, JType {
            name: "Unary".to_string(),
            jtype: types::I64,
         })
      },
      Expression::LiteralIntroduction(_li,_span) => unimplemented!("compile expression: LiteralIntroduction"),
      Expression::TupleIntroduction(_ti,_span) => unimplemented!("compile expression: TupleIntroduction"),
      Expression::VariableReference(_vi,_span) => unimplemented!("compile expression: VariableReference"),
      Expression::FunctionReference(_vi,_span) => unimplemented!("compile expression: FunctionReference"),
      Expression::FunctionApplication(fi,args,_span) => {
         let mut arg_types = Vec::new();
         for a in args.iter() {
            let (je,jt) = compile_expr(p, f, a);
            arg_types.push(jt);
         }
         apply_fn(p, *fi, arg_types)
      },
      Expression::PatternMatch(_pe,_lrs,_span) => unimplemented!("compile expression: PatternMatch"),
      Expression::Failure(_span) => unimplemented!("compile expression: Failure"),
   }
}

pub fn apply_fn<S: Clone + Debug>(p: &Program<S>, fi: usize, args: Vec<JType>) -> (JExpr,JType) {
   unimplemented!("apply function: f#{}", fi)
}

impl JProgram {
   //functions will not be compiled until referenced
   pub fn compile<S: Clone + Debug>(&mut self, p: &Program<S>) {
      //int main(int *args, size_t args_count);
      let pointer_type = self.module.target_config().pointer_type();
      self.ctx.func.signature.params.push(AbiParam::new(pointer_type));
      self.ctx.func.signature.params.push(AbiParam::new(types::I64));
      self.ctx.func.signature.returns.push(AbiParam::new(types::I64));

      let mut main = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
      let entry_block = main.create_block();
      main.append_block_params_for_function_params(entry_block);
      main.switch_to_block(entry_block);
      main.seal_block(entry_block);

      for pe in p.expressions.iter() {
         compile_expr(p, &mut main, pe);
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
   pub fn eval(&self) -> Result<ast::Value,Error<String>> {
      Ok(ast::Value::unary(b"00"))
   }
}
