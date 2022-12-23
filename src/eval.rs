use std::rc::Rc;
use std::collections::{HashMap};
use crate::ast::{Value,Expression,Program,LIPart,TIPart};

pub fn eval_e(lctx: &mut HashMap<usize,Value>, pctx: &Program, e: &Expression) -> Value {
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
            LIPart::InlineVariable(vi) => {
               if let Some(Value::Literal(vs,ve,vcs)) = lctx.get(vi) {
               for ci in *vs..*ve {
                  lcs.push(vcs[ci]);
               }} else { panic!("v#{} not found", vi) }
            },
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
            TIPart::Variable(vi) => {
               if let Some(vt) = lctx.get(vi) {
                  tcs.push(vt.clone());
               } else { panic!("v#{} not found", vi) }
            },
            TIPart::InlineVariable(vi) => {
               if let Some(Value::Tuple(vs,ve,vcs)) = lctx.get(vi) {
               for ti in *vs..*ve {
                  tcs.push(vcs[ti].clone());
               }} else { panic!("inline tuple v#{} not found", vi) }
            },
         }}
         Value::Tuple(0,tcs.len(),Rc::new(tcs))
      },
      Expression::FunctionReference(fi) => {
         Value::Function(*fi)
      },
      Expression::VariableReference(li) => {
         if let Some(lv) = lctx.get(li) {
            lv.clone()
         } else { panic!("eval_e(v#{})", li) }
      },
      Expression::FunctionApplication(fi,pxs) => {
         if *fi>pctx.functions.len() { panic!("f#{} undefined", fi); }
         if pxs.len()!=pctx.functions[*fi].args.len() { panic!("f#{} call with wrong arity", fi); }
         let mut ps = Vec::new();
         for px in pxs.iter() {
            ps.push(eval_e(lctx, pctx, px));
         }
         let mut new_ctx = HashMap::new();
         for (pi,pv) in std::iter::zip(&pctx.functions[*fi].args, ps) {
            new_ctx.insert(*pi, pv);
         }
         let mut top_value = Value::tuple(Vec::new());
         for be in pctx.functions[*fi].body.iter() {
            top_value = eval_e(&mut new_ctx, pctx, be);
         }
         top_value
      },
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
