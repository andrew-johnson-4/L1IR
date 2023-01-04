use crate::ast::{FunctionDefinition,Expression,LIPart};
use cranelift::prelude::*;

pub fn import<'f>() -> (Vec<types::Type>,FunctionDefinition<()>,fn(&mut FunctionBuilder<'f>,Vec<Value>) -> Value,types::Type) {
   (vec![types::I64,types::I64],
    FunctionDefinition::define(
       vec![0,1],
       vec![Expression::li(vec![
          LIPart::variable(0),
          LIPart::variable(1),
       ],())]
    ),|ctx,val| {
       let val0 = val[0].clone();
       let val1 = val[1].clone();
       ctx.ins().iadd(val0, val1)
    }, types::I64)
}
