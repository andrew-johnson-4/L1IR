use crate::ast::{Type};
use crate::value;
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

pub fn s_u64(lo: u64, hi: u64) -> (u64,u64) {
   let v = value::Value::from_lohi(lo,hi);
   let mut capacity = 0;
   for ti in v.start()..v.end() {
      let vi = v.vslot(ti);
      if vi.tag() != value::Tag::Tuple { continue; }
      capacity += vi.end() - vi.start();
   }
   let n = value::Value::tuple_with_capacity(capacity as u64);
   for ti in v.start()..v.end() {
      let vi = v.vslot(ti);
      if vi.tag() != value::Tag::Tuple { continue; }
      for vti in vi.start()..vi.end() {
         n.push(vi.vslot(vti));
      }
   }
   n.lohi()
}

pub fn f_u64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let arg1 = val[1].clone();
   let fref = frefs.get(".flatten:(Tuple)->Tuple").unwrap();
   let call = ctx.ins().call(*fref, &[arg0,arg1]);
   let lo = ctx.inst_results(call)[0];
   let hi = ctx.inst_results(call)[1];
   ctx.ins().iconcat(lo,hi)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I128],
      arg_types: vec![Type::nominal("Value")],
      name: ".flatten:(Tuple)->Tuple".to_string(),
      cons: f_u64,
      symbol: Some(s_u64 as *const u8),
      rname: "Value".to_string(),
      rtype: types::I128,
   }
]}
