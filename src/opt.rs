use std::fmt::Debug;
use crate::ast;
use crate::ast::{Program,Error,Expression,FunctionDefinition,LIPart};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};
use num_bigint::BigUint;
use num_traits::ToPrimitive;

pub struct JProgram {
   main: *const u8,
}

pub struct JExpr {
   pub value: Value,
}

#[derive(Clone,Eq,PartialEq)]
pub struct JType {
   pub name: String,
   pub jtype: types::Type,
}

pub fn compile_expr<'f,S: Clone + Debug>(ctx: &mut FunctionBuilder<'f>, p: &Program<S>, e: &Expression<S>) -> (JExpr,JType) {
   match e {
      Expression::UnaryIntroduction(ui,_span) => {
         let ui = ui.to_i64().unwrap();
         (JExpr {
            value: ctx.ins().iconst(types::I64, ui)
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
            let jejt = compile_expr(ctx, p, a);
            arg_types.push(jejt);
         }
         apply_fn(ctx, p, *fi, arg_types)
      },
      Expression::PatternMatch(_pe,_lrs,_span) => unimplemented!("compile expression: PatternMatch"),
      Expression::Failure(_span) => unimplemented!("compile expression: Failure"),
   }
}

pub fn apply_fn<'f, S: Clone + Debug>(ctx: &mut FunctionBuilder<'f>, p: &Program<S>, fi: usize, args: Vec<(JExpr,JType)>) -> (JExpr,JType) {
   if let Some((je,jt)) = check_hardcoded_call(ctx, p, fi, &args) {
      return (je, jt);
   }
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

      let mut main = FunctionBuilder::new(&mut ctx.func, &mut builder_context);
      let entry_block = main.create_block();
      main.append_block_params_for_function_params(entry_block);
      main.switch_to_block(entry_block);

      if p.expressions.len()==0 {
         let rval = main.ins().iconst(types::I64, i64::from(12345));
         main.ins().return_(&[rval]);
      } else {
         for pi in 0..(p.expressions.len()-1) {
            compile_expr(&mut main, p, &p.expressions[pi]);
         }
         let (je,jt) = compile_expr(&mut main, p, &p.expressions[p.expressions.len()-1]);
         main.ins().return_(&[je.value]);
      }

      main.seal_block(entry_block);
      main.finalize();

      module.define_function(fn_main, &mut ctx).unwrap();
      module.clear_context(&mut ctx);
      module.finalize_definitions();

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

pub fn check_hardcoded_call<'f, S: Clone + Debug>(ctx: &mut FunctionBuilder<'f>, p: &Program<S>, fi: usize, args: &Vec<(JExpr,JType)>) -> Option<(JExpr,JType)> {
   let sig = args.iter().map(|(_je,jt)| jt.jtype).collect::<Vec<types::Type>>();
   let val = args.iter().map(|(je,_jt)| je.value).collect::<Vec<Value>>();
   let hardcoded: Vec<(Vec<types::Type>,FunctionDefinition<()>,fn(&mut FunctionBuilder<'f>,Vec<Value>) -> Value)> = vec![
      (vec![types::I64,types::I64],
       FunctionDefinition::define(
         vec![0,1],
         vec![Expression::li(vec![
            LIPart::variable(0),
            LIPart::variable(1),
         ],())]
      ),|ctx,val| {
         let val0 = val[0].clone();
         let val1 = val[1].clone();
         ctx.ins().iadd(val0, val1)
      })
   ];
   for (hsig,hdef,hexpr) in hardcoded.iter() {
      if &sig == hsig && p.functions[fi].equals(hdef) {
         let rval = hexpr(ctx, val);
         return Some((
            JExpr { value: rval },
            JType { name:"".to_string(), jtype: types::I64 },
         ));
      }
   }
   None
}
