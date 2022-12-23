use std::rc::Rc;
use l1_ir::ast::{Value,Expression,Program,LIPart};
use l1_ir::eval::{eval};

fn by_expressions(es: Vec<Expression>) -> Program {
   Program {
      functions: vec![],
      expressions: es,
   }
}
fn by_expression(e: Expression) -> Program {
   by_expressions(vec![ e ])
}

#[test]
fn eval_empty() {
   assert_eq!(
      eval(by_expressions(vec![])),
      Value::tuple(vec![])
   );
}

#[test]
fn eval_li() {
   assert_eq!(
      eval(by_expression(
         Expression::LiteralIntroduction(vec![])
      )),
      Value::literal("")
   );
   assert_eq!(
      eval(by_expression(
         Expression::LiteralIntroduction(vec![
            LIPart::Linear(Rc::new(vec!['a']))
         ])
      )),
      Value::literal("a")
   );
}
