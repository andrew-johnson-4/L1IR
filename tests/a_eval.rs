use l1_ir::ast::{Value,Expression,Program,LIPart,FunctionDefinition,LHSPart};
use l1_ir::eval::{eval};

#[test]
fn eval_empty() {
   assert_eq!(
      eval(Program::<()>::program(vec![],vec![]),&[]).unwrap(),
      Value::tuple(vec![])
   );
}

#[test]
fn eval_failure() {
   assert!(
      eval(Program::program(
         vec![],
         vec![Expression::failure(())]
      ),&[]).is_err()
   );
}

#[test]
fn eval_li() {
   assert_eq!(
      eval(Program::program(
         vec![],
         vec![Expression::literal("",())],
      ),&[]).unwrap(),
      Value::literal("")
   );
   assert_eq!(
      eval(Program::program(
         vec![],
         vec![Expression::li(vec![
            LIPart::literal("a")
         ],())],
      ),&[]).unwrap(),
      Value::literal("a")
   );
   assert_eq!(
      eval(Program::program(
         vec![],
         vec![Expression::li(vec![
            LIPart::literal("a"),
            LIPart::literal("bc"),
         ],())],
      ),&[]).unwrap(),
      Value::literal("abc")
   );
}

#[test]
fn eval_ti() {
   assert_eq!(
      format!("{:?}",eval(Program::program(
         vec![],
         vec![Expression::ti(vec![],())],
      ),&[]).unwrap()),
      "()"
   );
}

#[test]
fn eval_function() {
   assert_eq!(
      format!("{:?}",eval(Program::program(
         vec![FunctionDefinition::define(
            "empty",
            vec![],
            vec![],
         )],
         vec![
            Expression::apply("empty",vec![],()),
         ],
      ),&[]).unwrap()),
      r#"()"#
   );
}

#[test]
fn eval_pattern() {
   assert!(
      eval(Program::program(
         vec![],
         vec![Expression::pattern(
            Expression::li(vec![
               LIPart::literal("b"),
               LIPart::literal("cd"),
            ],()),
            vec![],
         ())],
      ),&[]).is_err(),
   );
   assert_eq!(
      format!("{:?}",eval(Program::program(
         vec![],
         vec![Expression::pattern(
            Expression::li(vec![
               LIPart::literal("b"),
               LIPart::literal("cd"),
            ],()),
            vec![(
               LHSPart::any(),
               Expression::literal("c",())
            )],
         ())],
      ),&[]).unwrap()),
      r#""c""#
   );
   assert_eq!(
      format!("{:?}",eval(Program::program(
         vec![],
         vec![Expression::pattern(
            Expression::li(vec![
               LIPart::literal("b"),
               LIPart::literal("cd"),
            ],()),
            vec![(
               LHSPart::literal("bcd"),
               Expression::literal("e",())
            )],
         ())],
      ),&[]).unwrap()),
      r#""e""#
   );
   assert_eq!(
      format!("{:?}",eval(Program::program(
         vec![],
         vec![Expression::pattern(
            Expression::li(vec![
               LIPart::literal("b"),
               LIPart::literal("cd"),
            ],()),
            vec![(
               LHSPart::variable(123),
               Expression::variable(123,())
            )],
         ())],
      ),&[]).unwrap()),
      r#""bcd""#
   );
}
