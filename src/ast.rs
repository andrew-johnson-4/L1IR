use num_bigint::{BigUint,ToBigUint};
use std::collections::HashMap;
use std::sync::Arc;
use std::fmt::Debug;

pub struct Error<S:Debug + Clone> {
   pub error_type: String,
   pub error_msg: String,
   pub span: S,
}
impl<S:Debug + Clone> std::fmt::Debug for Error<S> {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}: {} at {:?}", self.error_type, self.error_msg, self.span)
   }
}
pub fn error<S:Debug + Clone>(t:&str, m:&str, s:&S) -> Error<S> {
   Error {
      error_type: t.to_string(),
      error_msg: m.to_string(),
      span: s.clone()
   }
}

#[derive(Clone)]
pub struct Type {
   pub name: Option<String>,
   pub regex: Option<String>,
   pub strct: Option<Vec<Type>>,
   pub fnid: Option<String>,
   pub invariants: Vec<String>,
}
impl std::fmt::Debug for Type {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.nom())
   }
}
impl Type {
   pub fn nom(&self) -> String {
      self.name.clone().unwrap_or("Value".to_string())
   }
   pub fn default() -> Type {
      Type {
         name: None,
         regex: None,
         strct: None,
         fnid: None,
         invariants: vec![],
      }
   }
   pub fn nominal(n: &str) -> Type {
      Type {
         name: Some(n.to_string()),
         regex: None,
         strct: None,
         fnid: None,
         invariants: vec![],
      }
   }
   pub fn regex(r: &str) -> Type {
      Type {
         name: None,
         regex: Some(r.to_string()),
         strct: None,
         fnid: None,
         invariants: vec![],
      }
   }
   pub fn function(f: &str) -> Type {
      Type {
         name: None,
         regex: None,
         strct: None,
         fnid: Some(f.to_string()),
         invariants: vec![],
      }
   }
   pub fn reject<S:Debug + Clone>(msg: &str, span: S) -> Error<S> {
      Error {
         error_type: "Dynamic Type Error".to_string(),
         error_msg: msg.to_string(),
         span: span
      }
   }
   pub fn accepts<S:Debug + Clone>(v: &Value, constraint: &Type, span: S) -> Result<(),Error<S>> {
      if let Some(ref _cr) = constraint.regex {
         unimplemented!("Check literal constraint")
      }
      if let Some(ref _cstrct) = constraint.strct {
         unimplemented!("Check structural constraint")
      }
      if let Some(cfi) = &constraint.fnid {
         if let Value::Function(vfi,_) = v {
         if cfi != vfi { return Err(Type::reject(
            &format!("Function #{} does not satisfy constraint: {:?}", vfi, constraint),
            span
         )); }} else { return Err(Type::reject(
            &format!("Value {:?} is not a function", v),
            span
         )); }
      }
      if let Some(ref cnom) = constraint.name {
         if let Some(ref vnom) = v.typof() {
            if cnom != vnom { return Err(Type::reject(
               &format!("Type {:?} does not satisfy constraint: {:?}", vnom, constraint),
               span
            )); }
         } else { return Err(Type::reject(
            &format!("Type ? does not satisfy constraint: {:?}", constraint),
            span
         )); }
      }
      Ok(())
   }
   pub fn accepts_any<S:Debug + Clone>(v: &Value, constraints: &Vec<Type>, span: S) -> Result<(),Error<S>> {
      let mut accepts = false;
      for cc in constraints.iter() {
      if Type::accepts(v, cc, span.clone()).is_ok() {
         accepts = true;
      }}
      if accepts {
         Ok(())
      } else { Err(Type::reject(
         &format!("Value {:?} does not satisfy any constraint", v),
         span
      )) }
   }
}

