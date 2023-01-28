use crate::ast::{FunctionDefinition,Expression,LIPart,Type};
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;

pub fn import<'f>() -> Vec<FFI<'f>> {vec![
   FFI {
      args: vec![types::I64,types::I64],
      fdef: FunctionDefinition::define(
         "+:(U64,U64)->U64",
         vec![(0,Type::nominal("U64")), (1,Type::nominal("U64"))],
         vec![Expression::li(vec![
            LIPart::variable(0),
            LIPart::variable(1),
         ],())]
      ),
      cons: |ctx,val| {
         let val0 = val[0].clone();
         let val1 = val[1].clone();
         ctx.ins().iadd(val0, val1)
      },
      rname: "U64".to_string(),
      rtype: types::I64,
   }
]}
