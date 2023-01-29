use std::fmt::Debug;
use crate::value;
use crate::ast;
use crate::value::{Tag};
use crate::ast::{Program,Expression,LHSPart,LHSLiteralPart,LIPart,TIPart,FunctionDefinition};
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataContext, Linkage, Module, FuncOrDataId};
use num_traits::ToPrimitive;
use std::collections::HashMap;
use std::sync::{Mutex};

lazy_static! {
   static ref TYPE_CONTEXT: Mutex<HashMap<usize, String>> = {
      Mutex::new(HashMap::new())
   };
   static ref STDLIB: Mutex<HashMap<String, FFI>> = {
      let mut lib = HashMap::new();
      for ffi in crate::recipes::cranelift::import().into_iter() {
         lib.insert(ffi.name.clone(), ffi);
      }
      Mutex::new(lib)
   };
}

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

pub fn function_parameters<S: Debug + Clone>(fd: &FunctionDefinition<S>) -> Vec<types::Type> {
   fd.args.iter().map(|(_ti,tt)|type_by_name(tt)).collect::<Vec<types::Type>>()
}
pub fn function_return<S: Debug + Clone>(fd: &FunctionDefinition<S>) -> Vec<types::Type> {
   let rt = type_by_name(&fd.body[fd.body.len()-1].typ());
   vec![rt]
}

pub fn type_by_name(tn: &ast::Type) -> types::Type {
   if let Some(ref tn) = tn.name {
   match tn.as_str() {
      "U64" => types::I64,
      _ => unimplemented!("type_by_name({})", tn),
   }} else { types::I128 }
}
pub fn jtype_by_name(tn: &ast::Type) -> JType {
   if let Some(ref tn) = tn.name {
   match tn.as_str() {
      "U64" => JType { name: tn.clone(), jtype: types::I64 },
      _ => unimplemented!("type_by_name({})", tn),
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
   else { panic!("Could not cast {} as {}", ot, nt) }
}

pub fn compile_fn<'f,S: Clone + Debug>(jmod: &mut JITModule, builder_context: &mut FunctionBuilderContext, p: &Program<S>, fi: String) {
   println!("compile fn {}", fi);
   let pf = p.functions.get(&fi).unwrap();
   let hpars = function_parameters(&pf);
   let hrets = function_return(&pf);

   let mut ctx = jmod.make_context();

   let mut sig_fn = jmod.make_signature();
   for pt in hpars.iter() {
      sig_fn.params.push(AbiParam::new(*pt));
   }
   for rt in hrets.iter() {
      sig_fn.returns.push(AbiParam::new(*rt));
   }

   let fn0 = jmod
      .declare_function(&fi, Linkage::Local, &sig_fn)
      .unwrap();
   ctx.func.signature = sig_fn;

   let mut fnb = FunctionBuilder::new(&mut ctx.func, builder_context);
   let mut blk = fnb.create_block();
   fnb.append_block_params_for_function_params(blk);
   fnb.switch_to_block(blk);

   for (pi,(vi,vt)) in pf.args.iter().enumerate() {
      let ptyp = type_by_name(vt);
      let pvar = Variable::from_u32(*vi as u32);
      fnb.declare_var(pvar, ptyp);
      TYPE_CONTEXT.lock().unwrap().insert(*vi, vt.name.clone().unwrap_or("Value".to_string()));

      let pval = fnb.block_params(blk)[pi];
      fnb.def_var(pvar, pval);
   }

   if pf.body.len()==0 {
      let rval = fnb.ins().iconst(types::I64, i64::from(0));
      fnb.ins().return_(&[rval]);
   } else {
      for pi in 0..(pf.body.len()-1) {
         let (je,_jt) = compile_expr(jmod, &mut fnb, blk, p, &pf.body[pi]);
         blk = je.block;
      }
      let (je,_jt) = compile_expr(jmod, &mut fnb, blk, p, &pf.body[pf.body.len()-1]);
      blk = je.block;
      fnb.ins().return_(&[je.value]);
   }

   fnb.seal_block(blk);
   fnb.finalize();

   jmod.define_function(fn0, &mut ctx).unwrap();
   jmod.clear_context(&mut ctx);
}

pub fn compile_lhs<'f>(ctx: &mut FunctionBuilder<'f>, mut lblk: Block, rblk: Block, lhs: &LHSPart, nblk: Block, mut val: Value, typ: &str) {
   println!("compile lhs");
   ctx.switch_to_block(lblk);
   match lhs {
      LHSPart::Tuple(_lts) => unimplemented!("compile_lhs(Tuple)"),
      LHSPart::Literal(lts) => {
         let cond = if typ=="U64" {
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
            TYPE_CONTEXT.lock().unwrap().insert(*mi, typ.to_string());
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
   println!("try inline plurals");
   if let Expression::TupleIntroduction(tis,_tt,_span) = pe {
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
                  compile_lhs(ctx, current, next, lt, lblocks[li+1], header[lti], "Value");
                  current = next;
               }
            },
            LHSPart::Any => {
               compile_lhs(ctx, lblocks[li], rblocks[li], l, lblocks[li+1], noval, "Value");
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
         name: "Value".to_string(),
         jtype: types::I64,
      }))
   } else { None }
}

