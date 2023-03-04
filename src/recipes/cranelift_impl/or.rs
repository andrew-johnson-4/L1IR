use crate::ast::{Type};
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

fn f_u64<'f>(_frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let val0 = val[0].clone();
   let val1 = val[1].clone();
   ctx.ins().bor(val0, val1)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I8,types::I8],
      arg_types: vec![Type::nominal("U8"), Type::nominal("U8")],
      name: "||:(U8,U8)->U8".to_string(),
      cons: f_u64,
      symbol: None,
      rname: "U8".to_string(),
      rtype: types::I8,
   }
]}
