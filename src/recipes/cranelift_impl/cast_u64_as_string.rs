use crate::ast::{Type};
use crate::value;
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

pub fn s_u64(t: u64) -> (u64,u64) {
   let r = value::Value::string(&format!("{}",t),"");
   r.lohi()
}
pub fn s_i64(t: i64) -> (u64,u64) {
   let r = value::Value::string(&format!("{}",t),"");
   r.lohi()
}

pub fn f_u64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let fref = frefs.get("as:(U64)->String").unwrap();
   let call = ctx.ins().call(*fref, &[arg0]);
   let lo = ctx.inst_results(call)[0];
   let hi = ctx.inst_results(call)[1];
   ctx.ins().iconcat(lo,hi)
}
pub fn f_i64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let fref = frefs.get("as:(I64)->String").unwrap();
   let call = ctx.ins().call(*fref, &[arg0]);
   let lo = ctx.inst_results(call)[0];
   let hi = ctx.inst_results(call)[1];
   ctx.ins().iconcat(lo,hi)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I64],
      arg_types: vec![Type::nominal("U64")],
      name: "as:(U64)->String".to_string(),
      cons: f_u64,
      symbol: Some(s_u64 as *const u8),
      rname: "String".to_string(),
      rtype: types::I128,
   },
   FFI {
      args: vec![types::I64],
      arg_types: vec![Type::nominal("I64")],
      name: "as:(I64)->String".to_string(),
      cons: f_i64,
      symbol: Some(s_i64 as *const u8),
      rname: "String".to_string(),
      rtype: types::I128,
   }
]}
