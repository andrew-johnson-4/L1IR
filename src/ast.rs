use num_bigint::{BigUint,ToBigUint};
use regex::Regex;
use std::rc::Rc;
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
   pub regex: Option<Rc<Regex>>,
   pub strct: Option<Vec<Type>>,
   pub fnid: Option<usize>,
   pub invariants: Vec<usize>,
}
impl std::fmt::Debug for Type {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      if let Some(n) = &self.name {
         write!(f, "{}", n)?
      }
      write!(f, "")
   }
}
impl Type {
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
         regex: Some(Rc::new(Regex::new(r).unwrap())),
         strct: None,
         fnid: None,
         invariants: vec![],
      }
   }
   pub fn function(f: usize) -> Type {
      Type {
         name: None,
         regex: None,
         strct: None,
         fnid: Some(f),
         invariants: vec![],
      }
   }
   pub fn reject(msg: &str) -> Error<()> {
      Error {
         error_type: "Dynamic Type Error".to_string(),
         error_msg: msg.to_string(),
         span: ()
      }
   }
   pub fn accepts(v: &Value, constraint: &Type) -> Result<(),Error<()>> {
      if let Some(ref _cr) = constraint.regex {
         unimplemented!("Check literal constraint")
      }
      if let Some(ref _cstrct) = constraint.strct {
         unimplemented!("Check structural constraint")
      }
      if let Some(cfi) = constraint.fnid {
         if let Value::Function(vfi,_) = v {
         if cfi != *vfi { return Err(Type::reject(
            &format!("Function #{} does not satisfy constraint: {:?}", vfi, constraint)
         )); }} else { return Err(Type::reject(
            &format!("Value {:?} is not a function", v)
         )); }
      }
      if let Some(ref cnom) = constraint.name {
         if let Some(ref vnom) = v.typof() {
            if cnom != vnom { return Err(Type::reject(
               &format!("Type {:?} does not satisfy constraint: {:?}", vnom, constraint)
            )); }
         } else { return Err(Type::reject(
            &format!("Type ? does not satisfy constraint: {:?}", constraint)
         )); }
      }
      Ok(())
   }
}

#[derive(Clone)]
pub enum Value {
   Unary(BigUint,Option<String>), //a unary number, represented as "0"...
   Literal(usize,usize,Rc<Vec<char>>,Option<String>), //avoid copy-on-slice
   Tuple(usize,usize,Rc<Vec<Value>>,Option<String>), //avoid copy-on-slice
   Function(usize,Option<String>), //all functions are static program indices
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
   pub fn unary(buf: &[u8]) -> Value {
      let ui = BigUint::parse_bytes(buf, 10).expect("unary parse_bytes failed");
      Value::Unary(ui,None)
   }
   pub fn literal(cs: &str) -> Value {
      let cs = cs.chars().collect::<Vec<char>>();
      Value::Literal(0,cs.len(),Rc::new(cs),None)
   }
   pub fn tuple(ts: Vec<Value>) -> Value {
      Value::Tuple(0,ts.len(),Rc::new(ts),None)
   }
   pub fn function(fid: usize) -> Value {
      Value::Function(fid,None)
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
         write!(f, "f#{}", fid)
      } else if let Value::Unary(ui,_tt) = self {
         write!(f, "{}", ui)
      } else { unreachable!("exhaustive") }
   }
}

#[derive(Clone)]
pub struct FunctionDefinition<S:Debug + Clone> {
   pub args: Vec<usize>,
   pub body: Vec<Expression<S>>,
   pub entry_points: Vec<()>,
}
impl<S:Debug + Clone> FunctionDefinition<S> {
   pub fn define(args: Vec<usize>, body: Vec<Expression<S>>) -> FunctionDefinition<S> {
      FunctionDefinition {
         args: args,
         body: body,
         entry_points: Vec::new(),
      }
   }
}

#[derive(Clone)]
pub struct Program<S:Debug + Clone> {
   pub functions: Vec<FunctionDefinition<S>>,
   pub expressions: Vec<Expression<S>>,
}
impl<S:Debug + Clone> Program<S> {
   pub fn program(functions: Vec<FunctionDefinition<S>>, expressions: Vec<Expression<S>>) -> Program<S> {
      Program {
         functions: functions,
         expressions: expressions,
      }
   }
}

