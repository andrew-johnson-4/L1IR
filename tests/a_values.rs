use l1_ir::value::{Value,Tag};

#[test]
fn value_nil() {
   let v = Value::unit("T#0");
   assert_eq!(v.tag_as_str(), "Unit");
   assert_eq!(v.name(), "T#0");
   assert_eq!(format!("{:?}",v), "()");
}

#[test]
fn value_number_i8() {
   let v = Value::i8(-2, "T#1");
   assert_eq!(v.tag_as_str(), "I8");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::I8, 0), -2);
   assert_eq!(format!("{:?}",v), "-2");

   let v = Value::i8(100, "T#3");
   assert_eq!(v.tag_as_str(), "I8");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::I8, 0), 100);
   assert_eq!(format!("{:?}",v), "100");
}

#[test]
fn value_number_u8() {
   let v = Value::u8(12, "T#3");
   assert_eq!(v.tag_as_str(), "U8");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::U8, 0), 12);
   assert_eq!(format!("{:?}",v), "12");

   let v = Value::u8(9, "T#4");
   assert_eq!(v.tag_as_str(), "U8");
   assert_eq!(v.name(), "T#4");
   assert_eq!(v.slot(Tag::U8, 0), 9);
   assert_eq!(format!("{:?}",v), "9");
}

#[test]
fn value_numbers_i8s() {
   let v = Value::i8s(&vec![3,-2,4,7], "T#1");
   assert_eq!(v.tag_as_str(), "I84");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::I8, 0), 3);
   assert_eq!(v.slot(Tag::I8, 1), -2);
   assert_eq!(v.slot(Tag::I8, 2), 4);
   assert_eq!(v.slot(Tag::I8, 3), 7);
   assert_eq!(format!("{:?}",v), "(3,-2,4,7)");
}

#[test]
fn value_numbers_u8s() {
   let v = Value::u8s(&vec![3,0,4,7,101], "T#1");
   assert_eq!(v.tag_as_str(), "U85");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::I8, 0), 3);
   assert_eq!(v.slot(Tag::I8, 1), 0);
   assert_eq!(v.slot(Tag::I8, 2), 4);
   assert_eq!(v.slot(Tag::I8, 3), 7);
   assert_eq!(v.slot(Tag::I8, 4), 101);
   assert_eq!(format!("{:?}",v), "(3,0,4,7,101)");
}

#[test]
fn value_number_i16() {
   let v = Value::i16(-2, "T#1");
   assert_eq!(v.tag_as_str(), "I16");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::I16, 0), -2);
   assert_eq!(format!("{:?}",v), "-2");

   let v = Value::i16(100, "T#3");
   assert_eq!(v.tag_as_str(), "I16");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::I16, 0), 100);
   assert_eq!(format!("{:?}",v), "100");
}

#[test]
fn value_number_u16() {
   let v = Value::u16(12, "T#3");
   assert_eq!(v.tag_as_str(), "U16");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::U16, 0), 12);
   assert_eq!(format!("{:?}",v), "12");

   let v = Value::u16(9, "T#4");
   assert_eq!(v.tag_as_str(), "U16");
   assert_eq!(v.name(), "T#4");
   assert_eq!(v.slot(Tag::U16, 0), 9);
   assert_eq!(format!("{:?}",v), "9");
}

#[test]
fn value_numbers_i16s() {
   let v = Value::i16s(&vec![3,-2,4,7], "T#1");
   assert_eq!(v.tag_as_str(), "I164");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::I16, 0), 3);
   assert_eq!(v.slot(Tag::I16, 1), -2);
   assert_eq!(v.slot(Tag::I16, 2), 4);
   assert_eq!(v.slot(Tag::I16, 3), 7);
   assert_eq!(format!("{:?}",v), "(3,-2,4,7)");
}

#[test]
fn value_numbers_u16s() {
   let v = Value::u16s(&vec![3,0,4,7,101], "T#1");
   assert_eq!(v.tag_as_str(), "U165");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::I16, 0), 3);
   assert_eq!(v.slot(Tag::I16, 1), 0);
   assert_eq!(v.slot(Tag::I16, 2), 4);
   assert_eq!(v.slot(Tag::I16, 3), 7);
   assert_eq!(v.slot(Tag::I16, 4), 101);
   assert_eq!(format!("{:?}",v), "(3,0,4,7,101)");
}

#[test]
fn value_number_i32() {
   let v = Value::i32(-2, "T#1");
   assert_eq!(v.tag_as_str(), "I32");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::I32, 0), -2);
   assert_eq!(format!("{:?}",v), "-2");

   let v = Value::i32(100, "T#3");
   assert_eq!(v.tag_as_str(), "I32");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::I32, 0), 100);
   assert_eq!(format!("{:?}",v), "100");
}

