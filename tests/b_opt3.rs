use l1_ir::ast::{Expression,Program,FunctionDefinition,LIPart};
use l1_ir::opt::{JProgram};

#[test]
fn eval_string1() {
   let nojit = Program::program(
      vec![FunctionDefinition::define(
         "abc",
         vec![],
         vec![Expression::li(vec![
            LIPart::literal("abc"),
         ],()).typed("String"),],
      )],
      vec![
         Expression::apply("abc",vec![],()),
      ],
   );
   let jit = JProgram::compile(&nojit);
   let jval = jit.eval(&[]);
   assert_eq!(format!(r#""abc""#), format!("{:?}",jval));
}
