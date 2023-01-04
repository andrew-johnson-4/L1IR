use std::rc::Rc;
use std::cell::RefCell;
use num_bigint::{BigUint,ToBigUint};
use std::fmt::Debug;
use num_traits::cast::ToPrimitive;
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
         if let Value::Literal(rs,re,rcs,_tt) = rval {
            for li in 0..(re-rs) {
            if lcs[li] != rcs[rs+li] {
               return false;
            }}
            true
         } else if let Value::Unary(ri,_tt) = rval {
            if &lcs.len().to_biguint().unwrap() != ri { return false; }
            for li in 0..lcs.len() {
            if lcs[li] != '0' {
               return false;
            }}
            true
         } else { false }
      },
      LHSPart::UnpackLiteral(prel,midl,sufl) => {
         if let Value::Unary(lu,_tt) = rval {
            let mut lu = lu.clone();
            for pl in prel.iter() {
            if let LHSLiteralPart::Literal(pcs) = pl {
               if pcs.len().to_biguint().unwrap() > lu { return false; }
               for pc in pcs.iter() {
               if pc != &'0' {
                  return false;
               }}
               lu = lu - pcs.len().to_biguint().unwrap();
            } else if let LHSLiteralPart::Variable(pid) = pl {
               if let Some(Value::Unary(pu,_)) = lctx.borrow().get(pid) {
                  if &lu < pu { return false; }
                  lu = lu - pu;
               } else {
                  unimplemented!("UnpackLiteral prefix")
               }
            }}
            for sl in sufl.iter() {
            if let LHSLiteralPart::Literal(scs) = sl {
               if scs.len().to_biguint().unwrap() > lu { return false; }
               for sc in scs.iter() {
               if sc != &'0' {
                  return false;
               }}
               lu = lu - scs.len().to_biguint().unwrap();
            } else if let LHSLiteralPart::Variable(sid) = sl {
               if let Some(Value::Unary(su,_)) = lctx.borrow().get(sid) {
                  if &lu < su { return false; }
                  lu = lu - su;
               } else {
                  unimplemented!("UnpackLiteral suffix")
               }
            }}
            if let Some(midl) = midl {
               lctx.borrow_mut().insert(*midl, Value::Unary(lu,None));
               true
            } else {
               let lc = lu == (0).to_biguint().unwrap();
               lc
            }
         } else if let Value::Literal(_ls,_le,_lcs,_tt) = rval {
            unimplemented!("TODO: unpack literal {:?}", rval)
         } else { return false; }
      },
      LHSPart::Tuple(lcs) => {
         if let Value::Tuple(rs,re,rts,_tt) = rval {
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

fn ucatcs(lui: &mut BigUint, lcs: &mut Vec<char>, vs:usize, ve:usize, v: &Vec<char>) {
   let mut unary = lcs.len()==0;
   if unary {
      for vi in vs..ve {
      if v[vi] != '0' {
         unary = false;
         break;
      }}
      if unary {
         *lui = lui.clone() + (ve-vs).to_biguint().unwrap();
      }
   }
   if !unary {
      for vi in vs..ve {
         lcs.push(v[vi]);
      }
   }
}
fn ucatu(lui: &mut BigUint, lcs: &mut Vec<char>, u:&BigUint) {
   if lcs.len()==0 {
      *lui = lui.clone() + u.clone();
   } else {
      let u = u.to_u32().unwrap();
      for _ in 0..u {
         lcs.push('0');
      }
   }
}

pub fn eval_e<S:Debug + Clone>(mut lctx: Rc<RefCell<HashMap<usize,Value>>>, pctx: &Program<S>, mut e: Expression<S>) -> Result<Value,Error<S>> {
   loop {
   match e {
      Expression::UnaryIntroduction(ui,_span) => {
         return Ok(Value::Unary(ui,None))
      },
      Expression::LiteralIntroduction(lps,span) => {
         if lps.len()==1 {
         if let LIPart::Literal(lcs) = &lps[0] {
            return Ok(Value::Literal(0,lcs.len(),lcs.clone(),None));
         }}
         let mut lui = 0.to_biguint().unwrap();
         let mut lcs = Vec::new();
         for lip in lps.iter() {
         match lip {
            LIPart::Literal(cs) => { ucatcs(&mut lui, &mut lcs, 0, cs.len(), cs) }
            LIPart::InlineVariable(vi) => {
               match lctx.borrow().get(vi) {
                  Some(Value::Literal(vs,ve,vcs,_tt)) => { ucatcs(&mut lui, &mut lcs, *vs, *ve, &vcs) },
                  Some(Value::Unary(ui,_tt)) => { ucatu(&mut lui, &mut lcs, &ui) },
                  _ => { return Err(error("Runtime Error", &format!("v#{} not found", vi), &span)); }
               }
            },
            LIPart::Expression(pe) => {
               match eval_e(lctx.clone(), pctx, pe.clone())? {
                  Value::Literal(vs,ve,vcs,_tt) => { ucatcs(&mut lui, &mut lcs, vs, ve, &vcs) },
                  Value::Unary(ui,_tt) => { ucatu(&mut lui, &mut lcs, &ui) },
                  _ => { return Err(error("Runtime Error", "invalid literal expression", &span)) }
               }
            },
         }}
         if lcs.len()==0 {
            return Ok(Value::Unary(lui,None))
         } else {
            return Ok(Value::Literal(0,lcs.len(),Rc::new(lcs),None));
         }
      },
      Expression::TupleIntroduction(tps,span) => {
         if tps.len()==1 {
         if let TIPart::Tuple(tcs) = &tps[0] {
            return Ok(Value::Tuple(0,tcs.len(),tcs.clone(),None));
         }}
         let mut tcs = Vec::new();
         for tip in tps.iter() {
         match tip {
            TIPart::Tuple(vs) => {
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
               if let Some(Value::Tuple(vs,ve,vcs,None)) = lctx.borrow().get(vi) {
               for ti in *vs..*ve {
                  tcs.push(vcs[ti].clone());
               }} else { 
                  return Err(error("Runtime Error", &format!("inline tuple v#{} not found", vi), &span));
               }
            },
         }}
         return Ok(Value::Tuple(0,tcs.len(),Rc::new(tcs),None));
      },
      Expression::FunctionReference(fi,_span) => {
         return Ok(Value::Function(fi,None));
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
            break;
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
