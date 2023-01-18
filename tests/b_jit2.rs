use l1_ir::ast::{Expression,Program,FunctionDefinition,LIPart,Value,LHSPart,LHSLiteralPart};
use l1_ir::opt::{JProgram};

#[test]
fn eval_tuple_introduction() {
   let nojit = Program::program(
      vec![],
      vec![],
   );
   let jit = JProgram::compile(&nojit);

   for x in 0..20 {
      let jval = jit.eval(&[x]).unwrap();
      assert_eq!(format!("{:?}",jval), r#"(1,"True",(2.00000,(3,4,5))"#);
   }
}
