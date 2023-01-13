//Each IR "Type" must have a single unambiguous Value representation with the exception of `String and `Tuple
//i.e. there must exist a unique mapping from any supported Cranelift Type to an L1 Value
//This assertion does not presume that all Types are "Values", just that there exists a relation

//type Value : U128 : U32[4]
//tag: U16 | nominal_type: U16 | vals: U32[3]
//`Unit    | `T                |
//`U8#     | `T                | U8[12]
//`I8#     | `T                | I8[12]
//`U16#    | `T                | U16[6]
//`I16#    | `T                | I16[6]
//`U32#    | `T                | U32[3]
//`I32#    | `T                | I32[3]
//`U64     | `T                | U64
//`I64     | `T                | I64
//`F32#    | `T                | F32[3]
//`F64     | `T                | F64
//`String  | `T                | start: U32 | end: U32 | U32 Offset -> StringData
//`Tuple   | `T                | start: U32 | end: U32 | U32 Offset -> TupleData

//type StringData: ?Sized
//  ref_count: U64
//  data: U32[SIZE]

//type TupleData: ?Sized
//  ref_count: U64
//  data: Value[SIZE]

use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(FromPrimitive)]
#[derive(Debug)]
#[repr(u16)]
pub enum Tag {
   Unit,
   I8, I82, I83, I84, I85, I86, I87, I88, I89, I810, I811, I812,
   U8, U82, U83, U84, U85, U86, U87, U88, U89, U810, U811, U812,
}

pub struct Value(u128);

impl Value {
   pub fn from_parts(tag: u16, name: u16, slots: u128) -> Value {
      let tag = (tag as u128) << 112;
      let name = (name as u128) << 96;
      Value(tag + name + slots)
   }
   pub fn unit(nom: &str) -> Value {
      Value::from_parts(Tag::Unit as u16, 0, 0)
   }
   pub fn i8(slot: i8, nom: &str) -> Value {
      Value::from_parts(Tag::I8 as u16, 0, 0)
   }
   pub fn u8(slot: u8, nom: &str) -> Value {
      Value::from_parts(Tag::U8 as u16, 0, 0)
   }
   pub fn i8s(slots: &[i8], nom: &str) -> Value {
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, 0, 0),
         1 => Value::from_parts(Tag::I8 as u16, 0, 0),
         2 => Value::from_parts(Tag::I82 as u16, 0, 0),
         3 => Value::from_parts(Tag::I83 as u16, 0, 0),
         4 => Value::from_parts(Tag::I84 as u16, 0, 0),
         5 => Value::from_parts(Tag::I85 as u16, 0, 0),
         6 => Value::from_parts(Tag::I86 as u16, 0, 0),
         7 => Value::from_parts(Tag::I87 as u16, 0, 0),
         8 => Value::from_parts(Tag::I88 as u16, 0, 0),
         9 => Value::from_parts(Tag::I89 as u16, 0, 0),
         10 => Value::from_parts(Tag::I810 as u16, 0, 0),
         11 => Value::from_parts(Tag::I811 as u16, 0, 0),
         12 => Value::from_parts(Tag::I812 as u16, 0, 0),
         _ => unreachable!(),
      }
   }
   pub fn u8s(slots: &[u8], nom: &str) -> Value {
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, 0, 0),
         1 => Value::from_parts(Tag::U8 as u16, 0, 0),
         2 => Value::from_parts(Tag::U82 as u16, 0, 0),
         3 => Value::from_parts(Tag::U83 as u16, 0, 0),
         4 => Value::from_parts(Tag::U84 as u16, 0, 0),
         5 => Value::from_parts(Tag::U85 as u16, 0, 0),
         6 => Value::from_parts(Tag::U86 as u16, 0, 0),
         7 => Value::from_parts(Tag::U87 as u16, 0, 0),
         8 => Value::from_parts(Tag::U88 as u16, 0, 0),
         9 => Value::from_parts(Tag::U89 as u16, 0, 0),
         10 => Value::from_parts(Tag::U810 as u16, 0, 0),
         11 => Value::from_parts(Tag::U811 as u16, 0, 0),
         12 => Value::from_parts(Tag::U812 as u16, 0, 0),
         _ => unreachable!(),
      }
   }
   pub fn tag<'t>(&self) -> String {
      let t = (self.0 >> 112) as u16;
      let t: Tag = FromPrimitive::from_i32(t.into()).expect("Invalid Tag in Value");
      format!("{:?}", t)
   }
   pub fn name(&self) -> String {
      format!("Type#123")
   }
   pub fn slot(&self, slot: usize) -> i128 {
      0
   }
}
