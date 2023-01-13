use l1_ir::value::{Value,Tag};

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
   assert_eq!(v.slot(Tag::I8, 0), -2);

   let v = Value::i8(100, "T#3");
   assert_eq!(v.tag(), "I8");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::I8, 0), 100);
}

#[test]
fn value_number_u8() {
   let v = Value::u8(12, "T#3");
   assert_eq!(v.tag(), "U8");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::U8, 0), 12);

   let v = Value::u8(9, "T#4");
   assert_eq!(v.tag(), "U8");
   assert_eq!(v.name(), "T#4");
   assert_eq!(v.slot(Tag::U8, 0), 9);
}

#[test]
fn value_numbers_i8() {
   let v = Value::i8s(&vec![3,-2,4,7], "T#1");
   assert_eq!(v.tag(), "I84");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::I8, 0), 3);
   assert_eq!(v.slot(Tag::I8, 1), -2);
   assert_eq!(v.slot(Tag::I8, 2), 4);
   assert_eq!(v.slot(Tag::I8, 3), 7);
}

#[test]
fn value_numbers_u8() {
   let v = Value::u8s(&vec![3,0,4,7,101], "T#1");
   assert_eq!(v.tag(), "U85");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::I8, 0), 3);
   assert_eq!(v.slot(Tag::I8, 1), 0);
   assert_eq!(v.slot(Tag::I8, 2), 4);
   assert_eq!(v.slot(Tag::I8, 3), 7);
   assert_eq!(v.slot(Tag::I8, 4), 101);
}

