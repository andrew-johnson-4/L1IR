use l1_ir::ast::{Expression,Program,FunctionDefinition,LIPart};
use l1_ir::eval::{eval};
use l1_ir::opt::{JProgram};

#[test]
fn eval_add() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![FunctionDefinition::define(
            vec![24,27],
            vec![Expression::li(vec![
               LIPart::variable(24),
               LIPart::variable(27),
            ],())]
         )],
         vec![
            Expression::apply(0,vec![
               Expression::unary(format!("{}",x).as_bytes(), ()),
               Expression::unary(format!("{}",y).as_bytes(), ()),
            ],()),
         ],
      );
      let jit = JProgram::compile(&nojit);
      let nval = eval(nojit).unwrap();
      let jval = jit.eval().unwrap();
      assert_eq!(nval, jval);
   }}
}