#[derive(Clone)]
pub enum Value {
   Unary(BigUint,Option<String>), //a unary number, represented as "0"...
   Literal(usize,usize,Arc<Vec<char>>,Option<String>), //avoid copy-on-slice
   Tuple(usize,usize,Arc<Vec<Value>>,Option<String>), //avoid copy-on-slice
   Function(String,Option<String>), //all functions are static program indices
}
impl Value {
   pub fn typof<'a>(&'a self) -> &'a Option<String> {
      match self {
         Value::Unary(_,tt) => tt,
         Value::Literal(_,_,_,tt) => tt,
         Value::Tuple(_,_,_,tt) => tt,
         Value::Function(_,tt) => tt,
      }
   }
   pub fn from_u64(v: u64) -> Value {
      let ui = BigUint::from(v);
      Value::Unary(ui,None)
   }
   pub fn unary(buf: &[u8]) -> Value {
      let ui = BigUint::parse_bytes(buf, 10).expect("unary parse_bytes failed");
      Value::Unary(ui,None)
   }
   pub fn literal(cs: &str) -> Value {
      let cs = cs.chars().collect::<Vec<char>>();
      Value::Literal(0,cs.len(),Arc::new(cs),None)
   }
   pub fn tuple(ts: Vec<Value>) -> Value {
      Value::Tuple(0,ts.len(),Arc::new(ts),None)
   }
   pub fn function(fid: &str) -> Value {
      Value::Function(fid.to_string(),None)
   }
   pub fn typed(self, tt: &str) -> Value {
      let tt = tt.to_string();
      match self {
         Value::Unary(ui,_) => Value::Unary(ui,Some(tt)),
         Value::Literal(cs,ce,cvs,_) => Value::Literal(cs,ce,cvs,Some(tt)),
         Value::Tuple(ts,te,tvs,_) => Value::Tuple(ts,te,tvs,Some(tt)),
         Value::Function(fi,_) => Value::Function(fi,Some(tt)),
      }
   }
}
impl PartialEq for Value {
   fn eq(&self, other: &Self) -> bool {
      match (self, other) {
         (Value::Literal(ls,le,lv,_ltt),Value::Literal(rs,re,rv,_rtt)) if (le-ls)==(re-rs) => {
            for i in 0..(le-ls) {
            if lv[ls+i] != rv[rs+i] {
               return false;
            }}
            true
         },
         (Value::Tuple(ls,le,lv,_ltt),Value::Tuple(rs,re,rv,_rtt)) if (le-ls)==(re-rs) => {
            for i in 0..(le-ls) {
            if lv[ls+i] != rv[rs+i] {
               return false;
            }}
            true
         },
         (Value::Function(lf,_ltt),Value::Function(rf,_rtt)) => {
            lf == rf
         },
         (Value::Unary(li,_ltt),Value::Unary(ri,_rtt)) => {
            li == ri
         },
         (Value::Unary(li,_ltt),Value::Literal(rs,re,rv,_rtt)) => {
            for ri in *rs..*re {
            if rv[ri] != '0' {
               return false;
            }}
            li == &(re-rs).to_biguint().unwrap()
         },
         (Value::Literal(ls,le,lv,_ltt),Value::Unary(ri,_rtt)) => {
            for li in *ls..*le {
            if lv[li] != '0' {
               return false;
            }}
            ri == &(le-ls).to_biguint().unwrap()
         },
         _ => false,
      }
   }
}
impl Eq for Value {}
impl std::fmt::Debug for Value {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      if let Value::Literal(start,end,val,_tt) = self {
         let mut unary = true;
         for i in (*start)..(*end) {
         if val[i] != '0' {
            unary = false;
            break;
         }}
         if unary {
            return write!(f, "{}", end-start);
         }
         write!(f, r#"""#)?;
         for i in (*start)..(*end) {
            write!(f, "{}", val[i])?;
         }
         write!(f, r#"""#)
      } else if let Value::Tuple(start,end,val,_tt) = self {
         write!(f, r#"("#)?;
         for i in (*start)..(*end) {
            if i>(*start) {
               write!(f, r",")?;
            }
            write!(f, "{:?}", val[i])?;
         }
         if (*end-*start)==1 {
            write!(f, r",")?;
         }
         write!(f, r#")"#)
      } else if let Value::Function(fid,_tt) = self {
         write!(f, "{}", fid)
      } else if let Value::Unary(ui,_tt) = self {
         write!(f, "{}", ui)
      } else { unreachable!("exhaustive") }
   }
}

#[derive(Clone)]
pub struct FunctionDefinition<S:Debug + Clone> {
   pub name: String,
   pub args: Vec<(usize,Type)>,
   pub body: Vec<Expression<S>>,
}
impl<S:Debug + Clone> FunctionDefinition<S> {
   pub fn define(name: &str, args: Vec<(usize,Type)>, body: Vec<Expression<S>>) -> FunctionDefinition<S> {
      FunctionDefinition {
         name: name.to_string(),
         args: args,
         body: body,
      }
   }
   pub fn equals(&self, other: &FunctionDefinition<()>) -> bool {
      let self_args = self.args.iter().map(|(vi,_vt)| *vi).collect::<Vec<usize>>();
      let other_args = other.args.iter().map(|(vi,_vt)| *vi).collect::<Vec<usize>>();
      self.name == other.name &&
      self_args == other_args &&
      self.body.len() == other.body.len() &&
      std::iter::zip(self.body.iter(),other.body.iter()).all(|(l,r)| l.equals(r))
   }
}

#[derive(Clone)]
pub struct Program<S:Debug + Clone> {
   pub functions: HashMap<String,FunctionDefinition<S>>,
   pub expressions: Vec<Expression<S>>,
}
impl<S:Debug + Clone> Program<S> {
   pub fn program(functions: Vec<FunctionDefinition<S>>, expressions: Vec<Expression<S>>) -> Program<S> {
      let mut fs = HashMap::new();
      for f in functions.into_iter() {
         fs.insert(f.name.clone(), f);
      }
      Program {
         functions: fs,
         expressions: expressions,
      }
   }
}

#[derive(Clone)]
pub enum LIPart<S:Debug + Clone> {
   Literal(String),
   InlineVariable(usize),
   Expression(Expression<S>),
}
impl<S:Debug + Clone> LIPart<S> {
   pub fn vars(&self, vars: &mut Vec<usize>) {
      match self {
         LIPart::Literal(_lcs) => {},
         LIPart::InlineVariable(li) => { vars.push(*li); },
         LIPart::Expression(le) => { le.vars(vars); },
      }
   }
   pub fn equals(&self, other: &LIPart<()>) -> bool {
      match (self,other) {
         (LIPart::Literal(lcs),LIPart::Literal(rcs)) => { *lcs == *rcs },
         (LIPart::InlineVariable(lv),LIPart::InlineVariable(rv)) => { lv == rv },
         (LIPart::Expression(le),LIPart::Expression(re)) => { le.equals(re) },
         _ => false,
      }
   }
   pub fn literal(cs: &str) -> LIPart<S> {
      LIPart::Literal(cs.to_string())
   }
   pub fn variable(vi: usize) -> LIPart<S> {
      LIPart::InlineVariable(vi)
   }
   pub fn expression(ve: Expression<S>) -> LIPart<S> {
      LIPart::Expression(ve)
   }
}

#[derive(Clone)]
pub enum TIPart<S: Debug + Clone> {
   Tuple(Arc<Vec<Value>>),
   Variable(usize),
   InlineVariable(usize),
   Expression(Expression<S>),
}
impl<S: Debug + Clone> TIPart<S> {
   pub fn vars(&self, vars: &mut Vec<usize>) {
      match self {
         TIPart::Tuple(_ts) => {},
         TIPart::Variable(ti) => { vars.push(*ti); },
         TIPart::InlineVariable(ti) => { vars.push(*ti); },
         TIPart::Expression(e) => { e.vars(vars); },
      }
   }
   pub fn equals(&self, other: &TIPart<()>) -> bool {
      match (self,other) {
         (TIPart::InlineVariable(lv),TIPart::InlineVariable(rv)) => { lv == rv },
         (TIPart::Variable(lv),TIPart::Variable(rv)) => { lv == rv },
         (TIPart::Tuple(lts),TIPart::Tuple(rts)) => {
            lts.len() == rts.len() &&
            std::iter::zip(lts.iter(),rts.iter()).all(|(l,r)| l == r)
         },
         (TIPart::Expression(le),TIPart::Expression(re)) => {
            le.equals(re)
         },
         _ => false,
      }
   }
   pub fn tuple(ts: Vec<Value>) -> TIPart<S> {
      TIPart::Tuple(Arc::new(
         ts
      ))
   }
   pub fn expression(e: Expression<S>) -> TIPart<S> {
      TIPart::Expression(
         e
      )
   }
   pub fn variable(vi: usize) -> TIPart<S> {
      TIPart::Variable(vi)
   }
}

pub enum LHSLiteralPart {
   Literal(String),
   Variable(usize),
}
impl LHSLiteralPart {
   pub fn equals(&self, other: &LHSLiteralPart) -> bool {
      match (self,other) {
         (LHSLiteralPart::Literal(lcs),LHSLiteralPart::Literal(rcs)) => { lcs == rcs },
         (LHSLiteralPart::Variable(lcs),LHSLiteralPart::Variable(rcs)) => { lcs == rcs },
         _ => false,
      }
   }
   pub fn literal(cs: &str) -> LHSLiteralPart {
      LHSLiteralPart::Literal(cs.to_string())
   }
   pub fn variable(v: usize) -> LHSLiteralPart {
      LHSLiteralPart::Variable(v)
   }
}

pub enum LHSPart {
   Tuple(Vec<LHSPart>),
   Literal(String),
   UnpackLiteral(Vec<LHSLiteralPart>,Option<usize>,Vec<LHSLiteralPart>),
   Variable(usize),
   Any,
}
impl LHSPart {
   pub fn vars(&self, _vars: &mut Vec<usize>) {
   }
   pub fn equals(&self, other: &LHSPart) -> bool {
      match (self,other) {
         (LHSPart::Any,LHSPart::Any) => true,
         (LHSPart::Variable(lv),LHSPart::Variable(rv)) => { lv==rv },
         (LHSPart::Literal(lv),LHSPart::Literal(rv)) => { lv==rv },
         (LHSPart::Tuple(lv),LHSPart::Tuple(rv)) => {
            lv.len() == rv.len() &&
            std::iter::zip(lv.iter(),rv.iter()).all(|(l,r)| l.equals(r))
         },
         (LHSPart::UnpackLiteral(lpre,lmid,lsuf),LHSPart::UnpackLiteral(rpre,rmid,rsuf)) => {
            lpre.len() == rpre.len() &&
            std::iter::zip(lpre.iter(),rpre.iter()).all(|(l,r)| l.equals(r)) &&
            lmid == rmid &&
            lsuf.len() == rsuf.len() &&
            std::iter::zip(lsuf.iter(),rsuf.iter()).all(|(l,r)| l.equals(r))
         },
         _ => false,
      }
   }
   pub fn ul(pre: Vec<LHSLiteralPart>, mid: Option<usize>, suf: Vec<LHSLiteralPart>) -> LHSPart {
      LHSPart::UnpackLiteral(pre, mid, suf)
   }
   pub fn any() -> LHSPart {
      LHSPart::Any
   }
   pub fn variable(vi: usize) -> LHSPart {
      LHSPart::Variable(vi)
   }
   pub fn tuple(ts: Vec<LHSPart>) -> LHSPart {
      LHSPart::Tuple(ts)
   }
   pub fn literal(cs: &str) -> LHSPart {
      LHSPart::Literal(cs.to_string())
   }
}

#[derive(Clone)]
pub enum Expression<S:Debug + Clone> { //Expressions don't need to "clone"?
   Map(Arc<LHSPart>,Arc<Expression<S>>,Arc<TIPart<S>>,Type,S),
   ValueIntroduction(Value,Type,S),
   LiteralIntroduction(Arc<Vec<LIPart<S>>>,Type,S),
   TupleIntroduction(Arc<Vec<TIPart<S>>>,Type,S),
   VariableReference(usize,Type,S),
   FunctionReference(String,Type,S),
   FunctionApplication(String,Arc<Vec<Expression<S>>>,Type,S),
   PatternMatch(Arc<Expression<S>>,Arc<Vec<(LHSPart,Expression<S>)>>,Type,S),
   Failure(Type,S),
}
impl<S:Debug + Clone> Expression<S> {
   pub fn typed(self, nom: &str) -> Expression<S> {
      let nom = Type::nominal(nom);
      match self {
         Expression::Map(lhs,e,x,_,span) => { Expression::Map(lhs,e,x,nom,span) },
         Expression::ValueIntroduction(vi,_,span) => { Expression::ValueIntroduction(vi,nom,span) },
         Expression::LiteralIntroduction(vi,_,span) => { Expression::LiteralIntroduction(vi,nom,span) },
         Expression::TupleIntroduction(vi,_,span) => { Expression::TupleIntroduction(vi,nom,span) },
         Expression::VariableReference(vi,_,span) => { Expression::VariableReference(vi,nom,span) },
         Expression::FunctionReference(vi,_,span) => { Expression::FunctionReference(vi,nom,span) },
         Expression::FunctionApplication(fi,ps,_,span) => { Expression::FunctionApplication(fi,ps,nom,span) },
         Expression::PatternMatch(pe,lrs,_,span) => { Expression::PatternMatch(pe,lrs,nom,span) },
         Expression::Failure(_,span) => { Expression::Failure(nom,span) },
      }
   }
   pub fn typ(&self) -> Type {
      match self {
         Expression::Map(_lhs,_e,_x,tt,_span) => { tt.clone() },
         Expression::ValueIntroduction(_vi,tt,_span) => { tt.clone() },
         Expression::LiteralIntroduction(_vi,tt,_span) => { tt.clone() },
         Expression::TupleIntroduction(_vi,tt,_span) => { tt.clone() },
         Expression::VariableReference(_vi,tt,_span) => { tt.clone() },
         Expression::FunctionReference(_vi,tt,_span) => { tt.clone() },
         Expression::FunctionApplication(_fi,_ps,tt,_span) => { tt.clone() },
         Expression::PatternMatch(_pe,_lrs,tt,_span) => { tt.clone() },
         Expression::Failure(tt,_span) => { tt.clone() },
      }
   }
   pub fn vars(&self, vars: &mut Vec<usize>) {
      match self {
         Expression::Map(lhs,e,x,_,_) => {            
            e.vars(vars);
            //TODO hide shadowed variables
            //x.vars(vars);
         },
         Expression::VariableReference(vi,_,_) => {
            vars.push(*vi);
         },
         Expression::ValueIntroduction(_,_,_) => {},
         Expression::Failure(_,_) => {},
         Expression::FunctionReference(_,_,_) => {},
         Expression::LiteralIntroduction(lis,_,_) => {
            for li in lis.iter() {
               li.vars(vars);
            }
         },
         Expression::TupleIntroduction(tis,_,_) => {
            for ti in tis.iter() {
               ti.vars(vars);
            }
         },
         Expression::FunctionApplication(_,es,_,_) => {
            for e in es.iter() {
               e.vars(vars);
            }
         },
         Expression::PatternMatch(e,lrs,_,_) => {
            e.vars(vars);
            for (l,r) in lrs.iter() {
               //TODO hide shadowed variables
               r.vars(vars);
            }
         },
      }
   }
   pub fn equals(&self, other: &Expression<()>) -> bool {
      match (self,other) {
         (Expression::ValueIntroduction(lui,_,_),Expression::ValueIntroduction(rui,_,_)) => { lui == rui },
         (Expression::VariableReference(lui,_,_),Expression::VariableReference(rui,_,_)) => { lui == rui },
         (Expression::FunctionReference(lui,_,_),Expression::FunctionReference(rui,_,_)) => { lui == rui },
         (Expression::LiteralIntroduction(lli,_,_),Expression::LiteralIntroduction(rli,_,_)) => {
            lli.len() == rli.len() &&
            std::iter::zip(lli.iter(),rli.iter()).all(|(l,r)| l.equals(r))
         },
         (Expression::TupleIntroduction(lli,_,_),Expression::TupleIntroduction(rli,_,_)) => {
            lli.len() == rli.len() &&
            std::iter::zip(lli.iter(),rli.iter()).all(|(l,r)| l.equals(r))
         },
         (Expression::FunctionApplication(lf,lps,_,_),Expression::FunctionApplication(rf,rps,_,_)) => {
            lf == rf &&
            lps.len() == rps.len() &&
            std::iter::zip(lps.iter(),rps.iter()).all(|(l,r)| l.equals(r))
         },
         (Expression::PatternMatch(le,lps,_,_),Expression::PatternMatch(re,rps,_,_)) => {
            le.equals(re) &&
            lps.len() == rps.len() &&
            std::iter::zip(lps.iter(),rps.iter()).all(|((ll,le),(rl,re))| ll.equals(rl) && le.equals(re))
         },
         (Expression::Failure(_,_),Expression::Failure(_,_)) => { true },
         _ => false
      }
   }
   pub fn map(lhs: LHSPart, e: Expression<S>, x: TIPart<S>, span: S) -> Expression<S> {
      Expression::Map(Arc::new(lhs), Arc::new(e), Arc::new(x), Type::default(), span)
   }
   pub fn unary(ui: &[u8], span: S) -> Expression<S> {
      Expression::ValueIntroduction(Value::unary(ui), Type::default(), span)
   }
   pub fn variable(vi: usize, span: S) -> Expression<S> {
      Expression::VariableReference(vi, Type::default(), span)
   }
   pub fn failure(span: S) -> Expression<S> {
      Expression::Failure(Type::default(), span)
   }
   pub fn literal(cs: &str, span: S) -> Expression<S> {
      Expression::LiteralIntroduction(Arc::new(vec![
         LIPart::Literal(cs.to_string()),
      ]), Type::default(), span)
   }
   pub fn unit(span: S) -> Expression<S> {
      Expression::LiteralIntroduction(Arc::new(vec![
         LIPart::Literal("".to_string()),
      ]), Type::nominal("Unit"), span)
   }
   pub fn li(lps: Vec<LIPart<S>>, span: S) -> Expression<S> {
      Expression::LiteralIntroduction(Arc::new(
         lps
      ), Type::default(), span)
   }
   pub fn tuple(tps: Vec<Value>, span: S) -> Expression<S> {
      Expression::TupleIntroduction(Arc::new(vec![
         TIPart::tuple(tps)
      ]), Type::default(), span)
   }
   pub fn ti(tps: Vec<TIPart<S>>, span: S) -> Expression<S> {
      Expression::TupleIntroduction(Arc::new(
         tps
      ), Type::default(), span)
   }
   pub fn apply(fi: &str, args: Vec<Expression<S>>, span: S) -> Expression<S> {
      Expression::FunctionApplication(fi.to_string(),Arc::new(
         args
      ), Type::default(), span)
   }
   pub fn pattern(v: Expression<S>, lrs: Vec<(LHSPart,Expression<S>)>, span: S) -> Expression<S> {
      Expression::PatternMatch(
         Arc::new(v),
         Arc::new(lrs),
         Type::default(),
         span)
   }
}
