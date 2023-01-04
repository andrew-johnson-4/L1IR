use l1_ir::ast::{Expression,Program,FunctionDefinition,LIPart,Value};
use l1_ir::opt::{JProgram};

#[test]
fn eval_add() {
   let nojit = Program::program(
      vec![FunctionDefinition::define(
         vec![0,1],
         vec![Expression::li(vec![
            LIPart::variable(0),
            LIPart::variable(1),
         ],())]
      )],
      vec![
         Expression::apply(0,vec![
            Expression::variable(0, ()),
            Expression::variable(1, ()),
         ],()),
      ],
   );
   let jit = JProgram::compile(&nojit);

   for x in 0..20 {
   for y in 0..20 {
      let jval = jit.eval(&[x, y]).unwrap();
      assert_eq!(Value::from_u64(x + y), jval, "{} + {}", x, y);
   }}
}
