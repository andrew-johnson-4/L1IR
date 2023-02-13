use crate::ast::{Type};
use crate::value;
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;
use std::io::Write;

pub fn s_u64(t_lo: u64, t_hi: u64) -> u64 {
   dprintln!(".length");
   let t = value::Value::from_lohi(t_lo,t_hi);
   dprintln!(".length({:?})", t);
   (t.end() - t.start()) as u64
}

pub fn f_u64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let arg1 = val[1].clone();
   let fref = frefs.get(".length:(Tuple)->U64").unwrap();
   let call = ctx.ins().call(*fref, &[arg0,arg1]);
   let lo = ctx.inst_results(call)[0];
   let hi = ctx.inst_results(call)[1];
   ctx.ins().iconcat(lo,hi)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I128],
      arg_types: vec![Type::nominal("Tuple")],
      name: ".length:(Tuple)->U64".to_string(),
      cons: f_u64,
      symbol: Some(s_u64 as *const u8),
      rname: "U64".to_string(),
      rtype: types::I64,
   }
]}
