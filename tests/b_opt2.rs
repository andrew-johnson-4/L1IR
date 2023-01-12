use l1_ir::ast::{Expression,Program,FunctionDefinition,LHSPart,TIPart};
use l1_ir::eval::{eval};
use l1_ir::opt::{JProgram};

#[test]
fn eval_tuple1() {
   for x in 0..20 {
   for y in 0..20 {
      let nojit = Program::program(
         vec![FunctionDefinition::define(
            vec![0,1],
            vec![Expression::pattern(
               Expression::ti(vec![
                  TIPart::variable(0),
                  TIPart::expression(Expression::variable(1,())),
               ],()),
               vec![(
                  LHSPart::Any,
                  Expression::unary(b"0",())
               )],
            ())],
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
      let jval = jit.eval(&[]).unwrap();
      assert_eq!(nval, jval, "{} * {}", x, y);
   }}
}
