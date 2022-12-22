use l1_ir::ast::{Value,Program};
use l1_ir::eval::{eval};

#[test]
fn eval_empty() {
   assert_eq!(
      eval(Program {
         functions: vec![],
         expressions: vec![],
      }),
      Value::Tuple(vec![])
   );
}
