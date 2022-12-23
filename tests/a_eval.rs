use std::rc::Rc;
use l1_ir::ast::{Value,Expression,Program,LIPart,TIPart,FunctionDefinition};
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
   assert_eq!(
      eval(by_expression(
         Expression::LiteralIntroduction(vec![
            LIPart::Linear(Rc::new(vec!['a'])),
            LIPart::Linear(Rc::new(vec!['b','c']))
         ])
      )),
      Value::literal("abc")
   );
}

#[test]
fn eval_ti() {
   assert_eq!(
      format!("{:?}",eval(by_expression(
         Expression::TupleIntroduction(vec![])
      ))),
      "()"
   );
   assert_eq!(
      format!("{:?}",eval(by_expression(
         Expression::TupleIntroduction(vec![
            TIPart::Linear(Rc::new(vec![
               Value::literal("a"),
            ]))
         ])
      ))),
      r#"("a",)"#
   );
   assert_eq!(
      format!("{:?}",eval(by_expression(
         Expression::TupleIntroduction(vec![
            TIPart::Linear(Rc::new(vec![
               Value::literal("a"),
               Value::literal("bc"),
            ]))
         ])
      ))),
      r#"("a","bc")"#
   );
}

#[test]
fn eval_function() {
   assert_eq!(
      format!("{:?}",eval(Program {
         functions: vec![FunctionDefinition {
            args: vec![],
            body: Expression::TupleIntroduction(vec![
               TIPart::Linear(Rc::new(vec![
                  Value::literal("a"),
                  Value::literal("bc"),
               ]))
            ])
         }],
         expressions: vec![],
      })),
      "()"
   );
   assert_eq!(
      format!("{:?}",eval(Program {
         functions: vec![FunctionDefinition {
            args: vec![],
            body: Expression::TupleIntroduction(vec![
               TIPart::Linear(Rc::new(vec![
                  Value::literal("a"),
                  Value::literal("bc"),
               ]))
            ])
         }],
         expressions: vec![
            Expression::FunctionApplication(0,vec![]),
         ],
      })),
      r#"("a","bc")"#
   );
   assert_eq!(
      format!("{:?}",eval(Program {
         functions: vec![FunctionDefinition {
            args: vec![24],
            body: Expression::TupleIntroduction(vec![
               TIPart::Linear(Rc::new(vec![
                  Value::literal("a"),
               ])),
               TIPart::Variable(24),
            ])
         }],
         expressions: vec![
            Expression::FunctionApplication(0,vec![
               Expression::LiteralIntroduction(vec![
                  LIPart::Linear(Rc::new(vec!['b'])),
                  LIPart::Linear(Rc::new(vec!['c','d']))
               ])
            ]),
         ],
      })),
      r#"("a","bcd")"#
   );
}
