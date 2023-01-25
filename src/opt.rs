use std::fmt::Debug;
use crate::value;
use crate::ast::{Program,Error,Expression,LHSPart,LHSLiteralPart,LIPart,TIPart};
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module, FuncOrDataId};
use num_traits::ToPrimitive;

static mut UNIQUE_ID: usize = 1000000;
fn uid() -> usize {
   unsafe {
      let id = UNIQUE_ID;
      UNIQUE_ID += 1;
      id
   }
}

pub struct JProgram {
   main: *const u8,
}

pub struct JExpr {
   pub block: Block,
   pub value: Value,
}

#[derive(Clone,Eq,PartialEq)]
pub struct JType {
   pub name: String,
   pub jtype: types::Type,
}

pub fn compile_fn<'f,S: Clone + Debug>(jmod: &mut JITModule, builder_context: &mut FunctionBuilderContext, p: &Program<S>, fi: usize) {
   let hpars = p.functions[fi].args.iter().map(|_|types::I64).collect::<Vec<types::Type>>();
   if is_hardcoded(p, fi, &hpars) {
      return;
   }

   let mut ctx = jmod.make_context();

   let mut sig_fn = jmod.make_signature();
   for _ in p.functions[fi].args.iter() {
      sig_fn.params.push(AbiParam::new(types::I64));
   }
   sig_fn.returns.push(AbiParam::new(types::I64));

   let fn0 = jmod
      .declare_function(&format!("f#{}", fi), Linkage::Local, &sig_fn)
      .unwrap();
   ctx.func.signature = sig_fn;

   let mut fnb = FunctionBuilder::new(&mut ctx.func, builder_context);
   let mut blk = fnb.create_block();
   fnb.append_block_params_for_function_params(blk);
   fnb.switch_to_block(blk);

   for (pi,vi) in p.functions[fi].args.iter().enumerate() {
      let pvar = Variable::from_u32(*vi as u32);
      fnb.declare_var(pvar, types::I64);
      let pval = fnb.block_params(blk)[pi];
      fnb.def_var(pvar, pval);
   }

   if p.functions[fi].body.len()==0 {
      let rval = fnb.ins().iconst(types::I64, i64::from(0));
      fnb.ins().return_(&[rval]);
   } else {
      for pi in 0..(p.functions[fi].body.len()-1) {
         let (je,_jt) = compile_expr(jmod, &mut fnb, blk, p, &p.functions[fi].body[pi]);
         blk = je.block;
      }
      let (je,_jt) = compile_expr(jmod, &mut fnb, blk, p, &p.functions[fi].body[p.functions[fi].body.len()-1]);
      blk = je.block;
      fnb.ins().return_(&[je.value]);
   }

   fnb.seal_block(blk);
   fnb.finalize();

   jmod.define_function(fn0, &mut ctx).unwrap();
   jmod.clear_context(&mut ctx);
}

