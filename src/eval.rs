use std::rc::Rc;
use std::collections::{HashMap};
use crate::ast::{Value,Expression,Program,LIPart,TIPart};

pub fn eval_e(_lctx: &mut HashMap<usize,Value>, _pctx: &Program, e: &Expression) -> Value {
   match e {
      Expression::LiteralIntroduction(lps) => {
         if lps.len()==1 {
         if let LIPart::Linear(lcs) = &lps[0] {
            return Value::Literal(0,lcs.len(),lcs.clone());
         }}
         let mut lcs = Vec::new();
         for lip in lps.iter() {
         match lip {
            LIPart::Linear(cs) => {
            for c in cs.iter() {
               lcs.push(*c);
            }},
            LIPart::Variable(_v) => {},
         }}
         Value::Literal(0,lcs.len(),Rc::new(lcs))
      },
      Expression::TupleIntroduction(tps) => {
         if tps.len()==1 {
         if let TIPart::Linear(tcs) = &tps[0] {
            return Value::Tuple(0,tcs.len(),tcs.clone());
         }}
         let mut tcs = Vec::new();
         for tip in tps.iter() {
         match tip {
            TIPart::Linear(vs) => {
            for v in vs.iter() {
               tcs.push(v.clone());
            }},
            TIPart::Variable(_v) => {},
         }}
         Value::Tuple(0,tcs.len(),Rc::new(tcs))
      },
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
