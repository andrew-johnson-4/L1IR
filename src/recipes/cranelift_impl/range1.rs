use crate::ast::{Type};
use crate::value;
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;

fn s_u64(to: u64) -> u128 {
   value::Value::u8(34,"U8").0
}

fn f_u64<'f>(ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let val0 = val[0].clone();
   ctx.ins().iconcat(val0, val0)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I64],
      arg_types: vec![Type::nominal("U64")],
      name: "range:(U64)->U64[]".to_string(),
      cons: f_u64,
      symbol: Some(s_u64 as *const u8),
      rname: "Tuple".to_string(),
      rtype: types::I128,
   }
]}
