use crate::ast::{Type};
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

fn f_u64<'f>(_frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let val0 = val[0].clone();
   ctx.ins().icmp_imm(IntCC::Equal, val0, 0)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I8],
      arg_types: vec![Type::nominal("U8")],
      name: "not:(U8)->U8".to_string(),
      cons: f_u64,
      symbol: None,
      rname: "U8".to_string(),
      rtype: types::I8,
   }
]}
