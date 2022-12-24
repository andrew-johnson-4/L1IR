use std::rc::Rc;
use std::cell::RefCell;
use num_bigint::{ToBigUint};
use std::fmt::Debug;
use std::collections::{HashMap};
use crate::ast::{Error,error,Value,Expression,Program,LIPart,TIPart,LHSPart,LHSLiteralPart};

pub fn eval_lhs<S:Debug + Clone>(lctx: Rc<RefCell<HashMap<usize,Value>>>, pctx: &Program<S>, lhs: &LHSPart, rval: &Value) -> bool {
   match lhs {
      LHSPart::Any => true,
      LHSPart::Variable(lid) => {
         if let Some(lval) = lctx.borrow().get(lid) {
            return lval == rval;
         }
         lctx.borrow_mut().insert(*lid, rval.clone());
         true
      },
      LHSPart::Literal(lcs) => {
         if let Value::Literal(rs,re,rcs) = rval {
            for li in 0..(re-rs) {
            if lcs[li] != rcs[rs+li] {
               return false;
            }}
            true
         } else if let Value::Unary(ri) = rval {
            if &lcs.len().to_biguint().unwrap() != ri { return false; }
            for li in 0..lcs.len() {
            if lcs[li] != '0' {
               return false;
            }}
            true
         } else { false }
      },
      LHSPart::UnpackLiteral(prel,midl,sufl) => {
         if let Value::Unary(lu) = rval {
            let mut lu = lu.clone();
            for pl in prel.iter() {
            let LHSLiteralPart::Literal(pcs) = pl;
               if pcs.len().to_biguint().unwrap() > lu { return false; }
               for pc in pcs.iter() {
               if pc != &'0' {
                  return false;
               }}
               lu = lu - pcs.len().to_biguint().unwrap();
            }
            for sl in sufl.iter() {
            let LHSLiteralPart::Literal(scs) = sl;
               if scs.len().to_biguint().unwrap() > lu { return false; }
               for sc in scs.iter() {
               if sc != &'0' {
                  return false;
               }}
               lu = lu - scs.len().to_biguint().unwrap();
            }
            if let Some(midl) = midl {
               lctx.borrow_mut().insert(*midl, Value::Unary(lu));
            }
            true
         } else {
            unimplemented!("TODO: LHSPart::UnpackLiteral")
         }
      },
      LHSPart::Tuple(lcs) => {
         if let Value::Tuple(rs,re,rts) = rval {
            if lcs.len() != (re-rs) { return false; }
            for li in 0..lcs.len() {
            if !eval_lhs(lctx.clone(), pctx, &lcs[li], &rts[rs+li]) {
               return false;
            }}
            true
         } else { false }
      },
   }
}

pub fn eval_e<S:Debug + Clone>(mut lctx: Rc<RefCell<HashMap<usize,Value>>>, pctx: &Program<S>, mut e: Expression<S>) -> Result<Value,Error<S>> {
   loop {
   match e {
      Expression::UnaryIntroduction(ui,_span) => {
         return Ok(Value::Unary(ui))
      },
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
               if let Some(Value::Literal(vs,ve,vcs)) = lctx.borrow().get(vi) {
               for ci in *vs..*ve {
                  lcs.push(vcs[ci]);
               }} else {
                  return Err(error("Runtime Error", &format!("v#{} not found", vi), &span));
               }
            },
         }}
         return Ok(Value::Literal(0,lcs.len(),Rc::new(lcs)));
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
               if let Some(vt) = lctx.borrow().get(vi) {
                  tcs.push(vt.clone());
               } else {
                  return Err(error("Runtime Error", &format!("v#{} not found", vi), &span));
               }
            },
            TIPart::InlineVariable(vi) => {
               if let Some(Value::Tuple(vs,ve,vcs)) = lctx.borrow().get(vi) {
               for ti in *vs..*ve {
                  tcs.push(vcs[ti].clone());
               }} else { 
                  return Err(error("Runtime Error", &format!("inline tuple v#{} not found", vi), &span));
               }
            },
         }}
         return Ok(Value::Tuple(0,tcs.len(),Rc::new(tcs)));
      },
      Expression::FunctionReference(fi,_span) => {
         return Ok(Value::Function(fi));
      },
      Expression::VariableReference(li,span) => {
         if let Some(lv) = lctx.borrow().get(&li) {
            return Ok(lv.clone());
         } else {
            return Err(error("Runtime Error", &format!("v#{} not found", li), &span));
         }
      },
      Expression::FunctionApplication(fi,pxs,span) => {
         if fi>pctx.functions.len() {
            return Err(error("Runtime Error", &format!("f#{} undefined", fi), &span));
         }
         if pxs.len()!=pctx.functions[fi].args.len() {
            return Err(error("Runtime Error", &format!("f#{} called with wrong arity", fi), &span));
         }
         let mut ps = Vec::new();
         for px in pxs.iter() {
            ps.push(eval_e(lctx.clone(), pctx, px.clone())?);
         }
         let mut new_ctx = HashMap::new();
         for (pi,pv) in std::iter::zip(&pctx.functions[fi].args, ps) {
            new_ctx.insert(*pi, pv);
         }
         let ref bes = pctx.functions[fi].body;
         if bes.len()==0 {
            return Ok(Value::tuple(Vec::new()));
         }
         let new_ctx = Rc::new(RefCell::new(new_ctx));
         for bi in 0..(bes.len()-1) {
            eval_e(new_ctx.clone(), pctx, bes[bi].clone())?;
         }
         lctx = new_ctx;
         e = bes[bes.len()-1].clone();
      },
      Expression::PatternMatch(re,lrs,span) => {
         let mut matched: Option<Expression<S>> = None;
         let rv = eval_e(lctx.clone(), pctx, (*re).clone())?;
         for (l,r) in lrs.iter() {
         if eval_lhs(lctx.clone(), pctx, l, &rv) {
            matched = Some(r.clone());
         }
         }
         if let Some(ne) = matched {
            e = ne.clone();
         } else {
            return Err(error("Runtime Error", &format!("pattern did not match on {:?}", rv), &span));
         }
      },
      Expression::Failure(span) => {
         return Err(error("Runtime Error", "failure", &span));
      },
   }}
}

pub fn eval<S:Debug + Clone>(p: Program<S>) -> Result<Value,Error<S>> {
   let mut top_value = Value::tuple(Vec::new());
   let top_ctx = Rc::new(RefCell::new(HashMap::new()));
   for e in p.expressions.iter() {
      top_value = eval_e(top_ctx.clone(), &p, e.clone())?;
   }
   Ok(top_value)
}
