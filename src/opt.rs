use std::fmt::Debug;
use std::borrow::Borrow;
use crate::value;
use crate::ast;
use crate::value::{Tag};
use crate::ast::{Program,Expression,LHSPart,LHSLiteralPart,LIPart,TIPart,FunctionDefinition};
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module, FuncOrDataId, FuncId};
use cranelift_codegen::settings::{self, Configurable};
use cranelift_codegen::ir::FuncRef;
use cranelift_native;
use num_traits::ToPrimitive;
use std::collections::HashMap;

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

pub fn function_parameters<S: Debug + Clone>(fd: &FunctionDefinition<S>) -> Vec<types::Type> {
   fd.args.iter().map(|(_ti,tt)|type_by_name(tt)).collect::<Vec<types::Type>>()
}
pub fn function_return<S: Debug + Clone>(fd: &FunctionDefinition<S>) -> types::Type {
   let rt = type_by_name(&fd.body[fd.body.len()-1].typ());
   rt
}

pub fn type_by_name(tn: &ast::Type) -> types::Type {
   if let Some(ref tn) = tn.name {
   match tn.as_str() {
      "U8" => types::I8,
      "U64" => types::I64,
      "String" => types::I128,
      "Tuple" => types::I128,
      "Value" => types::I128,
      _ => unimplemented!("type_by_name({})", tn),
   }} else { types::I128 }
}
pub fn jtype_by_name(tn: &ast::Type) -> JType {
   if let Some(ref tn) = tn.name {
   match tn.as_str() {
      "U8" => JType { name: tn.clone(), jtype: types::I8 },
      "U64" => JType { name: tn.clone(), jtype: types::I64 },
      "String" => JType { name: tn.clone(), jtype: types::I128 },
      "Tuple" => JType { name: tn.clone(), jtype: types::I128 },
      "Value" => JType { name: tn.clone(), jtype: types::I128 },
      _ => unimplemented!("jtype_by_name({})", tn),
   }} else { JType { name: "Value".to_string(), jtype: types::I128 } }
}

pub fn type_cast<'f>(ctx: &mut FunctionBuilder<'f>, ot: &str, nt: &str, v: Value) -> Value {
   if ot == nt { v }
   else if ot=="Value" && nt=="U64" {
      let (low64,high64) = ctx.ins().isplit(v);
      let high16 = ctx.ins().ushr_imm(high64, 48);
      let aeq = ctx.ins().icmp_imm(IntCC::Equal, high16, (Tag::U64 as u16) as i64);
      ctx.ins().trapz(aeq, TrapCode::BadConversionToInteger);
      low64
   }
   else if ot=="U64" && nt=="Value" {
      let high64 = ((Tag::U64 as u16) as u64) * (2_u64.pow(48));
      let high64 = unsafe { std::mem::transmute::<u64,i64>(high64) };
      let high64 = ctx.ins().iconst(types::I64, high64);
      ctx.ins().iconcat(v, high64)
   }
   else if ot=="U8" && nt=="Value" {
      let low64  = ctx.ins().uextend(types::I64, v);
      let high64 = ((Tag::U8 as u16) as u64) * (2_u64.pow(48));
      let high64 = unsafe { std::mem::transmute::<u64,i64>(high64) };
      let high64 = ctx.ins().iconst(types::I64, high64);
      ctx.ins().iconcat(low64, high64)
   }
   else if ot=="String" && nt=="Value" { v }
   else if ot=="Value" && nt=="String" { v }
   else if ot=="Tuple" && nt=="Value" { v }
   else if ot=="Value" && nt=="Tuple" { v }
   else { panic!("Could not cast {} as {}", ot, nt) }
}

