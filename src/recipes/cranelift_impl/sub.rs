use crate::ast::{FunctionDefinition,Expression,LHSPart,LHSLiteralPart};
use cranelift::prelude::*;

pub fn import<'f>() -> (Vec<types::Type>,FunctionDefinition<()>,fn(&mut FunctionBuilder<'f>,Vec<Value>) -> Value,types::Type) {
   (vec![types::I64,types::I64],
   FunctionDefinition::define(
      vec![0,1],
      vec![Expression::pattern(
         Expression::variable(0,()),
         vec![(
            LHSPart::ul(
               vec![LHSLiteralPart::variable(1)],
               Some(2),
               vec![],
            ),
            Expression::variable(2,()),
         )],
      ())],
   ),|ctx,val| {
      let val0 = val[0].clone();
      let val1 = val[1].clone();
      ctx.ins().isub(val0, val1)
   }, types::I64)
}

