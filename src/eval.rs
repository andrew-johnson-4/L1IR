use std::rc::Rc;
use std::fmt::Debug;
use std::collections::{HashMap};
use crate::ast::{Error,error,Value,Expression,Program,LIPart,TIPart};

pub fn eval_e<S:Debug + Clone>(lctx: &mut HashMap<usize,Value>, pctx: &Program<S>, e: &Expression<S>) -> Result<Value,Error<S>> {
   match e {
      Expression::LiteralIntroduction(lps,span) => {
         if lps.len()==1 {
         if let LIPart::Linear(lcs) = &lps[0] {
            return Ok(Value::Literal(0,lcs.len(),lcs.clone()));
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
               }} else {
                  return Err(error("Runtime Error", &format!("v#{} not found", vi), span));
               }
            },
         }}
         Ok(Value::Literal(0,lcs.len(),Rc::new(lcs)))
      },
      Expression::TupleIntroduction(tps,span) => {
         if tps.len()==1 {
         if let TIPart::Linear(tcs) = &tps[0] {
            return Ok(Value::Tuple(0,tcs.len(),tcs.clone()));
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
               } else {
                  return Err(error("Runtime Error", &format!("v#{} not found", vi), span));
               }
            },
            TIPart::InlineVariable(vi) => {
               if let Some(Value::Tuple(vs,ve,vcs)) = lctx.get(vi) {
               for ti in *vs..*ve {
                  tcs.push(vcs[ti].clone());
               }} else { 
                  return Err(error("Runtime Error", &format!("inline tuple v#{} not found", vi), span));
               }
            },
         }}
         Ok(Value::Tuple(0,tcs.len(),Rc::new(tcs)))
      },
      Expression::FunctionReference(fi,_span) => {
         Ok(Value::Function(*fi))
      },
      Expression::VariableReference(li,span) => {
         if let Some(lv) = lctx.get(li) {
            Ok(lv.clone())
         } else {
            return Err(error("Runtime Error", &format!("v#{} not found", li), span));
         }
      },
      Expression::FunctionApplication(fi,pxs,span) => {
         if *fi>pctx.functions.len() {
            return Err(error("Runtime Error", &format!("f#{} undefined", fi), span));
         }
         if pxs.len()!=pctx.functions[*fi].args.len() {
            return Err(error("Runtime Error", &format!("f#{} called with wrong arity", fi), span));
         }
         let mut ps = Vec::new();
         for px in pxs.iter() {
            ps.push(eval_e(lctx, pctx, px)?);
         }
         let mut new_ctx = HashMap::new();
         for (pi,pv) in std::iter::zip(&pctx.functions[*fi].args, ps) {
            new_ctx.insert(*pi, pv);
         }
         let mut top_value = Value::tuple(Vec::new());
         for be in pctx.functions[*fi].body.iter() {
            top_value = eval_e(&mut new_ctx, pctx, be)?;
         }
         Ok(top_value)
      },
      Expression::PatternMatch(span) => unimplemented!("eval_e(PatternMatch) at {:?}", span),
      Expression::Failure(span) => {
         return Err(error("Runtime Error", "Failure", span));
      },
   }
}

pub fn eval<S:Debug + Clone>(p: Program<S>) -> Result<Value,Error<S>> {
   let mut top_value = Value::tuple(Vec::new());
   let mut top_ctx = HashMap::new();
   for e in p.expressions.iter() {
      top_value = eval_e(&mut top_ctx, &p, e)?;
   }
   Ok(top_value)
}