pub fn compile_fn<'f,S: Clone + Debug>(type_context: &mut HashMap<usize, String>, stdlib: &mut HashMap<String,FFI>, global_finfs: &Vec<(String,FuncId)>, jmod: &mut JITModule, builder_context: &mut FunctionBuilderContext, p: &Program<S>, fi: String) {
   let pf = p.functions.get(&fi).unwrap();
   let hpars = function_parameters(&pf);
   let hrets = function_return(&pf);

   let mut ctx = jmod.make_context();
   let mut sig_fn = jmod.make_signature();
   for pt in hpars.iter() {
      if *pt == types::I128 {
         sig_fn.params.push(AbiParam::new(types::I64));
         sig_fn.params.push(AbiParam::new(types::I64));
      } else {
         sig_fn.params.push(AbiParam::new(*pt));
      }
   }
   if hrets == types::I128 {
      sig_fn.returns.push(AbiParam::new(types::I64));
      sig_fn.returns.push(AbiParam::new(types::I64));
   } else {
      sig_fn.returns.push(AbiParam::new(hrets));
   }
   ctx.func.signature = sig_fn;

   let mut fnb = FunctionBuilder::new(&mut ctx.func, builder_context);
   let mut finfs = inject_stdlib_locals(jmod, &mut fnb, global_finfs);
   let mut blk = fnb.create_block();
   fnb.append_block_params_for_function_params(blk);
   fnb.switch_to_block(blk);

   for (pi,(vi,vt)) in pf.args.iter().enumerate() {
      let ptyp = type_by_name(vt);
      let pvar = Variable::from_u32(*vi as u32);
      fnb.declare_var(pvar, ptyp);
      type_context.insert(*vi, vt.name.clone().unwrap_or("Value".to_string()));

      let pval = fnb.block_params(blk)[pi];
      fnb.def_var(pvar, pval);
   }

   if pf.body.len()==0 {
      if hrets == types::I128 {
         let zero = fnb.ins().iconst(types::I64, 0);
         fnb.ins().return_(&[zero,zero]);
      } else {
         let zero = fnb.ins().iconst(types::I64, 0);
         fnb.ins().return_(&[zero]);
      }
   } else {
      for pi in 0..(pf.body.len()-1) {
         let (je,_jt) = compile_expr(type_context, stdlib, &mut finfs, jmod, &mut fnb, blk, p, &pf.body[pi]);
         blk = je.block;
      }
      let (je,_jt) = compile_expr(type_context, stdlib, &mut finfs, jmod, &mut fnb, blk, p, &pf.body[pf.body.len()-1]);
      blk = je.block;
      if hrets == types::I128 {
         let r = je.value;
         let (rlo,rhi) = fnb.ins().isplit(r);
         fnb.ins().return_(&[rlo,rhi]); 
      } else {
         fnb.ins().return_(&[je.value]);
      }
   }

   fnb.seal_block(blk);
   fnb.finalize();

   let Some(FuncOrDataId::Func(fn0)) = jmod.get_name(&fi)
   else { panic!("Could not find local function: {}", fi) };
   jmod.define_function(fn0, &mut ctx).unwrap();
   jmod.clear_context(&mut ctx);
}

