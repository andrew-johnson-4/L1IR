use crate::ast::{Type};
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

fn f_u64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let arg1 = val[1].clone();
   let fref = frefs.get("range:(U64,U64)->U64[]").unwrap();
   let call = ctx.ins().call(*fref, &[arg0,arg1]);
   ctx.inst_results(call)[0]
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I64,types::I64],
      arg_types: vec![Type::nominal("U64"),Type::nominal("U64")],
      name: "range:(U64,U64)->U64[]".to_string(),
      cons: f_u64,
      symbol: None,
      rname: "Tuple".to_string(),
      rtype: types::I128,
   }
]}
