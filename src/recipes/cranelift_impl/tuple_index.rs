use crate::ast::{Type};
use crate::value;
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

pub fn s_u64(t_lo: u64, t_hi: u64, i: u64) -> (u64,u64) {
   let v = value::Value::from_lohi(t_lo, t_hi);
   let vi = v.vslot(i as usize);
   vi.lohi()
}

pub fn f_u64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let arg1 = val[1].clone();
   let fref = frefs.get("[]:(Tuple,U64)->Value").unwrap();
   let call = ctx.ins().call(*fref, &[arg0,arg1]);
   let lo = ctx.inst_results(call)[0];
   let hi = ctx.inst_results(call)[1];
   ctx.ins().iconcat(lo,hi)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I128,types::I64],
      arg_types: vec![Type::nominal("Value"),Type::nominal("U64")],
      name: "[]:(Tuple,U64)->Value".to_string(),
      cons: f_u64,
      symbol: Some(s_u64 as *const u8),
      rname: "Value".to_string(),
      rtype: types::I128,
   }
]}
