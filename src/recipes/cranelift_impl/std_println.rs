use crate::ast::{Type};
use crate::value;
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

pub fn s_u64(v: u128) -> u64 {
   println!("{:?}", value::Value(v));
   0
}

fn f_u64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let fref = frefs.get("println:(Value)->U64").unwrap();
   let call = ctx.ins().call(*fref, &[arg0]);
   ctx.inst_results(call)[0]
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I128],
      arg_types: vec![Type::nominal("Value")],
      name: "println:(Value)->U64".to_string(),
      cons: f_u64,
      symbol: Some(s_u64 as *const u8),
      rname: "U64".to_string(),
      rtype: types::I64,
   }
]}
