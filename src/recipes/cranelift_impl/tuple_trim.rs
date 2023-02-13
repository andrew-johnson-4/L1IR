use crate::ast::{Type};
use crate::value;
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

pub fn s_u64(t_lo: u64, t_hi: u64) -> (u64,u64) {
   let mut v = value::Value::from_lohi(t_lo,t_hi);
   let mut ei = v.end();
   for i in v.start()..v.end() {
      if v.vslot(i).0 == 0 {
         ei = i;
         break;
      }
   }
   v.set_end(ei);
   v.lohi()
}

pub fn f_u64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let fref = frefs.get("trim_capacity:(Tuple)->Tuple").unwrap();
   let call = ctx.ins().call(*fref, &[arg0]);
   let lo = ctx.inst_results(call)[0];
   let hi = ctx.inst_results(call)[1];
   ctx.ins().iconcat(lo,hi)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I128],
      arg_types: vec![Type::nominal("Tuple")],
      name: "trim_capacity:(Tuple)->Tuple".to_string(),
      cons: f_u64,
      symbol: Some(s_u64 as *const u8),
      rname: "Tuple".to_string(),
      rtype: types::I128,
   }
]}
