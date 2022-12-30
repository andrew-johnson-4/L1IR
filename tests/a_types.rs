use l1_ir::ast::{Value,Type};

#[test]
fn accept_nominal() {
   assert!(Type::accepts(
      Value::typed(
         Value::literal("a"),
         Type::nominal("A",vec![])
      ),
      Type::nominal("A",vec![]),
   ).is_ok())
}

#[test]
fn reject_nominal() {
   assert!(Type::accepts(
      Value::typed(
         Value::literal("a"),
         Type::nominal("A",vec![])
      ),
      Type::nominal("B",vec![]),
   ).is_err())
}