pub fn compile_lhs<'f>(type_context: &mut HashMap<usize, String>, ctx: &mut FunctionBuilder<'f>, mut lblk: Block, rblk: Block, lhs: &LHSPart, nblk: Block, mut val: Value, typ: &str) {
   ctx.switch_to_block(lblk);
   match lhs {
      LHSPart::Tuple(_lts) => unimplemented!("compile_lhs(Tuple)"),
      LHSPart::Literal(lts) => {
         let cond = if typ=="U8" {
            let v = lts.parse::<u64>().unwrap() as i64;
            ctx.ins().icmp_imm(IntCC::Equal, val, v)
         } else if typ=="U64" {
            let v = lts.parse::<u64>().unwrap();
            let v = unsafe { std::mem::transmute::<u64,i64>(v) };
            ctx.ins().icmp_imm(IntCC::Equal, val, v)
         } else {
            unimplemented!("compile_lhs(Literal:{})", typ)
         };
         ctx.ins().brnz(cond, rblk, &[]);
         ctx.ins().jump(nblk, &[]);
      },
      LHSPart::UnpackLiteral(pres,mid,sufs) => {
         for p in pres.iter() {
         if let LHSLiteralPart::Literal(cs) = p {
            let sub = if typ=="U64" {
               let v = cs.parse::<u64>().unwrap();
               unsafe { std::mem::transmute::<u64,i64>(v) }
            } else {
               unimplemented!("compile_lhs(Literal:{})", typ)
            };
            let cond = ctx.ins().icmp_imm(IntCC::UnsignedLessThan, val, sub);
            let bb = ctx.create_block(); //basic blocks can't compute after jump
            ctx.ins().brnz(cond, nblk, &[]);
            ctx.ins().jump(bb, &[]);
            ctx.seal_block(lblk);
            ctx.switch_to_block(bb);
            lblk = bb;
            let len = ctx.ins().iconst(types::I64, sub);
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
            type_context.insert(*mi, typ.to_string());
            ctx.def_var(jv, val);
         }
         ctx.ins().jump(rblk, &[]);
      },
      LHSPart::Variable(vi) => {
         let jv = Variable::from_u32(*vi as u32);
         ctx.declare_var(jv, types::I64);
         type_context.insert(*vi, typ.to_string());
         ctx.def_var(jv, val);
      },
      LHSPart::Any => {
         ctx.ins().jump(rblk, &[]);
      },
   }
   ctx.seal_block(lblk);
}

pub fn compile_expr<'f,S: Clone + Debug>(type_context: &mut HashMap<usize, String>, stdlib: &mut HashMap<String,FFI>, finfs: &mut HashMap<String,FuncRef>, jmod: &mut JITModule, ctx: &mut FunctionBuilder<'f>, mut blk: Block, p: &Program<S>, e: &Expression<S>) -> (JExpr,JType) {
   match e {
      Expression::Map(lhs,iterable,x,_tt,_span) => {
         let (je,_jt) = compile_expr(type_context, stdlib, finfs, jmod, ctx, blk, p, iterable);
         blk = je.block;
         let (e_lo,e_hi) = ctx.ins().isplit(je.value);

         let map_len = *finfs.get(".length:(Tuple)->U64").unwrap();
         let map_len = ctx.ins().call(map_len,&[e_lo,e_hi]);
         let map_len = ctx.inst_results(map_len)[0];

         let map_new = *finfs.get("with_capacity:(U64)->Tuple").unwrap();
         let map_new = ctx.ins().call(map_new,&[map_len]);
         let map_new_lo = ctx.inst_results(map_new)[0];
         let map_new_hi = ctx.inst_results(map_new)[1];

         let loop_controller = ctx.create_block();
         ctx.append_block_param(loop_controller, types::I64);

         let mut in_loop = ctx.create_block();
         ctx.append_block_param(in_loop, types::I64);

         let after_loop = ctx.create_block();

         let zero = ctx.ins().iconst(types::I64, 0);
         ctx.ins().jump(loop_controller, &[zero]); //start loop at i=0
         //seal blk

         ctx.switch_to_block(loop_controller);     //loop while i < map_len
         let i = ctx.block_params(loop_controller)[0];
         let cond = ctx.ins().icmp(IntCC::UnsignedLessThan, i, map_len);
         ctx.ins().brnz(cond, in_loop, &[i]);
         ctx.ins().jump(after_loop, &[]);
         //seal loop_controller

         ctx.switch_to_block(in_loop);
         let i = ctx.block_params(in_loop)[0];
         let ii = *finfs.get("[]:(Tuple,U64)->Value").unwrap();
         let ii = ctx.ins().call(ii,&[e_lo,e_hi,i]);
         let ii_lo = ctx.inst_results(ii)[0];
         let ii_hi = ctx.inst_results(ii)[1];
         let ii = ctx.ins().iconcat(ii_lo, ii_hi);
         match (*lhs).borrow() {
            LHSPart::Any => {},
            LHSPart::Variable(vi) => {
               println!("declare variable: {}", vi);
               let jv = Variable::from_u32(*vi as u32);
               ctx.declare_var(jv, types::I128);
               type_context.insert(*vi, "Value".to_string());
               ctx.def_var(jv, ii);
            },
            _ => panic!("Invalid IR: for loop bindings must not be fallible")
         }
         let x_val = match x.borrow() {
            TIPart::Tuple(_ts) => unimplemented!(".flatmap Tuple"),
            TIPart::Variable(vi) => {
               println!("use variable: {}", vi);
               let jv = Variable::from_u32(*vi as u32);
               ctx.use_var(jv)
            }
            TIPart::InlineVariable(_vi) => unimplemented!(".flatmap InlineVariable"),
            TIPart::Expression(xe) => {
               let mut xe = xe;
               if let Expression::PatternMatch(guard_cond,plrs,_ptt,_span) = xe.borrow() {
               if plrs.len()==1 {
               if let (LHSPart::Literal(guard_literal),guarded) = &plrs[0] { 
               if guard_literal=="1" {
                  let (lie,_lit) = compile_expr(type_context, stdlib, finfs, jmod, ctx, in_loop, p, guard_cond.borrow());
                  in_loop = lie.block;

                  let skip = ctx.create_block();
                  let push = ctx.create_block();
                  ctx.ins().brz(lie.value, skip, &[]);
                  ctx.ins().jump(push, &[]);
                  ctx.seal_block(in_loop);

                  ctx.switch_to_block(skip);
                  let i = ctx.ins().iadd_imm(i, 1);
                  ctx.ins().jump(loop_controller, &[i]);
                  ctx.seal_block(skip);

                  ctx.switch_to_block(push);
                  in_loop = push;
                  xe = guarded;
               }}}}
               let (lie,_lit) = compile_expr(type_context, stdlib, finfs, jmod, ctx, in_loop, p, xe.borrow());
               in_loop = lie.block;
               lie.value
            }
         };
         let (x_val_lo,x_val_hi) = ctx.ins().isplit(x_val);

         let xi = *finfs.get(".push:(Tuple,Value)->U64").unwrap();
         ctx.ins().call(xi,&[map_new_lo,map_new_hi,x_val_lo,x_val_hi]);
         let i = ctx.ins().iadd_imm(i, 1);
         ctx.ins().jump(loop_controller, &[i]);
         //seal in_loop

         ctx.seal_block(in_loop);
         ctx.seal_block(loop_controller);
         ctx.seal_block(blk);                      //seal iterable expression block
         ctx.switch_to_block(after_loop);
         let map_out = *finfs.get("trim_capacity:(Tuple)->Tuple").unwrap();
         let map_out = ctx.ins().call(map_out,&[map_new_lo,map_new_hi]);
         let map_out_lo = ctx.inst_results(map_out)[0];
         let map_out_hi = ctx.inst_results(map_out)[1];
         let map_out = ctx.ins().iconcat(map_out_lo,map_out_hi);

         (JExpr {
            block: after_loop,
            value: map_out,
         }, JType {
            name: "Value".to_string(),
            jtype: types::I128,
         })
      },
      Expression::ValueIntroduction(ui,tt,_span) => {
      if let ast::Value::Unary(ui,_) = ui {
         let tname = tt.nom();
         if "Value" == &tname {
            let ui = ui.to_i64().unwrap();
            let vlow = ctx.ins().iconst(types::I64, ui);
            let vhigh = ctx.ins().iconst(types::I64, (Tag::U64 as i64) * (2_i64.pow(48)));
            (JExpr {
               block: blk,
               value: ctx.ins().iconcat(vlow, vhigh),
            }, JType {
               name: "Value".to_string(),
               jtype: types::I128,
            })
         } else if "U64" == &tname {
            let ui = ui.to_i64().unwrap();
            let v = ctx.ins().iconst(types::I64, ui);
            (JExpr {
               block: blk,
               value: v,
            }, JType {
               name: "U64".to_string(),
               jtype: types::I64,
            })
         } else {
            unimplemented!("compile expression Value::Unary({:?})", ui)
         }
      } else {
         unimplemented!("compile expression {:?}", ui)
      }},
      Expression::LiteralIntroduction(lis,tt,_span) => {
         if tt.nom() == "Unit" {
            let v = value::Value::from_parts(value::Tag::Unit as u16, 0, 0).0;
            let high = (v >> 64) as i64;
            let low = ((v << 64) >> 64) as i64;
            let high = ctx.ins().iconst(types::I64, high);
            let low = ctx.ins().iconst(types::I64, low);
            let val = ctx.ins().iconcat(low, high);
            (JExpr {
               block: blk,
               value: val,
            }, JType {
               name: "Value".to_string(),
               jtype: types::I128,
            })
         } else if tt.nom() == "U64" {
            let mut val = ctx.ins().iconst(types::I64, 0);
            for li in lis.iter() {
            match li {
               LIPart::Expression(e) => {
                  let (je,_jt) = compile_expr(type_context, stdlib, finfs, jmod, ctx, blk, p, e);
                  blk = je.block;
                  val = ctx.ins().iadd(val, je.value);
               },
               LIPart::Literal(cs) => {
                  let v = cs.parse::<u64>().expect("U64");
                  let v = unsafe { std::mem::transmute::<u64,i64>(v) };
                  val = ctx.ins().iadd_imm(val, v);
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
               name: "U64".to_string(),
               jtype: types::I64,
            })
         } else if tt.nom() == "Value" {
            //just checking
            let v = value::Value::from_parts(value::Tag::U64 as u16, 0, 123).0;
            let high = (v >> 64) as i64;
            let low = ((v << 64) >> 64) as i64;
            let high = ctx.ins().iconst(types::I64, high);
            let low = ctx.ins().iconst(types::I64, low);
            let val = ctx.ins().iconcat(low, high);
            (JExpr {
               block: blk,
               value: val,
            }, JType {
               name: "Value".to_string(),
               jtype: types::I128,
            })
         } else if tt.nom() == "String" {
            let mut s_value = Vec::new();
            for li in lis.iter() {
            match li {
               LIPart::Literal(cs) => {
                  for c in cs[1..cs.len()-1].chars() {
                     s_value.push(c);
                  }
               },
               LIPart::Expression(_e) => {
                  unimplemented!("Introduce String Literal Expression")
               },
               LIPart::InlineVariable(_vi) => {
                  unimplemented!("Introduce String Literal Inline Variable")
               },
            }}
            let s_len = ctx.ins().iconst(types::I64, s_value.len() as i64);
            let s_new = *finfs.get("with_capacity:(U64)->String").unwrap();
            let s_new = ctx.ins().call(s_new,&[s_len]);
            let s_lo = ctx.inst_results(s_new)[0];
            let s_hi = ctx.inst_results(s_new)[1];
            for c in s_value.iter() {
               let s_c = ctx.ins().iconst(types::I32, *c as i64);
               let si = *finfs.get(".push:(String,U32)->U64").unwrap();
               ctx.ins().call(si,&[s_lo,s_hi,s_c]);
            }
            let s_new = ctx.ins().iconcat(s_lo, s_hi);
            (JExpr {
               block: blk,
               value: s_new,
            }, JType {
               name: "String".to_string(),
               jtype: types::I128,
            })
         } else {
            unimplemented!("Unknown literal introduction: {:?}", tt)
         }
      }
      Expression::TupleIntroduction(ts,_tt,_span) => {
         if ts.len()==1 {
         if let TIPart::Tuple(tes) = &ts[0] {
            let map_len = ctx.ins().iconst(types::I64, tes.len() as i64);
            let map_new = *finfs.get("with_capacity:(U64)->Tuple").unwrap();
            let map_new = ctx.ins().call(map_new,&[map_len]);
            let map_lo = ctx.inst_results(map_new)[0];
            let map_hi = ctx.inst_results(map_new)[1];

            for te in tes.iter() {
               let (je,_jt) = compile_expr(type_context, stdlib, finfs, jmod, ctx, blk, p, te.borrow());
               blk = je.block;
               let (je_lo,je_hi) = ctx.ins().isplit(je.value);
               let xi = *finfs.get(".push:(Tuple,Value)->U64").unwrap();
               
               ctx.ins().call(xi,&[map_lo,map_hi,je_lo,je_hi]);
            }
            let map_new = ctx.ins().iconcat(map_lo,map_hi);

            return (JExpr {
               block: blk,
               value: map_new,
            }, JType {
               name: "Value".to_string(),
               jtype: types::I128,
            })
         }}
         unimplemented!("compile_expr Expression::TupleIntroduction")
      },
      Expression::VariableReference(vi,tt,_span) => {
         let jv = Variable::from_u32(*vi as u32);
         let jv = ctx.use_var(jv);
         let jt = type_by_name(tt);
         let nt = tt.name.clone().unwrap_or("Value".to_string());
         let ot = type_context.get(vi).expect(&format!("Could not find Type of Variable v#{}", vi));
         let nv = type_cast(ctx, ot, &nt, jv);
         (JExpr {
            block: blk,
            value: nv
         }, JType {
            name: nt,
            jtype: jt,
         })
      },
      Expression::FunctionReference(_vi,_tt,_span) => unimplemented!("compile expression: FunctionReference"),
      Expression::FunctionApplication(fi,args,_tt,_span) => {
         let mut arg_types = Vec::new();
         for a in args.iter() {
            let (je,jt) = compile_expr(type_context, stdlib, finfs, jmod, ctx, blk, p, a);
            blk = je.block;
            arg_types.push((je,jt));
         }
         apply_fn(stdlib, finfs, jmod, ctx, blk, p, fi.clone(), arg_types)
      },
      Expression::PatternMatch(pe,lrs,tt,_span) => {
         let mut rjt = JType {
            name: tt.name.clone().unwrap_or("Value".to_string()),
            jtype: type_by_name(tt),
         };

         let (je,jt) = compile_expr(type_context, stdlib, finfs, jmod, ctx, blk, p, pe);
         blk = je.block;

         let failblk = ctx.create_block(); //failure block
         let succblk = ctx.create_block(); //success block
         let st = type_by_name(&lrs[lrs.len()-1].1.typ());
         ctx.append_block_param(succblk, st);

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
            compile_lhs(type_context, ctx, lblocks[li], rblocks[li], l, lblocks[li+1], je.value, &jt.name);
         }

         for (ri,(_l,r)) in lrs.iter().enumerate() {
            ctx.switch_to_block(rblocks[ri]);
            let (je,jt) = compile_expr(type_context, stdlib, finfs, jmod, ctx, rblocks[ri], p, r);
            rjt = jt;
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
         }, rjt)
      },
      Expression::Failure(_tt,_span) => unimplemented!("compile expression: Failure"),
   }
}