pub fn compile_lhs<'f>(ctx: &mut FunctionBuilder<'f>, mut lblk: Block, rblk: Block, lhs: &LHSPart, nblk: Block, mut val: Value) {
   ctx.switch_to_block(lblk);
   match lhs {
      LHSPart::Tuple(_lts) => unimplemented!("compile_lhs(Tuple)"),
      LHSPart::Literal(lts) => {
         let cond = ctx.ins().icmp_imm(IntCC::Equal, val, lts.len() as i64);
         ctx.ins().brnz(cond, rblk, &[]);
         ctx.ins().jump(nblk, &[]);
      },
      LHSPart::UnpackLiteral(pres,mid,sufs) => {
         for p in pres.iter() {
         if let LHSLiteralPart::Literal(cs) = p {
            let cond = ctx.ins().icmp_imm(IntCC::UnsignedLessThan, val, cs.len() as i64);
            let bb = ctx.create_block(); //basic blocks can't compute after jump
            ctx.ins().brnz(cond, nblk, &[]);
            ctx.ins().jump(bb, &[]);
            ctx.seal_block(lblk);
            ctx.switch_to_block(bb);
            lblk = bb;
            let len = ctx.ins().iconst(types::I64, cs.len() as i64);
            val = ctx.ins().isub(val, len);
         } else if let LHSLiteralPart::Variable(vi) = p {
            let jv = Variable::from_u32(*vi as u32);
            let jv = ctx.use_var(jv);
            let cond = ctx.ins().icmp(IntCC::UnsignedLessThan, val, jv);
            let bb = ctx.create_block(); //basic blocks can't compute after jump
            ctx.ins().brnz(cond, nblk, &[]);
            ctx.ins().jump(bb, &[]);
            ctx.seal_block(lblk);
            ctx.switch_to_block(bb);
            lblk = bb;
            val = ctx.ins().isub(val, jv);
         }}
         for s in sufs.iter() {
         if let LHSLiteralPart::Literal(cs) = s {
            let cond = ctx.ins().icmp_imm(IntCC::UnsignedLessThan, val, cs.len() as i64);
            let bb = ctx.create_block(); //basic blocks can't compute after jump
            ctx.ins().brnz(cond, nblk, &[]);
            ctx.ins().jump(bb, &[]);
            ctx.seal_block(lblk);
            ctx.switch_to_block(bb);
            lblk = bb;
            let len = ctx.ins().iconst(types::I64, cs.len() as i64);
            val = ctx.ins().isub(val, len);
         } else if let LHSLiteralPart::Variable(vi) = s {
            let jv = Variable::from_u32(*vi as u32);
            let jv = ctx.use_var(jv);
            let cond = ctx.ins().icmp(IntCC::UnsignedLessThan, val, jv);
            let bb = ctx.create_block(); //basic blocks can't compute after jump
            ctx.ins().brnz(cond, nblk, &[]);
            ctx.ins().jump(bb, &[]);
            ctx.seal_block(lblk);
            ctx.switch_to_block(bb);
            lblk = bb;
            val = ctx.ins().isub(val, jv);
         }}
         if let Some(mi) = mid {
            let jv = Variable::from_u32(*mi as u32);
            ctx.declare_var(jv, types::I64);
            ctx.def_var(jv, val);
         }
         ctx.ins().jump(rblk, &[]);
      },
      LHSPart::Variable(_lv) => unimplemented!("compile_lhs(Variable)"),
      LHSPart::Any => {
         ctx.ins().jump(rblk, &[]);
      },
   }
   ctx.seal_block(lblk);
}

pub fn try_inline_plurals<'f,S: Clone + Debug>(jmod: &mut JITModule, ctx: &mut FunctionBuilder<'f>, mut blk: Block, p: &Program<S>,
                                               pe: &Expression<S>, lrs: &Vec<(LHSPart,Expression<S>)>, _span: &S) -> Option<(JExpr,JType)> {
   if let Expression::TupleIntroduction(tis,_span) = pe {
      for ts in tis.iter() {
      match ts {
         TIPart::Variable(_) => {},    //ok to inline
         TIPart::Expression(_) => {},  //ok to inline
         _ => { return None; }, //can't inline plural
      }}
      for (l,_r) in lrs.iter() {
      match l {
         LHSPart::Tuple(_) => {},
         LHSPart::Any => {},
         _ => { return None; },
      }}
      let mut header = Vec::new();
      for ts in tis.iter() {
      match ts {
         TIPart::Variable(vi) => {
            let jv = Variable::from_u32(*vi as u32);
            let jv = ctx.use_var(jv);
            header.push(jv);
         },
         TIPart::Expression(ve) => {
            let (je,_jt) = compile_expr(jmod, ctx, blk, p, ve);
            blk = je.block;
            let id = uid();
            let jv = Variable::from_u32(id as u32);
            ctx.declare_var(jv, types::I64);
            ctx.def_var(jv, je.value);
            let jv = ctx.use_var(jv);
            header.push(jv);
         },
         _ => { unreachable!() },
      }}

      let failblk = ctx.create_block(); //failure block
      let succblk = ctx.create_block(); //success block
      ctx.append_block_param(succblk, types::I64);

      let mut lblocks = Vec::new();
      let mut rblocks = Vec::new();
      for _ in lrs.iter() {
         lblocks.push(ctx.create_block());
         rblocks.push(ctx.create_block());
      }
      lblocks.push(failblk);
      let noval = ctx.ins().iconst(types::I64, 0);
      ctx.ins().jump(lblocks[0], &[]); //jump into first lhs guard
      ctx.seal_block(blk);             //seal pattern expression

      for (li,(l,_r)) in lrs.iter().enumerate() {
         match l {
            LHSPart::Tuple(lts) => {
               let mut current = lblocks[li];
               for (lti,lt) in lts.iter().enumerate() {
                  let next = if lti == (lts.len()-1) {
                     rblocks[li]
                  } else {
                     ctx.create_block()
                  };
                  compile_lhs(ctx, current, next, lt, lblocks[li+1], header[lti]);
                  current = next;
               }
            },
            LHSPart::Any => {
               compile_lhs(ctx, lblocks[li], rblocks[li], l, lblocks[li+1], noval);
            }
            _ => unreachable!(),
         }
      }

      for (ri,(_l,r)) in lrs.iter().enumerate() {
         ctx.switch_to_block(rblocks[ri]);
         let (je,_jt) = compile_expr(jmod, ctx, rblocks[ri], p, r);
         ctx.ins().jump(succblk, &[je.value]);
         ctx.seal_block(je.block);
      }

      ctx.switch_to_block(failblk); //define failure block
      ctx.ins().trap(TrapCode::UnreachableCodeReached);
      ctx.seal_block(failblk);

      ctx.switch_to_block(succblk); //return cfg to success block
      Some((JExpr {
         block: succblk,
         value: ctx.block_params(succblk)[0],
      }, JType {
         name: "Unary".to_string(),
         jtype: types::I64,
      }))
   } else { None }
}

