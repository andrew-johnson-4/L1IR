use l1_ir::value::{Value};

#[test]
fn value_nil() {
   let v = Value::unit("T#0");
   assert_eq!(v.tag(), "Unit");
   assert_eq!(v.name(), "T#0");
}

#[test]
fn value_number_i8() {
   let v = Value::i8(-2, "T#1");
   assert_eq!(v.tag(), "I8");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(0), -2);
}

#[test]
fn value_number_u8() {
   let v = Value::u8(12, "T#3");
   assert_eq!(v.tag(), "U8");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(0), 12);
}
