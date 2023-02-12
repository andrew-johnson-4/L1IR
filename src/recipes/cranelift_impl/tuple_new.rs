use crate::ast::{Type};
use crate::value;
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

pub fn s_u64(t: u64) -> u128 {
   println!("with_capacity({})", t);
   value::Value::tuple_with_capacity(t).0
}

pub fn f_u64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let fref = frefs.get("with_capacity:(U64)->Tuple").unwrap();
   let call = ctx.ins().call(*fref, &[arg0]);
   ctx.inst_results(call)[0]
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I64],
      arg_types: vec![Type::nominal("U64")],
      name: "with_capacity:(U64)->Tuple".to_string(),
      cons: f_u64,
      symbol: Some(s_u64 as *const u8),
      rname: "Tuple".to_string(),
      rtype: types::I128,
   }
]}