#[test]
fn value_number_u32() {
   let v = Value::u32(12, "T#3");
   assert_eq!(v.tag_as_str(), "U32");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::U32, 0), 12);
   assert_eq!(format!("{:?}",v), "12");

   let v = Value::u32(9, "T#4");
   assert_eq!(v.tag_as_str(), "U32");
   assert_eq!(v.name(), "T#4");
   assert_eq!(v.slot(Tag::U32, 0), 9);
   assert_eq!(format!("{:?}",v), "9");
}

#[test]
fn value_numbers_i32s() {
   let v = Value::i32s(&vec![3,-2,4], "T#1");
   assert_eq!(v.tag_as_str(), "I323");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::I32, 0), 3);
   assert_eq!(v.slot(Tag::I32, 1), -2);
   assert_eq!(v.slot(Tag::I32, 2), 4);
   assert_eq!(format!("{:?}",v), "(3,-2,4)");
}

#[test]
fn value_numbers_u32s() {
   let v = Value::u32s(&vec![3,0,4], "T#1");
   assert_eq!(v.tag_as_str(), "U323");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::U32, 0), 3);
   assert_eq!(v.slot(Tag::U32, 1), 0);
   assert_eq!(v.slot(Tag::U32, 2), 4);
   assert_eq!(format!("{:?}",v), "(3,0,4)");
}

#[test]
fn value_number_f32() {
   let v = Value::f32(-2.0, "T#1");
   assert_eq!(v.tag_as_str(), "F32");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::F32, 0), 3221225472);
   assert_eq!(format!("{:?}",v), "-2.00000");
	
   let v = Value::f32(100.0, "T#3");
   assert_eq!(v.tag_as_str(), "F32");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::F32, 0), 1120403456);
   assert_eq!(format!("{:?}",v), "100.00000");
}

#[test]
fn value_numbers_f32s() {
   let v = Value::f32s(&vec![3.2,0.5,4.321], "T#1");
   assert_eq!(v.tag_as_str(), "F323");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::F32, 0), 1078774989);
   assert_eq!(v.slot(Tag::F32, 1), 1056964608);
   assert_eq!(v.slot(Tag::F32, 2), 1082803618);
   assert_eq!(format!("{:?}",v), "(3.20000,0.50000,4.32100)");
}

#[test]
fn value_number_i64() {
   let v = Value::i64(-2, "T#1");
   assert_eq!(v.tag_as_str(), "I64");
   assert_eq!(v.name(), "T#1");
   assert_eq!(v.slot(Tag::I64, 0), -2);
   assert_eq!(format!("{:?}",v), "-2");

   let v = Value::i64(100, "T#3");
   assert_eq!(v.tag_as_str(), "I64");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::I64, 0), 100);
   assert_eq!(format!("{:?}",v), "100");
}

#[test]
fn value_number_u64() {
   let v = Value::u64(12, "T#3");
   assert_eq!(v.tag_as_str(), "U64");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::U64, 0), 12);
   assert_eq!(format!("{:?}",v), "12");

   let v = Value::u64(9, "T#4");
   assert_eq!(v.tag_as_str(), "U64");
   assert_eq!(v.name(), "T#4");
   assert_eq!(v.slot(Tag::U64, 0), 9);
   assert_eq!(format!("{:?}",v), "9");
}

#[test]
fn value_number_f64() {
   let v = Value::f64(12.345, "T#3");
   assert_eq!(v.tag_as_str(), "F64");
   assert_eq!(v.name(), "T#3");
   assert_eq!(v.slot(Tag::F64, 0), 4623139235229744497);
   assert_eq!(format!("{:?}",v), "12.34500");

   let v = Value::f64(-98.76543210, "T#4");
   assert_eq!(v.tag_as_str(), "F64");
   assert_eq!(v.name(), "T#4");
   assert_eq!(v.slot(Tag::F64, 0), 13860022453216687040);
   assert_eq!(format!("{:?}",v), "-98.76543");
}

#[test]
fn value_string() {
   let v = Value::string("abc", "T#2");
   assert_eq!(v.tag_as_str(), "String");
   assert_eq!(v.name(), "T#2");
   assert_eq!(v.literal(), "abc");
   assert_eq!(format!("{:?}",v), r#""abc""#);

   let v2 = v.slice(1,2);
   assert_eq!(v2.tag_as_str(), "String");
   assert_eq!(v2.name(), "T#2");
   assert_eq!(v2.literal(), "b");
   assert_eq!(format!("{:?}",v2), r#""b""#);
}