pub fn apply_fn<'f, S: Clone + Debug>(stdlib: &mut HashMap<String,FFI>, finfs: &mut HashMap<String,FuncRef>, jmod: &mut JITModule, ctx: &mut FunctionBuilder<'f>, blk: Block, p: &Program<S>, fi: String, args: Vec<(JExpr,JType)>) -> (JExpr,JType) {
   let mut coerced_args: Vec<(JExpr,JType)> = Vec::new();
   if let Some(ffi) = stdlib.get(&fi) {
      for (ji,(mut je,mut jt)) in args.into_iter().enumerate() {
         let nt = jtype_by_name(&ffi.arg_types[ji]);
         je.value = type_cast(ctx, &jt.name, &nt.name, je.value);
         jt = nt;
         coerced_args.push((je, jt));
      }
   } else if let Some(pf) = p.functions.get(&fi) {
      for (ji,(mut je,mut jt)) in args.into_iter().enumerate() {
         let nt = jtype_by_name(&pf.args[ji].1);
         je.value = type_cast(ctx, &jt.name, &nt.name, je.value);
         jt = nt;
         coerced_args.push((je, jt));
      }
   } else {
      panic!("attempt to apply undefined function, fn {}", fi)
   };
   let args = coerced_args;
   if let Some((je,jt)) = check_hardcoded_call(stdlib, finfs, ctx, blk, fi.clone(), &args) {
      return (je, jt);
   }
   if let Some(FuncOrDataId::Func(fnid)) = jmod.get_name(&fi) {
      let pf = p.functions.get(&fi).unwrap();
      let fnref = if let Some(fnref) = finfs.get(&fi) { 
         *fnref
      } else {
         let fnref = jmod.declare_func_in_func(fnid, ctx.func);
         finfs.insert(fi, fnref);
         fnref
      };
      let mut cargs = Vec::new();
      for (ce,ct) in args.iter() {
         if ct.jtype == types::I128 {
            let (clo,chi) = ctx.ins().isplit(ce.value);
            cargs.push(clo);
            cargs.push(chi);
         } else {
            cargs.push(ce.value);
         }
      }
      let call = ctx.ins().call(
         fnref,
         &cargs
      );
      let ftype = pf.body[pf.body.len()-1].typ();
      let rname = ftype.name.clone().unwrap_or("Value".to_string());
      let rtype = type_by_name(&ftype);
      let cval = if rtype == types::I128 {
         let clo = ctx.inst_results(call)[0];
         let chi = ctx.inst_results(call)[1];
         ctx.ins().iconcat(clo,chi)
      } else {
         ctx.inst_results(call)[0]
      };
      return (JExpr {
         block: blk,
         value: cval,
      }, JType {
         name: rname,
         jtype: rtype,
      });
   }
   unreachable!("function undefined: {}", fi)
}

