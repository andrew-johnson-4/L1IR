use crate::ast::{FunctionDefinition,Expression,LIPart,Type};
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;

fn f_u64<'f>(ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let val0 = val[0].clone();
   let val1 = val[1].clone();
   ctx.ins().udiv(val0, val1)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I64,types::I64],
      fdef: FunctionDefinition::define(
         "/:(U64,U64)->U64",
         vec![(0,Type::nominal("U64")), (1,Type::nominal("U64"))],
         vec![Expression::li(vec![
            LIPart::variable(0),
            LIPart::variable(1),
         ],())]
      ),
      cons: f_u64,
      rname: "U64".to_string(),
      rtype: types::I64,
   }
]}
