use l1_ir::ast::{Expression,Program,FunctionDefinition,LHSPart,LHSLiteralPart};
use l1_ir::eval::{eval};

#[test]
fn eval_recursive_loop() {
   assert_eq!(
      format!("{:?}",eval(Program::program(
         vec![FunctionDefinition::define(
            vec![24],
            vec![Expression::pattern(
               Expression::variable(24,()),
               vec![
                  (
                     LHSPart::ul(
                        vec![LHSLiteralPart::literal("0")],
                        Some(2),
                        vec![],
                     ),
                     Expression::apply(0,vec![
                        Expression::variable(2,()),
                     ],()),
                  ),
                  (
                     LHSPart::literal(""),
                     Expression::literal("ok",()),
                  ),
               ],
            ())],
         )],
         vec![
            Expression::apply(0,vec![
               Expression::unary(b"999999", ())
            ],()),
         ],
      )).unwrap()),
      r#""ok""#
   );
}