fn inject_stdlib_symbols(module: &mut JITModule, stdlib: &mut HashMap<String,FFI>) -> Vec<(String,FuncId)> {
   let mut fs = Vec::new();
   for (sk,sv) in stdlib.iter() {
   if let Some(_) = sv.symbol {
      let mut sig_s = module.make_signature();
      for at in sv.args.iter() {
         if *at == types::I128 {
            sig_s.params.push(AbiParam::new(types::I64));
            sig_s.params.push(AbiParam::new(types::I64));
         } else {
            sig_s.params.push(AbiParam::new(*at));
         }
      }
      if sv.rtype == types::I128 {
         sig_s.returns.push(AbiParam::new(types::I64));
         sig_s.returns.push(AbiParam::new(types::I64));
      } else {
         sig_s.returns.push(AbiParam::new(sv.rtype));
      }

      let func_s = module
        .declare_function(sk, Linkage::Import, &sig_s)
        .unwrap();
      fs.push((sk.clone(), func_s));
   }}
   fs
}

fn inject_stdlib_locals<'f>(module: &mut JITModule, ctx: &mut FunctionBuilder<'f>, finfs: &Vec<(String,FuncId)>) -> HashMap<String,FuncRef> {
   let mut locs = HashMap::new();
   for (k,fi) in finfs.iter() {
      let l = module.declare_func_in_func(*fi, &mut ctx.func);
      locs.insert(k.clone(), l);
   }
   locs
}

