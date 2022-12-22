use std::rc::Rc;

#[derive(Clone)]
pub enum Value {
   Literal(usize,usize,Rc<Vec<char>>), //avoid copy-on-slice
   Tuple(Vec<Value>),
   Function(usize), //all functions are static program indices
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
         (Value::Tuple(ls),Value::Tuple(rs)) if ls.len()==rs.len() => {
            for i in 0..ls.len() {
            if ls[i] != rs[i] {
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
      } else if let Value::Tuple(ts) = self {
         write!(f, r"(")?;
         for i in 0..ts.len() {
            if i>0 {
               write!(f, r",")?;
            }
            ts[i].fmt(f)?;
         }
         if ts.len()==1 {
            write!(f, r",")?;
         }
         write!(f, r")")
      } else if let Value::Function(fid) = self {
         write!(f, "f#{}", fid)
      } else { unreachable!("exhaustive") }
   }
}

pub struct FunctionDefinition {
}

pub struct Program {
   pub functions: Vec<FunctionDefinition>,
   pub expressions: Vec<Expression>,
}

pub enum Expression {
   LiteralIntroduction,
   TupleIntroduction,
   VariableReference(usize),
   FunctionApplication(Box<Expression>,Vec<Expression>),
   PatternMatch,
   Failure,
}
