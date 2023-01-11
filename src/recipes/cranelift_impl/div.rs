use crate::ast::{FunctionDefinition,Expression,LHSPart,LHSLiteralPart,LIPart};
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
            Expression::li(vec![
               LIPart::literal("0"),
               LIPart::expression(Expression::apply(0,vec![
                  Expression::variable(2,()),
                  Expression::variable(1,()),
               ],())),
            ],())
         ),(
            LHSPart::Any,
            Expression::unary(b"0",())
         )],
      ())],
   ),|ctx,val| {
      let val0 = val[0].clone();
      let val1 = val[1].clone();
      ctx.ins().udiv(val0, val1)
   }, types::I64)
}

