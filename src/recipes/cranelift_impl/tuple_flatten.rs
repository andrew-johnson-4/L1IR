use crate::ast::{Type};
use crate::value;
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

fn s_u64(t: u128) -> u128 {
   let v = value::Value(t);
   let mut capacity = 0;
   for ti in v.start()..v.end() {
      let vi = v.vslot(ti);
      capacity += vi.end() - vi.start();
   }
   let n = value::Value::tuple_with_capacity(capacity as u64);
   for ti in v.start()..v.end() {
      let vi = v.vslot(ti);
      for vti in vi.start()..v.end() {
         n.push(vi.vslot(vti));
      }
   }
   n.0
}

fn f_u64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let fref = frefs.get(".flatten:(Tuple)->Tuple").unwrap();
   let call = ctx.ins().call(*fref, &[arg0]);
   ctx.inst_results(call)[0]
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
