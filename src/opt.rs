use std::fmt::Debug;
use crate::ast;
use crate::ast::{Program,Error,Expression};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};

pub struct JProgram {
   main: *const u8,
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
   pub fn compile<S: Clone + Debug>(p: &Program<S>) -> JProgram {
      let mut builder = JITBuilder::new(cranelift_module::default_libcall_names());
      let mut module = JITModule::new(builder.unwrap());
      let mut builder_context = FunctionBuilderContext::new();
      let mut ctx = module.make_context();
      let mut data_ctx = DataContext::new();

      //int main(int *args, size_t args_count);
      let mut sig_main = module.make_signature();
      sig_main.params.push(AbiParam::new(types::I64));
      sig_main.params.push(AbiParam::new(types::I64));
      sig_main.returns.push(AbiParam::new(types::I64));

      let fn_main = module
        .declare_function("main", Linkage::Local, &sig_main)
        .unwrap();
      ctx.func.signature = sig_main;
      //self.ctx.func.name = UserFuncName::user(0, 0); //0::0 = main

      let mut main = FunctionBuilder::new(&mut ctx.func, &mut builder_context);
      let entry_block = main.create_block();
      main.append_block_params_for_function_params(entry_block);
      main.switch_to_block(entry_block);
      let rval = main.ins().iconst(types::I64, i64::from(12345));
      main.ins().return_(&[rval]);
      main.seal_block(entry_block);
      main.finalize();

      module.define_function(fn_main, &mut ctx).unwrap();
      module.clear_context(&mut ctx);
      module.finalize_definitions();

      /*
      for pe in p.expressions.iter() {
         compile_expr(p, &mut main, pe);
      }
      */

      JProgram {
         main: module.get_finalized_function(fn_main),
      }
   }
   pub fn eval(&self) -> Result<ast::Value,Error<String>> {
      let ptr_main = unsafe { std::mem::transmute::<_, fn(u64,u64) -> u64>(self.main) };
      let res = ptr_main(1,2);
      Ok(ast::Value::from_u64(res))
   }
}
