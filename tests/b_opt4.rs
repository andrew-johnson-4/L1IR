use l1_ir::ast::{Expression,Program};
use l1_ir::opt::{JProgram};

#[test]
fn eval_string() {
   let nojit = Program::program(
      vec![],
      vec![
         Expression::literal(r#""abc""#, ()).typed("String"),
      ],
   );
   let jit = JProgram::compile(&nojit);
   let jval = jit.eval(&[]);
   assert_eq!(r#""abc""#, format!("{:?}",jval), "abc");
}