pub fn compile_expr<'f,S: Clone + Debug>(jmod: &mut JITModule, ctx: &mut FunctionBuilder<'f>, mut blk: Block, p: &Program<S>, e: &Expression<S>) -> (JExpr,JType) {
   match e {
      Expression::UnaryIntroduction(ui,_span) => {
         let ui = ui.to_i64().unwrap();
         (JExpr {
            block: blk,
            value: ctx.ins().iconst(types::I64, ui)
         }, JType {
            name: "Unary".to_string(),
            jtype: types::I64,
         })
      },
      Expression::LiteralIntroduction(lis,_span) => {
         let mut val = ctx.ins().iconst(types::I64, 0);
         for li in lis.iter() {
         match li {
            LIPart::Expression(e) => {
               let (je,_jt) = compile_expr(jmod, ctx, blk, p, e);
               blk = je.block;
               val = ctx.ins().iadd(val, je.value);
            },
            LIPart::Literal(cs) => {
               val = ctx.ins().iadd_imm(val, cs.len() as i64);
            },
            LIPart::InlineVariable(vi) => {
               let jv = Variable::from_u32(*vi as u32);
               let jv = ctx.use_var(jv);
               val = ctx.ins().iadd(val, jv);
            },
         }}
         (JExpr {
            block: blk,
            value: val,
         }, JType {
            name: "Unary".to_string(),
            jtype: types::I64,
         })
      }
      Expression::TupleIntroduction(_ti,_span) => unimplemented!("compile expression: TupleIntroduction"),
      Expression::VariableReference(vi,_span) => {
         let jv = Variable::from_u32(*vi as u32);
         let jv = ctx.use_var(jv);
         (JExpr {
            block: blk,
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
            let jejt = compile_expr(jmod, ctx, blk, p, a);
            arg_types.push(jejt);
         }
         apply_fn(jmod, ctx, blk, p, *fi, arg_types)
      },
      Expression::PatternMatch(pe,lrs,span) => {
         if let Some((je,jt)) = try_inline_plurals(jmod, ctx, blk, p, pe.as_ref(), lrs.as_ref(), span) {
            return (je,jt);
         }
         let (je,_jt) = compile_expr(jmod, ctx, blk, p, pe);
         blk = je.block;

         let failblk = ctx.create_block(); //failure block
         let succblk = ctx.create_block(); //success block
         ctx.append_block_param(succblk, types::I64);

         let mut lblocks = Vec::new();
         let mut rblocks = Vec::new();
         for _ in lrs.iter() {
            lblocks.push(ctx.create_block());
            rblocks.push(ctx.create_block());
         }
         lblocks.push(failblk);
         ctx.ins().jump(lblocks[0], &[]); //jump into first lhs guard
         ctx.seal_block(blk);             //seal pattern expression

         for (li,(l,_r)) in lrs.iter().enumerate() {
            compile_lhs(ctx, lblocks[li], rblocks[li], l, lblocks[li+1], je.value);
         }

         for (ri,(_l,r)) in lrs.iter().enumerate() {
            ctx.switch_to_block(rblocks[ri]);
            let (je,_jt) = compile_expr(jmod, ctx, rblocks[ri], p, r);
            ctx.ins().jump(succblk, &[je.value]);
            ctx.seal_block(je.block);
         }

         ctx.switch_to_block(failblk); //define failure block
         ctx.ins().trap(TrapCode::UnreachableCodeReached);
         ctx.seal_block(failblk);

         ctx.switch_to_block(succblk); //return cfg to success block
         (JExpr {
            block: succblk,
            value: ctx.block_params(succblk)[0],
         }, JType {
            name: "Unary".to_string(),
            jtype: types::I64,
         })
      },
      Expression::Failure(_span) => unimplemented!("compile expression: Failure"),
   }
}

pub fn apply_fn<'f, S: Clone + Debug>(jmod: &mut JITModule, ctx: &mut FunctionBuilder<'f>, blk: Block, p: &Program<S>, fi: usize, args: Vec<(JExpr,JType)>) -> (JExpr,JType) {
   if let Some((je,jt)) = check_hardcoded_call(ctx, blk, p, fi, &args) {
      return (je, jt);
   }
   if let Some(FuncOrDataId::Func(fnid)) = jmod.get_name(&format!("f#{}", fi)) {
      let fnref = jmod.declare_func_in_func(fnid, ctx.func);
      let args = args.iter().map(|(e,_t)| e.value).collect::<Vec<Value>>();
      let call = ctx.ins().call(
         fnref,
         &args
      );
      let cval = ctx.inst_results(call)[0];
      return (JExpr {
         block: blk,
         value: cval,
      }, JType {
         name: "Unary".to_string(),
         jtype: types::I64,
      });
   }
   unreachable!("function undefined: f#{}", fi)
}

