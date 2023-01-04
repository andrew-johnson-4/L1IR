use std::fmt::Debug;
use crate::ast;
use crate::ast::{Program,Error,Expression};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module};
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
      Expression::VariableReference(vi,_span) => {
         let jv = Variable::from_u32(*vi as u32);
         let jv = ctx.use_var(jv);
         (JExpr {
            value: jv
         }, JType {
            name: "Unary".to_string(),
            jtype: types::I64,
         })
      },
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
      let builder = JITBuilder::new(cranelift_module::default_libcall_names());
      let mut module = JITModule::new(builder.unwrap());
      let mut builder_context = FunctionBuilderContext::new();
      let mut ctx = module.make_context();
      let mut _data_ctx = DataContext::new();

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

      let mut pars = Vec::new();
      for pe in p.expressions.iter() {
         pe.vars(&mut pars);
      }
      for pi in pars.iter() {
         let pv = Variable::from_u32(*pi as u32);
         main.declare_var(pv, types::I64);
         let arg_base = main.block_params(entry_block)[0];
         let arg_offset = (8 * *pi) as i32;
         let arg_flags = MemFlags::new();
         let arg_value = main.ins().load(types::I64, arg_flags, arg_base, arg_offset);
         main.def_var(pv, arg_value);
      }

      if p.expressions.len()==0 {
         let rval = main.ins().iconst(types::I64, i64::from(0));
         main.ins().return_(&[rval]);
      } else {
         for pi in 0..(p.expressions.len()-1) {
            compile_expr(&mut main, p, &p.expressions[pi]);
         }
         let (je,_jt) = compile_expr(&mut main, p, &p.expressions[p.expressions.len()-1]);
         main.ins().return_(&[je.value]);
      }

      main.seal_block(entry_block);
      main.finalize();

      module.define_function(fn_main, &mut ctx).unwrap();
      module.clear_context(&mut ctx);
      module.finalize_definitions().unwrap();

      JProgram {
         main: module.get_finalized_function(fn_main),
      }
   }
   pub fn eval(&self, args: &[u64]) -> Result<ast::Value,Error<String>> {
      let ptr_main = unsafe { std::mem::transmute::<_, fn(*const u64,u64) -> u64>(self.main) };
      let res = ptr_main(args.as_ptr(), args.len() as u64);
      Ok(ast::Value::from_u64(res))
   }
}

pub fn check_hardcoded_call<'f, S: Clone + Debug>(ctx: &mut FunctionBuilder<'f>, p: &Program<S>, fi: usize, args: &Vec<(JExpr,JType)>) -> Option<(JExpr,JType)> {
   let sig = args.iter().map(|(_je,jt)| jt.jtype).collect::<Vec<types::Type>>();
   let val = args.iter().map(|(je,_jt)| je.value).collect::<Vec<Value>>();
   let hardcoded = crate::recipes::cranelift::import();
   for (hsig,hdef,hexpr,htype) in hardcoded.iter() {
      if &sig == hsig && p.functions[fi].equals(hdef) {
         let rval = hexpr(ctx, val);
         return Some((
            JExpr { value: rval },
            JType { name:"".to_string(), jtype: *htype },
         ));
      }
   }
   None
}