pub fn compile_expr<'f,S: Clone + Debug>(jmod: &mut JITModule, ctx: &mut FunctionBuilder<'f>, mut blk: Block, p: &Program<S>, e: &Expression<S>) -> (JExpr,JType) {
   println!("compile expr");
   match e {
      Expression::ValueIntroduction(ui,tt,_span) => {
      if let ast::Value::Unary(ui,_) = ui {
         let tname = tt.name.clone().unwrap_or("Value".to_string());
         if "Value" == &tname {
            println!("value introduction");
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
      Expression::LiteralIntroduction(lis,_tt,_span) => {
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
            name: "U64".to_string(),
            jtype: types::I64,
         })
      }
      Expression::TupleIntroduction(_ti,_tt,_span) => unimplemented!("compile expression: TupleIntroduction"),
      Expression::VariableReference(vi,tt,_span) => {
         println!("variable reference");
         let jv = Variable::from_u32(*vi as u32);
         let jv = ctx.use_var(jv);
         let jt = type_by_name(tt);
         let nt = tt.name.clone().unwrap_or("Value".to_string());
         let tc = TYPE_CONTEXT.lock().unwrap();
         let ot = tc.get(vi).expect(&format!("Could not find Type of Variable v#{}", vi));
         let nv = type_cast(ctx, ot, &nt, jv);
         println!("variable reference v#{} : {} as {}", vi, ot, nt);
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
            let jejt = compile_expr(jmod, ctx, blk, p, a);
            arg_types.push(jejt);
         }
         apply_fn(jmod, ctx, blk, p, fi.clone(), arg_types)
      },
      Expression::PatternMatch(pe,lrs,_tt,span) => {
         println!("pattern match");
         if let Some((je,jt)) = try_inline_plurals(jmod, ctx, blk, p, pe.as_ref(), lrs.as_ref(), span) {
            return (je,jt);
         }
         let (je,jt) = compile_expr(jmod, ctx, blk, p, pe);
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
            compile_lhs(ctx, lblocks[li], rblocks[li], l, lblocks[li+1], je.value, &jt.name);
         }

         let mut rjt = JType {
            name: "Value".to_string(),
            jtype: types::I128,
         };
         for (ri,(_l,r)) in lrs.iter().enumerate() {
            ctx.switch_to_block(rblocks[ri]);
            let (je,jt) = compile_expr(jmod, ctx, rblocks[ri], p, r);
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

pub fn apply_fn<'f, S: Clone + Debug>(jmod: &mut JITModule, ctx: &mut FunctionBuilder<'f>, blk: Block, p: &Program<S>, fi: String, args: Vec<(JExpr,JType)>) -> (JExpr,JType) {
   let mut coerced_args: Vec<(JExpr,JType)> = Vec::new();
   if let Some(ffi) = STDLIB.lock().unwrap().get(&fi) {
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
   println!("apply fn {}({})", fi,
      args.iter().map(|(_je,jt)| format!("{:?}",jt.name)).collect::<Vec<String>>().join(",")
   );
   if let Some((je,jt)) = check_hardcoded_call(ctx, blk, fi.clone(), &args) {
      return (je, jt);
   }
   if let Some(FuncOrDataId::Func(fnid)) = jmod.get_name(&fi) {
      let pf = p.functions.get(&fi).unwrap();
      let fnref = jmod.declare_func_in_func(fnid, ctx.func);
      let args = args.iter().map(|(e,_t)| e.value).collect::<Vec<Value>>();
      let call = ctx.ins().call(
         fnref,
         &args
      );
      let cval = ctx.inst_results(call)[0];
      let ftype = pf.body[pf.body.len()-1].typ();
      let rname = ftype.name.clone().unwrap_or("Value".to_string());
      let rtype = type_by_name(&ftype);
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

impl JProgram {
   //functions will not be compiled until referenced
   pub fn compile<S: Clone + Debug>(p: &Program<S>) -> JProgram {
      let builder = JITBuilder::new(cranelift_module::default_libcall_names());
      let mut module = JITModule::new(builder.unwrap());
      let mut builder_context = FunctionBuilderContext::new();
      let mut ctx = module.make_context();
      let mut _data_ctx = DataContext::new();

      println!("compile program 1");

      for (pn,pf) in p.functions.iter() {
         let isig = function_parameters(pf);
         let mut sig_f = module.make_signature();
         for ptt in isig.into_iter() {
            sig_f.params.push(AbiParam::new(ptt));
         }
         for rtt in function_return(pf).into_iter() {
            sig_f.returns.push(AbiParam::new(rtt));
         }
         module.declare_function(
            pn,
            Linkage::Local,
            &sig_f
         ).unwrap();
      }

      println!("compile program 2");

      //int main(int *args, size_t args_count);
      let mut sig_main = module.make_signature();
      sig_main.params.push(AbiParam::new(types::I64));
      sig_main.params.push(AbiParam::new(types::I64));
      sig_main.returns.push(AbiParam::new(types::I64));
      sig_main.returns.push(AbiParam::new(types::I64));

      let fn_main = module
        .declare_function("main", Linkage::Local, &sig_main)
        .unwrap();
      ctx.func.signature = sig_main;

      println!("compile program 3");

      let mut main = FunctionBuilder::new(&mut ctx.func, &mut builder_context);
      let mut blk = main.create_block();
      main.append_block_params_for_function_params(blk);
      main.switch_to_block(blk);

      println!("compile program 4");

      let mut pars = Vec::new();
      for pe in p.expressions.iter() {
         pe.vars(&mut pars);
      }
      for pi in pars.iter() {
         let pv = Variable::from_u32(*pi as u32);
         main.declare_var(pv, types::I128);
         TYPE_CONTEXT.lock().unwrap().insert(*pi, "Value".to_string());
         
         let arg_base = main.block_params(blk)[0];
         let arg_offset = (16 * *pi) as i32;
         let arg_flags = MemFlags::new();
         let arg_value = main.ins().load(types::I128, arg_flags, arg_base, arg_offset);
         main.def_var(pv, arg_value);
      }

      println!("compile program 5");

      if p.expressions.len()==0 {
         let jv = Variable::from_u32(0 as u32);
         let jv = main.use_var(jv);
         let (lval,rval) = main.ins().isplit(jv);
         main.ins().return_(&[lval,rval]);
      } else {
         println!("compile program 6.1");

         for pi in 0..(p.expressions.len()-1) {
            println!("compile program 6.2");
            let (je,_jt) = compile_expr(&mut module, &mut main, blk, p, &p.expressions[pi]);
            blk = je.block;
         }
         println!("compile program 6.3");
         let (mut je,jt) = compile_expr(&mut module, &mut main, blk, p, &p.expressions[p.expressions.len()-1]);
         je.value = type_cast(&mut main, &jt.name, "Value", je.value);
         blk = je.block;

         println!("compile program 6.4");
         let (lval,rval) = main.ins().isplit(je.value);
         main.ins().return_(&[lval,rval]);
      }

      main.seal_block(blk);
      main.finalize();

      module.define_function(fn_main, &mut ctx).unwrap();
      module.clear_context(&mut ctx);

      println!("compile program 7");

      for (fi,_f) in p.functions.iter() {
         compile_fn(&mut module, &mut builder_context, &p, fi.clone());
      }

      println!("compile program 8");

      module.finalize_definitions().unwrap();
      JProgram {
         main: module.get_finalized_function(fn_main),
      }
   }
   pub fn eval(&self, args: &[value::Value]) -> value::Value {
      let ptr_main = unsafe { std::mem::transmute::<_, fn(*const u128,u64) -> u128>(self.main) };
      let args = args.iter().map(|v|v.0).collect::<Vec<u128>>();
      let res = ptr_main(args.as_ptr(), args.len() as u64);
      value::Value(res)
   }
}

pub fn check_hardcoded_call<'f>(ctx: &mut FunctionBuilder<'f>, blk: Block, fi: String, args: &Vec<(JExpr,JType)>) -> Option<(JExpr,JType)> {
   let stdlib = STDLIB.lock().unwrap();
   if let Some(ffi) = stdlib.get(&fi) {
      let sig = args.iter().map(|(_je,jt)| jt.jtype).collect::<Vec<types::Type>>();
      if sig != ffi.args { panic!("Wrong argument types to function: {}", fi) }
      let val = args.iter().map(|(je,_jt)| je.value).collect::<Vec<Value>>();
      let rval = (ffi.cons)(ctx, &val);
      return Some((
         JExpr { block: blk, value: rval },
         JType { name: ffi.rname.clone(), jtype: ffi.rtype },
      ));
   } else { None }
}
