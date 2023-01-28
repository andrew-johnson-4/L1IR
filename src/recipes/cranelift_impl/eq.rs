use crate::ast::{FunctionDefinition,Expression,LHSPart,LHSLiteralPart,Type};
use cranelift::prelude::*;

pub fn import<'f>() -> (Vec<types::Type>,FunctionDefinition<()>,fn(&mut FunctionBuilder<'f>,Vec<Value>) -> Value,String,types::Type) {
      (vec![types::I64,types::I64],
       FunctionDefinition::define(
          vec![(0,Type::nominal("U64")), (1,Type::nominal("U64"))],
          vec![Expression::pattern(
             Expression::variable(0,()),
             vec![(
                LHSPart::ul(
                   vec![LHSLiteralPart::variable(1)],
                   None,
                   vec![],
                ),
                Expression::unary(b"1",()),
             ),(
                LHSPart::Any,
                Expression::unary(b"0",()),
             )],
          ())],
      ),|ctx,val| {
         let val0 = val[0].clone();
         let val1 = val[1].clone();
         let vi8 = ctx.ins().icmp(IntCC::Equal, val0, val1);
         ctx.ins().uextend(types::I64, vi8)
      }, "U64".to_string(), types::I64)
}

