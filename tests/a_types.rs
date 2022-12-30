use l1_ir::ast::{Value,Type};

#[test]
fn accept_nominal() {
   assert!(Type::accepts(
      &Value::literal("a").typed(
         Type::nominal("A")
      ),
      &Type::nominal("A"),
   ).is_ok());
   assert!(Type::accepts(
      &Value::literal("a").typed(
         Type::nominal("A<B>")
      ),
      &Type::nominal("A<B>"),
   ).is_ok());
   assert!(Type::accepts(
      &Value::literal("a").typed(
         Type::nominal("A<B,C>")
      ),
      &Type::nominal("A<B,C>"),
   ).is_ok());
}

#[test]
fn reject_nominal() {
   assert!(Type::accepts(
      &Value::literal("a").typed(
         Type::nominal("A")
      ),
      &Type::nominal("B"),
   ).is_err());
   assert!(Type::accepts(
      &Value::literal("a").typed(
         Type::nominal("A<B>")
      ),
      &Type::nominal("B<A>"),
   ).is_err());
   assert!(Type::accepts(
      &Value::literal("a").typed(
         Type::nominal("A<B>")
      ),
      &Type::nominal("A<C>"),
   ).is_err());
   assert!(Type::accepts(
      &Value::literal("a").typed(
         Type::nominal("A<B,C>")
      ),
      &Type::nominal("A<C,B>"),
   ).is_err());
}

