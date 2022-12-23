use std::collections::{HashMap};
use crate::ast::{Value,Expression,Program};

pub fn eval_e(_lctx: &mut HashMap<usize,Value>, _pctx: &Program, e: &Expression) -> Value {
   match e {
      Expression::LiteralIntroduction(_lps) => unimplemented!("eval_e(LiteralIntroduction)"),
      Expression::TupleIntroduction(_tps) => unimplemented!("eval_e(TupleIntroduction)"),
      Expression::VariableReference(_l) => unimplemented!("eval_e(VariableReference)"),
      Expression::FunctionApplication(_fx,_pxs) => unimplemented!("eval_e(FunctionApplication)"),
      Expression::PatternMatch => unimplemented!("eval_e()"),
      Expression::Failure => panic!("eval_e(Failure)"),
   }
}

pub fn eval(p: Program) -> Value {
   let mut top_value = Value::tuple(Vec::new());
   let mut top_ctx = HashMap::new();
   for e in p.expressions.iter() {
      top_value = eval_e(&mut top_ctx, &p, e);
   }
   top_value
}