impl JProgram {
   //functions will not be compiled until referenced
   pub fn compile<S: Clone + Debug>(p: &Program<S>) -> JProgram {
      let builder = JITBuilder::new(cranelift_module::default_libcall_names());
      let mut module = JITModule::new(builder.unwrap());
      let mut builder_context = FunctionBuilderContext::new();
      let mut ctx = module.make_context();
      let mut _data_ctx = DataContext::new();

      for (pi,pf) in p.functions.iter().enumerate() {
         let isig = pf.args.iter().map(|_|types::I64).collect::<Vec<types::Type>>();
         if is_hardcoded(p, pi, &isig) { continue; }
         let mut sig_f = module.make_signature();
         for _ in pf.args.iter() {
            sig_f.params.push(AbiParam::new(types::I64));
         }
         sig_f.returns.push(AbiParam::new(types::I64));
         module.declare_function(
            &format!("f#{}", pi),
            Linkage::Local,
            &sig_f
         ).unwrap();
      }

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
      let mut blk = main.create_block();
      main.append_block_params_for_function_params(blk);
      main.switch_to_block(blk);

      let mut pars = Vec::new();
      for pe in p.expressions.iter() {
         pe.vars(&mut pars);
      }
      for pi in pars.iter() {
         let pv = Variable::from_u32(*pi as u32);
         main.declare_var(pv, types::I64);
         let arg_base = main.block_params(blk)[0];
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
            let (je,_jt) = compile_expr(&mut module, &mut main, blk, p, &p.expressions[pi]);
            blk = je.block;
         }
         let (je,_jt) = compile_expr(&mut module, &mut main, blk, p, &p.expressions[p.expressions.len()-1]);
         blk = je.block;
         main.ins().return_(&[je.value]);
      }

      main.seal_block(blk);
      main.finalize();

      module.define_function(fn_main, &mut ctx).unwrap();
      module.clear_context(&mut ctx);

      for fi in 0..p.functions.len() {
         compile_fn(&mut module, &mut builder_context, &p, fi);
      }

      module.finalize_definitions().unwrap();
      JProgram {
         main: module.get_finalized_function(fn_main),
      }
   }
   pub fn eval(&self, args: &[value::Value]) -> Result<value::Value,Error<String>> {
      let ptr_main = unsafe { std::mem::transmute::<_, fn(*const u128,u64) -> u128>(self.main) };
      let args = args.iter().map(|v|v.0).collect::<Vec<u128>>();
      let res = ptr_main(args.as_ptr(), args.len() as u64);
      Ok(value::Value(res))
   }
}

pub fn is_hardcoded<S: Clone + Debug>(p: &Program<S>, fi: usize, sig: &Vec<types::Type>) -> bool {
   let hardcoded = crate::recipes::cranelift::import();
   for (hsig,hdef,_hexpr,_htype) in hardcoded.iter() {
      if sig == hsig && p.functions[fi].equals(hdef) {
         return true;
      }
   }
   false
}
pub fn check_hardcoded_call<'f, S: Clone + Debug>(ctx: &mut FunctionBuilder<'f>, blk: Block, p: &Program<S>, fi: usize, args: &Vec<(JExpr,JType)>) -> Option<(JExpr,JType)> {
   let sig = args.iter().map(|(_je,jt)| jt.jtype).collect::<Vec<types::Type>>();
   let val = args.iter().map(|(je,_jt)| je.value).collect::<Vec<Value>>();
   let hardcoded = crate::recipes::cranelift::import();
   for (hsig,hdef,hexpr,htype) in hardcoded.iter() {
      if &sig == hsig && p.functions[fi].equals(hdef) {
         let rval = hexpr(ctx, val);
         return Some((
            JExpr { block: blk, value: rval },
            JType { name:"".to_string(), jtype: *htype },
         ));
      }
   }
   None
}
