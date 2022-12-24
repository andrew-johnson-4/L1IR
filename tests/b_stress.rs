use std::rc::Rc;
use num_bigint::{BigUint};
use l1_ir::ast::{Expression,Program,FunctionDefinition,LHSPart,LIPart,LHSLiteralPart};
use l1_ir::eval::{eval};

#[test]
fn eval_recursive_loop() {
   assert_eq!(
      format!("{:?}",eval(Program {
         functions: vec![FunctionDefinition {
            args: vec![24],
            body: vec![Expression::PatternMatch(
               Rc::new(Expression::VariableReference(24,())),
               Rc::new(vec![
                  (
                     LHSPart::UnpackLiteral(
                        vec![LHSLiteralPart::Literal(vec!['0'])],
                        Some(2),
                        vec![],
                     ),
                     Expression::FunctionApplication(0,Rc::new(vec![
                        Expression::VariableReference(2,()),
                     ]),()),
                  ),
                  (
                     LHSPart::Literal(vec![]),
                     Expression::LiteralIntroduction(Rc::new(vec![
                        LIPart::Linear(Rc::new(vec!['o','k'])),
                     ]),()),
                  ),
               ]),
               ()
            )],
         }],
         expressions: vec![
            Expression::FunctionApplication(0,Rc::new(vec![
               Expression::UnaryIntroduction(
                  BigUint::parse_bytes(b"99999999999", 10).expect("unary parse_bytes failed")
               ,()),
            ]),()),
         ],
      }).unwrap()),
      r#""ok""#
   );
}

