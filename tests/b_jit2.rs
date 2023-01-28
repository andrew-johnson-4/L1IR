use l1_ir::value::Value;
use l1_ir::ast::{Expression,Program};
use l1_ir::opt::{JProgram};

#[test]
fn eval_string_introduction() {
   let nojit = Program::program(
      vec![],
      vec![
         Expression::variable(0,()),
      ],
   );
   let jit = JProgram::compile(&nojit);

   let arg = Value::string("abc", "String");
   let ret = jit.eval(&[arg]);
   assert_eq!(format!("{:?}",ret), r#""abc""#);
}
