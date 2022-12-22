use std::rc::Rc;
use l1_ir::ast::{Value};

#[test]
fn eval_literals() {
   assert_eq!(
      format!("{:?}",Value::Literal(0,1,Rc::new(vec!['a']))),
      r#""a""#,
   );
}
