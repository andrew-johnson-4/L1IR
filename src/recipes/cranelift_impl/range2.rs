use crate::ast::{Type};
use crate::value;
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

pub fn s_i64(from: i64, to: i64) -> (u64,u64) {
   value::Value::range(from, to, 1).lohi()
}

fn f_i64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let arg1 = val[1].clone();
   let fref = frefs.get("range:(I64,I64)->I64[]").unwrap();
   let call = ctx.ins().call(*fref, &[arg0,arg1]);
   let lo = ctx.inst_results(call)[0];
   let hi = ctx.inst_results(call)[1];
   ctx.ins().iconcat(lo,hi)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I64,types::I64],
      arg_types: vec![Type::nominal("I64"),Type::nominal("I64")],
      name: "range:(I64,I64)->I64[]".to_string(),
      cons: f_i64,
      symbol: Some(s_i64 as *const u8),
      rname: "Value".to_string(),
      rtype: types::I128,
   }
]}
