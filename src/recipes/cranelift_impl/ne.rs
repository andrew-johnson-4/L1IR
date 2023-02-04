use crate::ast::{Type};
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;

fn f_u64<'f>(ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let val0 = val[0].clone();
   let val1 = val[1].clone();
   let vi8 = ctx.ins().icmp(IntCC::NotEqual, val0, val1);
   ctx.ins().uextend(types::I64, vi8)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I64,types::I64],
      arg_types: vec![Type::nominal("U64"), Type::nominal("U64")],
      name: "!=:(U64,U64)->U64".to_string(),
      cons: f_u64,
      rname: "U64".to_string(),
      rtype: types::I64,
   }
]}
