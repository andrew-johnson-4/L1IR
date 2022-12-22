use std::rc::Rc;

pub enum Value {
   Literal(usize,usize,Rc<Vec<char>>), //avoid copy-on-slice
   Tuple(Vec<Value>),
   Function(usize), //all functions are static program indices
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
