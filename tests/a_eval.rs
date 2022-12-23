use std::rc::Rc;
use l1_ir::ast::{Value,Expression,Program,LIPart,TIPart,FunctionDefinition};
use l1_ir::eval::{eval};

fn by_expressions(es: Vec<Expression<()>>) -> Program<()> {
   Program {
      functions: vec![],
      expressions: es,
   }
}
fn by_expression(e: Expression<()>) -> Program<()> {
   by_expressions(vec![ e ])
}

#[test]
fn eval_empty() {
   assert_eq!(
      eval(by_expressions(vec![])).unwrap(),
      Value::tuple(vec![])
   );
}

#[test]
fn eval_failure() {
   assert!(
      eval(by_expression(Expression::Failure(()))).is_err()
   );
}

#[test]
fn eval_li() {
   assert_eq!(
      eval(by_expression(
         Expression::LiteralIntroduction(Rc::new(vec![]),())
      )).unwrap(),
      Value::literal("")
   );
   assert_eq!(
      eval(by_expression(
         Expression::LiteralIntroduction(Rc::new(vec![
            LIPart::Linear(Rc::new(vec!['a']))
         ]),())
      )).unwrap(),
      Value::literal("a")
   );
   assert_eq!(
      eval(by_expression(
         Expression::LiteralIntroduction(Rc::new(vec![
            LIPart::Linear(Rc::new(vec!['a'])),
            LIPart::Linear(Rc::new(vec!['b','c']))
         ]),())
      )).unwrap(),
      Value::literal("abc")
   );
}

#[test]
fn eval_ti() {
   assert_eq!(
      format!("{:?}",eval(by_expression(
         Expression::TupleIntroduction(Rc::new(vec![]),())
      )).unwrap()),
      "()"
   );
   assert_eq!(
      format!("{:?}",eval(by_expression(
         Expression::TupleIntroduction(Rc::new(vec![
            TIPart::Linear(Rc::new(vec![
               Value::literal("a"),
            ]))
         ]),())
      )).unwrap()),
      r#"("a",)"#
   );
   assert_eq!(
      format!("{:?}",eval(by_expression(
         Expression::TupleIntroduction(Rc::new(vec![
            TIPart::Linear(Rc::new(vec![
               Value::literal("a"),
               Value::literal("bc"),
            ]))
         ]),())
      )).unwrap()),
      r#"("a","bc")"#
   );
}

#[test]
fn eval_function() {
   assert_eq!(
      format!("{:?}",eval(Program {
         functions: vec![FunctionDefinition {
            args: vec![],
            body: vec![Expression::TupleIntroduction(Rc::new(vec![
               TIPart::Linear(Rc::new(vec![
                  Value::literal("a"),
                  Value::literal("bc"),
               ]))
            ]),())]
         }],
         expressions: vec![],
      }).unwrap()),
      "()"
   );
   assert_eq!(
      format!("{:?}",eval(Program {
         functions: vec![FunctionDefinition {
            args: vec![],
            body: vec![],
         }],
         expressions: vec![
            Expression::FunctionApplication(0,Rc::new(vec![]),()),
         ],
      }).unwrap()),
      r#"()"#
   );
   assert_eq!(
      format!("{:?}",eval(Program {
         functions: vec![FunctionDefinition {
            args: vec![],
            body: vec![Expression::TupleIntroduction(Rc::new(vec![
               TIPart::Linear(Rc::new(vec![
                  Value::literal("a"),
                  Value::literal("bc"),
               ]))
            ]),())]
         }],
         expressions: vec![
            Expression::FunctionApplication(0,Rc::new(vec![]),()),
         ],
      }).unwrap()),
      r#"("a","bc")"#
   );
   assert_eq!(
      format!("{:?}",eval(Program {
         functions: vec![FunctionDefinition {
            args: vec![24],
            body: vec![Expression::TupleIntroduction(Rc::new(vec![
               TIPart::Linear(Rc::new(vec![
                  Value::literal("a"),
               ])),
               TIPart::Variable(24),
            ]),())]
         }],
         expressions: vec![
            Expression::FunctionApplication(0,Rc::new(vec![
               Expression::LiteralIntroduction(Rc::new(vec![
                  LIPart::Linear(Rc::new(vec!['b'])),
                  LIPart::Linear(Rc::new(vec!['c','d']))
               ]),())
            ]),()),
         ],
      }).unwrap()),
      r#"("a","bcd")"#
   );
}