#[derive(Clone)]
pub enum LIPart<S:Debug + Clone> {
   Literal(Rc<Vec<char>>),
   InlineVariable(usize),
   Expression(Expression<S>),
}
impl<S:Debug + Clone> LIPart<S> {
   pub fn literal(cs: &str) -> LIPart<S> {
      let cs = cs.chars().collect::<Vec<char>>();
      LIPart::Literal(Rc::new(
         cs
      ))
   }
   pub fn variable(vi: usize) -> LIPart<S> {
      LIPart::InlineVariable(vi)
   }
   pub fn expression(ve: Expression<S>) -> LIPart<S> {
      LIPart::Expression(ve)
   }
}

#[derive(Clone)]
pub enum TIPart {
   Tuple(Rc<Vec<Value>>),
   Variable(usize),
   InlineVariable(usize),
}
impl TIPart {
   pub fn tuple(ts: Vec<Value>) -> TIPart {
      TIPart::Tuple(Rc::new(
         ts
      ))
   }
   pub fn variable(vi: usize) -> TIPart {
      TIPart::Variable(vi)
   }
}

pub enum LHSLiteralPart {
   Literal(Vec<char>),   
}
impl LHSLiteralPart {
   pub fn literal(cs: &str) -> LHSLiteralPart {
      let cs = cs.chars().collect::<Vec<char>>();
      LHSLiteralPart::Literal(cs)
   }
}

pub enum LHSPart {
   Tuple(Vec<LHSPart>),
   Literal(Vec<char>),
   UnpackLiteral(Vec<LHSLiteralPart>,Option<usize>,Vec<LHSLiteralPart>),
   Variable(usize),
   Any,
}
impl LHSPart {
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
      let cs = cs.chars().collect::<Vec<char>>();
      LHSPart::Literal(cs)
   }
}

#[derive(Clone)]
pub enum Expression<S:Debug + Clone> { //Expressions don't need to "clone"?
   UnaryIntroduction(BigUint,S),
   LiteralIntroduction(Rc<Vec<LIPart<S>>>,S),
   TupleIntroduction(Rc<Vec<TIPart>>,S),
   VariableReference(usize,S),
   FunctionReference(usize,S),
   FunctionApplication(usize,Rc<Vec<Expression<S>>>,S),
   PatternMatch(Rc<Expression<S>>,Rc<Vec<(LHSPart,Expression<S>)>>,S),
   Failure(S),
}
impl<S:Debug + Clone> Expression<S> {
   pub fn unary(ui: &[u8], span: S) -> Expression<S> {
      let ui = BigUint::parse_bytes(ui, 10).unwrap();
      Expression::UnaryIntroduction(ui, span)
   }
   pub fn variable(vi: usize, span: S) -> Expression<S> {
      Expression::VariableReference(vi,span)
   }
   pub fn failure(span: S) -> Expression<S> {
      Expression::Failure(span)
   }
   pub fn literal(cs: &str, span: S) -> Expression<S> {
      let cs = cs.chars().collect::<Vec<char>>();
      Expression::LiteralIntroduction(Rc::new(vec![
         LIPart::Literal(Rc::new(cs)),
      ]), span)
   }
   pub fn li(lps: Vec<LIPart<S>>, span: S) -> Expression<S> {
      Expression::LiteralIntroduction(Rc::new(
         lps
      ), span)
   }
   pub  fn tuple(tps: Vec<Value>, span: S) -> Expression<S> {
      Expression::TupleIntroduction(Rc::new(vec![
         TIPart::tuple(tps)
      ]), span)
   }
   pub fn ti(tps: Vec<TIPart>, span: S) -> Expression<S> {
      Expression::TupleIntroduction(Rc::new(
         tps
      ), span)
   }
   pub fn apply(fi: usize, args: Vec<Expression<S>>, span: S) -> Expression<S> {
      Expression::FunctionApplication(fi,Rc::new(
         args
      ), span)
   }
   pub fn pattern(v: Expression<S>, lrs: Vec<(LHSPart,Expression<S>)>, span: S) -> Expression<S> {
      Expression::PatternMatch(
         Rc::new(v),
         Rc::new(lrs),
         span)
   }
}
