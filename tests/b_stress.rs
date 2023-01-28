use l1_ir::ast::{Expression,Program,FunctionDefinition,LHSPart,LHSLiteralPart,LIPart,Type};
use l1_ir::eval::{eval};

#[test]
fn eval_recursive_loop() {
   assert_eq!(
      format!("{:?}",eval(Program::program(
         vec![FunctionDefinition::define(
            "loop",
            vec![(24,Type::nominal("U64"))],
            vec![Expression::pattern(
               Expression::variable(24,()),
               vec![
                  (
                     LHSPart::ul(
                        vec![LHSLiteralPart::literal("0")],
                        Some(2),
                        vec![],
                     ),
                     Expression::apply("loop",vec![
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
            Expression::apply("loop",vec![
               Expression::unary(b"999999", ())
            ],()),
         ],
      ),&[]).unwrap()),
      r#""ok""#
   );
}


#[test]
fn eval_fibonacci() {
   assert_eq!(
      format!("{:?}",eval(Program::program(
         vec![FunctionDefinition::define(
            "fib",
            vec![(24,Type::nominal("U64"))],
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
                           Expression::apply("fib",vec![
                              Expression::variable(2,()),
                           ],()),
                        ),
                        LIPart::expression(
                           Expression::apply("fib",vec![
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
            Expression::apply("fib",vec![
               Expression::unary(b"25", ())
            ],()),
         ],
      ),&[]).unwrap()),
      r#"75025"#
   );
}

