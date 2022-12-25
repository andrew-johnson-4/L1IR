use l1_ir::ast::{Expression,Program,FunctionDefinition,LHSPart,LHSLiteralPart,LIPart};
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


#[test]
fn eval_fibonacci() {
   assert_eq!(
      format!("{:?}",eval(Program::program(
         vec![FunctionDefinition::define(
            vec![24],
            vec![Expression::pattern(
               Expression::variable(24,()),
               vec![
                  (
                     LHSPart::literal(""),
                     Expression::unary(b"0",()),
                  ),
                  (
                     LHSPart::literal("0"),
                     Expression::unary(b"1",()),
                  ),
                  (
                     LHSPart::ul(
                        vec![LHSLiteralPart::literal("00")],
                        Some(2),
                        vec![],
                     ),
                     Expression::li(vec![
                        LIPart::expression(
                           Expression::apply(0,vec![
                              Expression::variable(2,()),
                           ],()),
                        ),
                        LIPart::expression(
                           Expression::apply(0,vec![
                              Expression::li(vec![
                                 LIPart::literal("0"),
                                 LIPart::variable(2),
                              ],()),
                           ],()),
                        ),
                     ],()),
                  ),
               ],
            ())],
         )],
         vec![
            Expression::apply(0,vec![
               Expression::unary(b"25", ())
            ],()),
         ],
      )).unwrap()),
      r#"75025"#
   );
}

