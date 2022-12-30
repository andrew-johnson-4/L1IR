use l1_ir::ast::{Value,Type};

#[test]
fn accept_nominal() {
   assert!(Type::accepts(
      &Value::literal("a").typed("A"),
      &Type::nominal("A"),
   ).is_ok());
   assert!(Type::accepts(
      &Value::literal("a").typed("A<B>"),
      &Type::nominal("A<B>"),
   ).is_ok());
   assert!(Type::accepts(
      &Value::literal("a").typed("A<B,C>"),
      &Type::nominal("A<B,C>"),
   ).is_ok());
}

#[test]
fn reject_nominal() {
   assert!(Type::accepts(
      &Value::literal("a").typed("A"),
      &Type::nominal("B"),
   ).is_err());
   assert!(Type::accepts(
      &Value::literal("a").typed("A<B>"),
      &Type::nominal("B<A>"),
   ).is_err());
   assert!(Type::accepts(
      &Value::literal("a").typed("A<B>"),
      &Type::nominal("A<C>"),
   ).is_err());
   assert!(Type::accepts(
      &Value::literal("a").typed("A<B,C>"),
      &Type::nominal("A<C,B>"),
   ).is_err());
}

#[test]
fn accept_function() {
   assert!(Type::accepts(
      &Value::function(0),
      &Type::function(0),
   ).is_ok());
   assert!(Type::accepts(
      &Value::function(1),
      &Type::function(1),
   ).is_ok());
   assert!(Type::accepts(
      &Value::function(123),
      &Type::function(123),
   ).is_ok());
}

#[test]
fn reject_function() {
   assert!(Type::accepts(
      &Value::function(0),
      &Type::function(1),
   ).is_err());
   assert!(Type::accepts(
      &Value::function(1),
      &Type::function(0),
   ).is_err());
   assert!(Type::accepts(
      &Value::function(123),
      &Type::function(321),
   ).is_err());
}

/* TODO gradual DFAs
#[test]
fn accept_regex() {
   assert!(Type::accepts(
      &Value::literal("a"),
      &Type::regex("^[ab]$"),
   ).is_ok());
   assert!(Type::accepts(
      &Value::literal("b"),
      &Type::regex("^[ab]$"),
   ).is_ok());
}
*/
