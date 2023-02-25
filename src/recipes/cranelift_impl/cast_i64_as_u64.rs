use crate::ast::{Type};
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

pub fn f_u64<'f>(_frefs: &HashMap<String,FuncRef>, _ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   val[0].clone()
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I64],
      arg_types: vec![Type::nominal("I64")],
      name: "as:(I64)->U64".to_string(),
      cons: f_u64,
      symbol: None,
      rname: "U64".to_string(),
      rtype: types::I64,
   }
]}