impl JProgram {
   //functions will not be compiled until referenced
   pub fn compile<S: Clone + Debug>(p: &Program<S>) -> JProgram {
      p.dump_l1ir();

      let mut type_context: HashMap<usize, String> = HashMap::new();
      let mut stdlib: HashMap<String, FFI> = {
         let mut lib = HashMap::new();
         for ffi in crate::recipes::cranelift::import().into_iter() {
            lib.insert(ffi.name.clone(), ffi);
         }
         lib
      };

      let mut flag_builder = settings::builder();
      flag_builder.set("use_colocated_libcalls", "false").unwrap();
      flag_builder.set("is_pic", "true").unwrap();
      //flag_builder.set("enable_llvm_abi_extensions", "true").unwrap();
      //flag_builder.set("opt_level", "speed").unwrap();
      let isa_builder = cranelift_native::builder().unwrap_or_else(|msg| {
          panic!("host machine is not supported: {}", msg);
      });
      let isa = isa_builder.finish(settings::Flags::new(flag_builder)).expect("Failed to build Cranelift ISA");

      let mut builder = JITBuilder::with_isa(isa, cranelift_module::default_libcall_names());
      for (sk,sv) in stdlib.iter() {
      if let Some(addr) = sv.symbol {
         builder.symbol(sk, addr);
      }}       

      let mut module = JITModule::new(builder);
      let global_finfs = inject_stdlib_symbols(&mut module, &mut stdlib);
      let mut builder_context = FunctionBuilderContext::new();
      let mut ctx = module.make_context();
      let mut _data_ctx = DataContext::new();

      for (pn,pf) in p.functions.iter() {
         let isig = function_parameters(pf);
         let mut sig_f = module.make_signature();
         for ptt in isig.into_iter() {
            if ptt == types::I128 {
               sig_f.params.push(AbiParam::new(types::I64));
               sig_f.params.push(AbiParam::new(types::I64));
            } else {
               sig_f.params.push(AbiParam::new(ptt));
            }
         }
         let rtt = function_return(pf);
         if rtt == types::I128 {
            sig_f.returns.push(AbiParam::new(types::I64));
            sig_f.returns.push(AbiParam::new(types::I64));
         } else {
            sig_f.returns.push(AbiParam::new(rtt));
         }
         module.declare_function(
            pn,
            Linkage::Local,
            &sig_f
         ).unwrap();
      }

      //int main(int *args, size_t args_count);
      let mut sig_main = module.make_signature();
      sig_main.params.push(AbiParam::new(types::I64));
      sig_main.params.push(AbiParam::new(types::I64));
      sig_main.returns.push(AbiParam::new(types::I64));
      sig_main.returns.push(AbiParam::new(types::I64));

      let fn_main = module
        .declare_function("main", Linkage::Export, &sig_main)
        .unwrap();
      ctx.func.signature = sig_main;

      let mut main = FunctionBuilder::new(&mut ctx.func, &mut builder_context);
      let mut finfs = inject_stdlib_locals(&mut module, &mut main, &global_finfs);

      let mut blk = main.create_block();
      main.append_block_params_for_function_params(blk);
      main.switch_to_block(blk);

      let mut pars = Vec::new();
      for pe in p.expressions.iter() {
         pe.vars(&mut pars);
      }
      let mut pi_declared = Vec::new();
      for pi in pars.iter() {
         if pi_declared.contains(pi) { continue; }
         pi_declared.push(*pi);
         let pv = Variable::from_u32(*pi as u32);
         main.declare_var(pv, types::I128);
         type_context.insert(*pi, "Value".to_string());
         
         let arg_base = main.block_params(blk)[0];
         let arg_offset = (16 * *pi) as i32;
         let arg_flags = MemFlags::new();
         let arg_value = main.ins().load(types::I128, arg_flags, arg_base, arg_offset);
         main.def_var(pv, arg_value);
      }

      if p.expressions.len()==0 {
         let zero = main.ins().iconst(types::I64, 0);
         main.ins().return_(&[zero,zero]);
      } else {
         for pi in 0..(p.expressions.len()-1) {
            let (je,_jt) = compile_expr(&mut type_context, &mut stdlib, &mut finfs, &mut module, &mut main, blk, p, &p.expressions[pi]);
            blk = je.block;
         }
         let (mut je,jt) = compile_expr(&mut type_context, &mut stdlib, &mut finfs, &mut module, &mut main, blk, p, &p.expressions[p.expressions.len()-1]);
         je.value = type_cast(&mut main, &jt.name, "Value", je.value);
         blk = je.block;

         let (jlo,jhi) = main.ins().isplit(je.value);
         main.ins().return_(&[jlo,jhi]);
      }

      main.seal_block(blk);
      main.finalize();

      module.define_function(fn_main, &mut ctx).unwrap();
      module.clear_context(&mut ctx);

      for (fi,_f) in p.functions.iter() {
         compile_fn(&mut type_context, &mut stdlib, &global_finfs, &mut module, &mut builder_context, &p, fi.clone());
      }

      module.finalize_definitions().unwrap();
      JProgram {
         main: module.get_finalized_function(fn_main),
      }
   }
   pub fn eval(&self, args: &[value::Value]) -> value::Value {
      let ptr_main = unsafe { std::mem::transmute::<_, fn(u64,u64) -> (u64,u64)>(self.main) };
      let args = args.iter().map(|v|v.0).collect::<Vec<u128>>();
      let (rlo,rhi) = ptr_main(args.as_ptr() as u64, args.len() as u64);
      let res = ((rhi as u128) << 64) | (rlo as u128);
      value::Value(res)
   }
}

pub fn check_hardcoded_call<'f>(stdlib: &mut HashMap<String,FFI>, finfs: &mut HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, blk: Block, fi: String, args: &Vec<(JExpr,JType)>) -> Option<(JExpr,JType)> {
   if let Some(ffi) = stdlib.get(&fi) {
      let sig = args.iter().map(|(_je,jt)| jt.jtype).collect::<Vec<types::Type>>();
      if sig != ffi.args { panic!("Wrong argument types to function: {}", fi) }
      let mut val = Vec::new();
      for (je,jt) in args.iter() {
         if jt.jtype == types::I128 {
            let (jlo,jhi) = ctx.ins().isplit(je.value);
            val.push(jlo);
            val.push(jhi);
         } else {
            val.push(je.value);
         }
      }
      let rval = (ffi.cons)(finfs, ctx, &val);
      return Some((
         JExpr { block: blk, value: rval },
         JType { name: ffi.rname.clone(), jtype: ffi.rtype },
      ));
   } else { None }
}
