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
use std::sync::Mutex;

#[derive(FromPrimitive)]
#[derive(Debug)]
#[repr(u16)]
pub enum Tag {
   Unit,
   I8, I82, I83, I84, I85, I86, I87, I88, I89, I810, I811, I812,
   U8, U82, U83, U84, U85, U86, U87, U88, U89, U810, U811, U812,
   U16, U162, U163, U164, U165, U166,
   I16, I162, I163, I164, I165, I166,
   U32, U322, U323,
   I32, I322, I323,
   F32, F322, F323,
   U64,
   I64,
   F64,
   String,
   Tuple,
}

static NAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub struct Value(u128);

impl Value {
   pub fn from_parts(tag: u16, name: u16, slots: u128) -> Value {
      let tag = (tag as u128) << 112;
      let name = (name as u128) << 96;
      Value(tag | name | slots)
   }
   pub fn push_name(nom: &str) -> u16 {
      let mut ns = NAMES.lock().unwrap();
      for (ni,n) in ns.iter().enumerate() {
      if n == nom {
         return ni as u16;
      }}
      let ni = ns.len();
      ns.push(nom.to_string());
      ni as u16
   }
   pub fn unit(nom: &str) -> Value {
      Value::from_parts(Tag::Unit as u16, Value::push_name(nom), 0)
   }
   pub fn string(lit: &str, nom: &str) -> Value {
      Value::from_parts(Tag::String as u16, Value::push_name(nom), 0)
   }
   pub fn i8(slot: i8, nom: &str) -> Value {
      Value::from_parts(Tag::I8 as u16, Value::push_name(nom), (slot as u8) as u128)
   }
   pub fn u8(slot: u8, nom: &str) -> Value {
      Value::from_parts(Tag::U8 as u16, Value::push_name(nom), (slot as u8) as u128)
   }
   pub fn i8s(slots: &[i8], nom: &str) -> Value {
      let mut v: u128 = 0;
      unsafe {
         if slots.len()>=12 { v += std::mem::transmute::<i8,u8>(slots[11]) as u128; } v <<= 8;
         if slots.len()>=11 { v += std::mem::transmute::<i8,u8>(slots[10]) as u128; } v <<= 8;
         if slots.len()>=10 { v += std::mem::transmute::<i8,u8>(slots[9])  as u128; } v <<= 8;
         if slots.len()>=9  { v += std::mem::transmute::<i8,u8>(slots[8])  as u128; } v <<= 8;
         if slots.len()>=8  { v += std::mem::transmute::<i8,u8>(slots[7])  as u128; } v <<= 8;
         if slots.len()>=7  { v += std::mem::transmute::<i8,u8>(slots[6])  as u128; } v <<= 8;
         if slots.len()>=6  { v += std::mem::transmute::<i8,u8>(slots[5])  as u128; } v <<= 8;
         if slots.len()>=5  { v += std::mem::transmute::<i8,u8>(slots[4])  as u128; } v <<= 8;
         if slots.len()>=4  { v += std::mem::transmute::<i8,u8>(slots[3])  as u128; } v <<= 8;
         if slots.len()>=3  { v += std::mem::transmute::<i8,u8>(slots[2])  as u128; } v <<= 8;
         if slots.len()>=2  { v += std::mem::transmute::<i8,u8>(slots[1])  as u128; } v <<= 8;
         if slots.len()>=1  { v += std::mem::transmute::<i8,u8>(slots[0])  as u128; }
      }
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, Value::push_name(nom), v),
         1 => Value::from_parts(Tag::I8 as u16, Value::push_name(nom), v),
         2 => Value::from_parts(Tag::I82 as u16, Value::push_name(nom), v),
         3 => Value::from_parts(Tag::I83 as u16, Value::push_name(nom), v),
         4 => Value::from_parts(Tag::I84 as u16, Value::push_name(nom), v),
         5 => Value::from_parts(Tag::I85 as u16, Value::push_name(nom), v),
         6 => Value::from_parts(Tag::I86 as u16, Value::push_name(nom), v),
         7 => Value::from_parts(Tag::I87 as u16, Value::push_name(nom), v),
         8 => Value::from_parts(Tag::I88 as u16, Value::push_name(nom), v),
         9 => Value::from_parts(Tag::I89 as u16, Value::push_name(nom), v),
         10 => Value::from_parts(Tag::I810 as u16, Value::push_name(nom), v),
         11 => Value::from_parts(Tag::I811 as u16, Value::push_name(nom), v),
         12 => Value::from_parts(Tag::I812 as u16, Value::push_name(nom), v),
         _ => unreachable!(),
      }
   }
   pub fn u8s(slots: &[u8], nom: &str) -> Value {
      let mut v: u128 = 0;
      if slots.len()>=12 { v += slots[11] as u128; } v <<= 8;
      if slots.len()>=11 { v += slots[10] as u128; } v <<= 8;
      if slots.len()>=10 { v += slots[9]  as u128; } v <<= 8;
      if slots.len()>=9  { v += slots[8]  as u128; } v <<= 8;
      if slots.len()>=8  { v += slots[7]  as u128; } v <<= 8;
      if slots.len()>=7  { v += slots[6]  as u128; } v <<= 8;
      if slots.len()>=6  { v += slots[5]  as u128; } v <<= 8;
      if slots.len()>=5  { v += slots[4]  as u128; } v <<= 8;
      if slots.len()>=4  { v += slots[3]  as u128; } v <<= 8;
      if slots.len()>=3  { v += slots[2]  as u128; } v <<= 8;
      if slots.len()>=2  { v += slots[1]  as u128; } v <<= 8;
      if slots.len()>=1  { v += slots[0]  as u128; }
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, Value::push_name(nom), v),
         1 => Value::from_parts(Tag::U8 as u16, Value::push_name(nom), v),
         2 => Value::from_parts(Tag::U82 as u16, Value::push_name(nom), v),
         3 => Value::from_parts(Tag::U83 as u16, Value::push_name(nom), v),
         4 => Value::from_parts(Tag::U84 as u16, Value::push_name(nom), v),
         5 => Value::from_parts(Tag::U85 as u16, Value::push_name(nom), v),
         6 => Value::from_parts(Tag::U86 as u16, Value::push_name(nom), v),
         7 => Value::from_parts(Tag::U87 as u16, Value::push_name(nom), v),
         8 => Value::from_parts(Tag::U88 as u16, Value::push_name(nom), v),
         9 => Value::from_parts(Tag::U89 as u16, Value::push_name(nom), v),
         10 => Value::from_parts(Tag::U810 as u16, Value::push_name(nom), v),
         11 => Value::from_parts(Tag::U811 as u16, Value::push_name(nom), v),
         12 => Value::from_parts(Tag::U812 as u16, Value::push_name(nom), v),
         _ => unreachable!(),
      }
   }
   pub fn i16(slot: i16, nom: &str) -> Value {
      Value::from_parts(Tag::I16 as u16, Value::push_name(nom), (slot as u16) as u128)
   }
   pub fn u16(slot: u16, nom: &str) -> Value {
      Value::from_parts(Tag::U16 as u16, Value::push_name(nom), (slot as u16) as u128)
   }
   pub fn i16s(slots: &[i16], nom: &str) -> Value {
      let mut v: u128 = 0;
      unsafe {
         if slots.len()>=6  { v += std::mem::transmute::<i16,u16>(slots[5])  as u128; } v <<= 16;
         if slots.len()>=5  { v += std::mem::transmute::<i16,u16>(slots[4])  as u128; } v <<= 16;
         if slots.len()>=4  { v += std::mem::transmute::<i16,u16>(slots[3])  as u128; } v <<= 16;
         if slots.len()>=3  { v += std::mem::transmute::<i16,u16>(slots[2])  as u128; } v <<= 16;
         if slots.len()>=2  { v += std::mem::transmute::<i16,u16>(slots[1])  as u128; } v <<= 16;
         if slots.len()>=1  { v += std::mem::transmute::<i16,u16>(slots[0])  as u128; }
      }
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, Value::push_name(nom), v),
         1 => Value::from_parts(Tag::I16 as u16, Value::push_name(nom), v),
         2 => Value::from_parts(Tag::I162 as u16, Value::push_name(nom), v),
         3 => Value::from_parts(Tag::I163 as u16, Value::push_name(nom), v),
         4 => Value::from_parts(Tag::I164 as u16, Value::push_name(nom), v),
         5 => Value::from_parts(Tag::I165 as u16, Value::push_name(nom), v),
         6 => Value::from_parts(Tag::I166 as u16, Value::push_name(nom), v),
         _ => unreachable!(),
      }
   }
   pub fn u16s(slots: &[u16], nom: &str) -> Value {
      let mut v: u128 = 0;
      if slots.len()>=6  { v += slots[5]  as u128; } v <<= 16;
      if slots.len()>=5  { v += slots[4]  as u128; } v <<= 16;
      if slots.len()>=4  { v += slots[3]  as u128; } v <<= 16;
      if slots.len()>=3  { v += slots[2]  as u128; } v <<= 16;
      if slots.len()>=2  { v += slots[1]  as u128; } v <<= 16;
      if slots.len()>=1  { v += slots[0]  as u128; }
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, Value::push_name(nom), v),
         1 => Value::from_parts(Tag::U16 as u16, Value::push_name(nom), v),
         2 => Value::from_parts(Tag::U162 as u16, Value::push_name(nom), v),
         3 => Value::from_parts(Tag::U163 as u16, Value::push_name(nom), v),
         4 => Value::from_parts(Tag::U164 as u16, Value::push_name(nom), v),
         5 => Value::from_parts(Tag::U165 as u16, Value::push_name(nom), v),
         6 => Value::from_parts(Tag::U166 as u16, Value::push_name(nom), v),
         _ => unreachable!(),
      }
   }
   pub fn i32(slot: i32, nom: &str) -> Value {
      Value::from_parts(Tag::I32 as u16, Value::push_name(nom), (slot as u32) as u128)
   }
   pub fn u32(slot: u32, nom: &str) -> Value {
      Value::from_parts(Tag::U32 as u16, Value::push_name(nom), (slot as u32) as u128)
   }
   pub fn i32s(slots: &[i32], nom: &str) -> Value {
      let mut v: u128 = 0;
      unsafe {
         if slots.len()>=3  { v += std::mem::transmute::<i32,u32>(slots[2])  as u128; } v <<= 32;
         if slots.len()>=2  { v += std::mem::transmute::<i32,u32>(slots[1])  as u128; } v <<= 32;
         if slots.len()>=1  { v += std::mem::transmute::<i32,u32>(slots[0])  as u128; }
      }
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, Value::push_name(nom), v),
         1 => Value::from_parts(Tag::I32 as u16, Value::push_name(nom), v),
         2 => Value::from_parts(Tag::I322 as u16, Value::push_name(nom), v),
         3 => Value::from_parts(Tag::I323 as u16, Value::push_name(nom), v),
         _ => unreachable!(),
      }
   }
   pub fn u32s(slots: &[u32], nom: &str) -> Value {
      let mut v: u128 = 0;
      if slots.len()>=3  { v += slots[2]  as u128; } v <<= 32;
      if slots.len()>=2  { v += slots[1]  as u128; } v <<= 32;
      if slots.len()>=1  { v += slots[0]  as u128; }
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, Value::push_name(nom), v),
         1 => Value::from_parts(Tag::U32 as u16, Value::push_name(nom), v),
         2 => Value::from_parts(Tag::U322 as u16, Value::push_name(nom), v),
         3 => Value::from_parts(Tag::U323 as u16, Value::push_name(nom), v),
         _ => unreachable!(),
      }
   }
   pub fn f32(slot: f32, nom: &str) -> Value {
      let slot = unsafe { std::mem::transmute::<f32,u32>(slot) };
      Value::from_parts(Tag::F32 as u16, Value::push_name(nom), slot as u128)
   }
   pub fn f32s(slots: &[f32], nom: &str) -> Value {
      let mut v: u128 = 0;
      unsafe {
         if slots.len()>=3  { v += std::mem::transmute::<f32,u32>(slots[2])  as u128; } v <<= 32;
         if slots.len()>=2  { v += std::mem::transmute::<f32,u32>(slots[1])  as u128; } v <<= 32;
         if slots.len()>=1  { v += std::mem::transmute::<f32,u32>(slots[0])  as u128; }
      }
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, Value::push_name(nom), v),
         1 => Value::from_parts(Tag::F32 as u16, Value::push_name(nom), v),
         2 => Value::from_parts(Tag::F322 as u16, Value::push_name(nom), v),
         3 => Value::from_parts(Tag::F323 as u16, Value::push_name(nom), v),
         _ => unreachable!(),
      }
   }
   pub fn i64(slot: i64, nom: &str) -> Value {
      Value::from_parts(Tag::I64 as u16, Value::push_name(nom), (slot as u64) as u128)
   }
   pub fn u64(slot: u64, nom: &str) -> Value {
      Value::from_parts(Tag::U64 as u16, Value::push_name(nom), (slot as u64) as u128)
   }
   pub fn f64(slot: f64, nom: &str) -> Value {
      let slot = unsafe { std::mem::transmute::<f64,u64>(slot) };
      Value::from_parts(Tag::F64 as u16, Value::push_name(nom), slot as u128)
   }
   pub fn tag<'t>(&self) -> String {
      let t = (self.0 >> 112) as u16;
      let t: Tag = FromPrimitive::from_i32(t.into()).expect(&format!("Invalid Tag in Value: {}", t));
      format!("{:?}", t)
   }
   pub fn name(&self) -> String {
      let ni = ((self.0 << 16) >> 112) as usize;
      let ns = NAMES.lock().unwrap();
      ns[ni].clone()
   }
   pub fn slice(&self, start: usize, end: usize) -> Value {
      Value(self.0)
   }
   pub fn literal(&self) -> String {
      unimplemented!("Value::literal")
   }
   pub fn slot(&self, tag: Tag, slot: usize) -> i128 {
      let mut s = ((self.0 << 32) >> 32) as u128;
      match tag {
         Tag::Unit => { s = 0; },
         Tag::U8|Tag::U82|Tag::U83|Tag::U84|Tag::U85|Tag::U86|Tag::U87|Tag::U88|Tag::U89|Tag::U810|Tag::U811|Tag::U812 => {
            s <<= 32 + 8 * (11 - slot);
            s >>= 32 + 8 * 11;
         },
         Tag::I8|Tag::I82|Tag::I83|Tag::I84|Tag::I85|Tag::I86|Tag::I87|Tag::I88|Tag::I89|Tag::I810|Tag::I811|Tag::I812 => {
            s <<= 32 + 8 * (11 - slot);
            s >>= 32 + 8 * 11;
            let sv = s as u8;
            s = unsafe { std::mem::transmute::<u8,i8>(sv) } as u128;
         },
         Tag::U16|Tag::U162|Tag::U163|Tag::U164|Tag::U165|Tag::U166 => {
            s <<= 32 + 16 * (5 - slot);
            s >>= 32 + 16 * 5;
         },
         Tag::I16|Tag::I162|Tag::I163|Tag::I164|Tag::I165|Tag::I166 => {
            s <<= 32 + 16 * (5 - slot);
            s >>= 32 + 16 * 5;
            let sv = s as u16;
            s = unsafe { std::mem::transmute::<u16,i16>(sv) } as u128;
         },
         Tag::U32|Tag::U322|Tag::U323 => {
            s <<= 32 + 32 * (2 - slot);
            s >>= 32 + 32 * 2;
         },
         Tag::I32|Tag::I322|Tag::I323 => {
            s <<= 32 + 32 * (2 - slot);
            s >>= 32 + 32 * 2;
            let sv = s as u32;
            s = unsafe { std::mem::transmute::<u32,i32>(sv) } as u128;
         },
         Tag::F32|Tag::F322|Tag::F323 => {
            s <<= 32 + 32 * (2 - slot);
            s >>= 32 + 32 * 2;
         },
         Tag::U64 => {},
         Tag::I64 => {
            let sv = s as u64;
            s = unsafe { std::mem::transmute::<u64,i64>(sv) } as u128;
         },
         Tag::F64 => {},
         _ => { panic!("Could not cast {:?} as I128",tag) },
      }
      unsafe { std::mem::transmute::<u128,i128>(s) }
   }
}
