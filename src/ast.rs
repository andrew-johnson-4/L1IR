use std::rc::Rc;
use std::fmt::Debug;

pub struct Error<S:Debug + Clone> {
   error_type: String,
   error_msg: String,
   span: S,
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
pub enum Value {
   Literal(usize,usize,Rc<Vec<char>>), //avoid copy-on-slice
   Tuple(usize,usize,Rc<Vec<Value>>), //avoid copy-on-slice
   Function(usize), //all functions are static program indices
}
impl Value {
   pub fn literal(cs: &str) -> Value {
      let cs = cs.chars().collect::<Vec<char>>();
      Value::Literal(0,cs.len(),Rc::new(cs))
   }
   pub fn tuple(ts: Vec<Value>) -> Value {
      Value::Tuple(0,ts.len(),Rc::new(ts))
   }
}
impl PartialEq for Value {
   fn eq(&self, other: &Self) -> bool {
      match (self, other) {
         (Value::Literal(ls,le,lv),Value::Literal(rs,re,rv)) if (le-ls)==(re-rs) => {
            for i in 0..(le-ls) {
            if lv[ls+i] != rv[rs+i] {
               return false;
            }}
            true
         },
         (Value::Tuple(ls,le,lv),Value::Tuple(rs,re,rv)) if (le-ls)==(re-rs) => {
            for i in 0..(le-ls) {
            if lv[ls+i] != rv[rs+i] {
               return false;
            }}
            true
         },
         (Value::Function(lf),Value::Function(rf)) => {
            lf == rf
         },
         _ => false,
      }
   }
}
impl Eq for Value {}
impl std::fmt::Debug for Value {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      if let Value::Literal(start,end,val) = self {
         write!(f, r#"""#)?;
         for i in (*start)..(*end) {
            write!(f, "{}", val[i])?;
         }
         write!(f, r#"""#)
      } else if let Value::Tuple(start,end,val) = self {
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
      } else if let Value::Function(fid) = self {
         write!(f, "f#{}", fid)
      } else { unreachable!("exhaustive") }
   }
}

pub struct FunctionDefinition<S:Debug + Clone> {
   pub args: Vec<usize>,
   pub body: Vec<Expression<S>>,
}

pub struct Program<S:Debug + Clone> {
   pub functions: Vec<FunctionDefinition<S>>,
   pub expressions: Vec<Expression<S>>,
}

#[derive(Clone)]
pub enum LIPart {
   Linear(Rc<Vec<char>>),
   InlineVariable(usize),
}
#[derive(Clone)]
pub enum TIPart {
   Linear(Rc<Vec<Value>>),
   Variable(usize),
   InlineVariable(usize),
}
pub enum LHSPart {
   Tuple(Vec<LHSPart>),
   Literal(Vec<char>),
   Variable(usize),
   Any,
}
#[derive(Clone)]
pub enum Expression<S:Debug + Clone> { //Expressions don't need to "clone"?
   LiteralIntroduction(Rc<Vec<LIPart>>,S),
   TupleIntroduction(Rc<Vec<TIPart>>,S),
   VariableReference(usize,S),
   FunctionReference(usize,S),
   FunctionApplication(usize,Rc<Vec<Expression<S>>>,S),
   PatternMatch(Rc<Expression<S>>,Rc<Vec<(LHSPart,Expression<S>)>>,S),
   Failure(S),
}
