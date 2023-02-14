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
//`String  | `T                | start: U16 | end: U16 | U64 Offset -> StringData
//`Tuple   | `T                | start: U16 | end: U16 | U64 Offset -> TupleData

//type StringData: ?Sized
//  ref_count: U32
//  data: U32[SIZE]

//type TupleData: ?Sized
//  ref_count: U32
//  data: Value[SIZE]

use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use std::alloc::{alloc_zeroed};
use std::iter::FromIterator;
use std::io::Write;

#[derive(FromPrimitive,Copy,Clone,Debug,PartialEq,Eq)]
#[repr(u16)]
pub enum Tag {
   Zero = 0, Unit,
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

pub struct Value(pub u128);

impl PartialEq for Value {
   fn eq(&self, other: &Self) -> bool {
      format!("{:?}",self) == format!("{:?}",other)
   }
}
impl Eq for Value {}
impl std::fmt::Debug for Value {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let tag = self.tag();
      match tag {
         Tag::Zero => write!(f,"_"),
         Tag::Unit => write!(f,"()"),
         Tag::I8 => write!(f,"{}",self.slot(tag,0)),
         Tag::I82 => write!(f,"({},{})",self.slot(tag,0), self.slot(tag,1)),
         Tag::I83 => write!(f,"({},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2)),
         Tag::I84 => write!(f,"({},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3)),
         Tag::I85 => write!(f,"({},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4)),
         Tag::I86 => write!(f,"({},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5)),
         Tag::I87 => write!(f,"({},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6)),
         Tag::I88 => write!(f,"({},{},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6), self.slot(tag,7)),
         Tag::I89 => write!(f,"({},{},{},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6), self.slot(tag,7), self.slot(tag,8)),
         Tag::I810 => write!(f,"({},{},{},{},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6), self.slot(tag,7), self.slot(tag,8), self.slot(tag,9)),
         Tag::I811 => write!(f,"({},{},{},{},{},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6), self.slot(tag,7), self.slot(tag,8), self.slot(tag,9), self.slot(tag,10)),
         Tag::I812 => write!(f,"({},{},{},{},{},{},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6), self.slot(tag,7), self.slot(tag,8), self.slot(tag,9), self.slot(tag,10), self.slot(tag,11)),
         Tag::U8 => write!(f,"{}",self.slot(tag,0)),
         Tag::U82 => write!(f,"({},{})",self.slot(tag,0), self.slot(tag,1)),
         Tag::U83 => write!(f,"({},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2)),
         Tag::U84 => write!(f,"({},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3)),
         Tag::U85 => write!(f,"({},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4)),
         Tag::U86 => write!(f,"({},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5)),
         Tag::U87 => write!(f,"({},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6)),
         Tag::U88 => write!(f,"({},{},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6), self.slot(tag,7)),
         Tag::U89 => write!(f,"({},{},{},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6), self.slot(tag,7), self.slot(tag,8)),
         Tag::U810 => write!(f,"({},{},{},{},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6), self.slot(tag,7), self.slot(tag,8), self.slot(tag,9)),
         Tag::U811 => write!(f,"({},{},{},{},{},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6), self.slot(tag,7), self.slot(tag,8), self.slot(tag,9), self.slot(tag,10)),
         Tag::U812 => write!(f,"({},{},{},{},{},{},{},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5), self.slot(tag,6), self.slot(tag,7), self.slot(tag,8), self.slot(tag,9), self.slot(tag,10), self.slot(tag,11)),
         Tag::I16 => write!(f,"{}",self.slot(tag,0)),
         Tag::I162 => write!(f,"({},{})",self.slot(tag,0), self.slot(tag,1)),
         Tag::I163 => write!(f,"({},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2)),
         Tag::I164 => write!(f,"({},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3)),
         Tag::I165 => write!(f,"({},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4)),
         Tag::I166 => write!(f,"({},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5)),
         Tag::U16 => write!(f,"{}",self.slot(tag,0)),
         Tag::U162 => write!(f,"({},{})",self.slot(tag,0), self.slot(tag,1)),
         Tag::U163 => write!(f,"({},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2)),
         Tag::U164 => write!(f,"({},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3)),
         Tag::U165 => write!(f,"({},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4)),
         Tag::U166 => write!(f,"({},{},{},{},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2), self.slot(tag,3),
            self.slot(tag,4), self.slot(tag,5)),
         Tag::I32 => write!(f,"{}",self.slot(tag,0)),
         Tag::I322 => write!(f,"({},{})",self.slot(tag,0), self.slot(tag,1)),
         Tag::I323 => write!(f,"({},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2)),
         Tag::U32 => write!(f,"{}",self.slot(tag,0)),
         Tag::U322 => write!(f,"({},{})",self.slot(tag,0), self.slot(tag,1)),
         Tag::U323 => write!(f,"({},{},{})",self.slot(tag,0), self.slot(tag,1), self.slot(tag,2)),
         Tag::F32 => {
            let v = unsafe { std::mem::transmute::<u32,f32>(self.slot(tag,0) as u32) };
            write!(f,"{:.5}",v)
         },
         Tag::F322 => {
            let v1 = unsafe { std::mem::transmute::<u32,f32>(self.slot(tag,0) as u32) };
            let v2 = unsafe { std::mem::transmute::<u32,f32>(self.slot(tag,1) as u32) };
            write!(f,"({:.5},{:.5})",v1,v2)
         },
         Tag::F323 => {
            let v1 = unsafe { std::mem::transmute::<u32,f32>(self.slot(tag,0) as u32) };
            let v2 = unsafe { std::mem::transmute::<u32,f32>(self.slot(tag,1) as u32) };
            let v3 = unsafe { std::mem::transmute::<u32,f32>(self.slot(tag,2) as u32) };
            write!(f,"({:.5},{:.5},{:.5})",v1,v2,v3)
         },
         Tag::I64 => write!(f,"{}",self.slot(tag,0)),
         Tag::U64 => write!(f,"{}",self.slot(tag,0)),
         Tag::F64 => {
            let v = unsafe { std::mem::transmute::<u64,f64>(self.slot(tag,0) as u64) };
            write!(f,"{:.5}",v)
         },
         Tag::String => write!(f, "{:?}", self.literal()),
         Tag::Tuple => {
            let start = self.start();
            let end = self.end();
            write!(f, "(")?;
            for ti in start..end {
               if ti>start { write!(f, ",")?; }
               write!(f, "{:?}", self.vslot(ti))?;
            }
            if start==end { write!(f,",")?; }
            write!(f, ")")
         }
      }
   }
}

impl Value {
   pub fn from_lohi(lo: u64, hi: u64) -> Value {
      Value( ((hi as u128) << 64) | (lo as u128) )
   }
   pub fn lohi(&self) -> (u64,u64) {
      let lo = ((self.0 << 64) >> 64) as u64;
      let hi = (self.0 >> 64) as u64;
      (lo,hi)
   }
   pub fn from_parts(tag: u16, name: u16, slots: u128) -> Value {
      dprintln!("from parts: ({},{},{})", tag, name, slots);
      let tag = (tag as u128) << 112;
      let name = (name as u128) << 96;
      Value(tag | name | slots)
   }
   pub fn to_parts(&self) -> (u16,u16,u128) {
      dprintln!("to parts: ({})", self.0);
      let tag = self.0 >> 112;
      let name = (self.0 << 16) >> 112;
      let slots = (self.0 << 32) >> 32;
      (tag as u16, name as u16, slots)
   }
   pub fn zero() -> Value {
      dprintln!("Value::zero");
      Value(0)
   }
   pub fn unit(_nom: &str) -> Value {
      dprintln!("Value::unit");
      Value::from_parts(Tag::Unit as u16, 0, 0)
   }
   pub fn range(from: u64, to: u64, step: u64) -> Value {
      dprintln!("Value::range({},{},{})", from, to, step);
      let mut vs = Vec::new();
      for i in (from..to).step_by(step as usize) {
         vs.push(Value::u64(i,"U64"));
      }
      Value::tuple(&vs,"Tuple")
   }
   pub fn string(lit: &str, _nom: &str) -> Value {
      dprintln!("Value::string({})", lit);
      let cs = lit.chars().collect::<Vec<char>>();
      let layout = std::alloc::Layout::array::<u32>(cs.len()).unwrap();
      let ptr = unsafe {
         let ptr = alloc_zeroed(layout) as *mut u32;
         if ptr.is_null() {
            panic!("Failed to allocate new memory for String");
         }
         for ci in 0..cs.len() {
            *ptr.offset(ci as isize) = cs[ci] as u32;
         }
         ptr
      };
      let ptr_bits = (ptr as usize) as u128;
      let start = 0 as u128;
      let end = cs.len() as u128;
      let mut raw: u128 = 0;
      raw |= start; raw <<= 16;
      raw |= end;   raw <<= 64;
      raw |= ptr_bits;
      Value::from_parts(Tag::String as u16, 0, raw)
   }
   pub fn tuple_with_capacity(cap: u64) -> Value {
      dprintln!("Value::tuple_with_capacity({})", cap);
      let mut vs = Vec::new();
      for _ in 0..cap {
         vs.push(Value::zero());
      }
      Value::tuple(&vs,"Tuple")
   }
   pub fn tuple(vs: &[Value], _nom: &str) -> Value {
      dprintln!("Value::tuple([{}])", vs.len());
      let layout = std::alloc::Layout::array::<u128>(vs.len()).unwrap();
      let ptr = unsafe {
         let ptr = alloc_zeroed(layout) as *mut u128;
         if ptr.is_null() {
            panic!("Failed to allocate new memory for Tuple");
         }
         for vi in 0..vs.len() {
            *ptr.offset(vi as isize) = vs[vi].0;
         }
         ptr
      };
      let ptr_bits_64 = (ptr as usize) as u64;
      let ptr_bits = (ptr as usize) as u128;
      let start = 0 as u128;
      let end = vs.len() as u128;
      let mut raw: u128 = 0;
      raw |= start; raw <<= 16;
      raw |= end;   raw <<= 64;
      raw |= ptr_bits;
      dprintln!("ptr_bits {}:u64 {}:u128 {}:start {}:end {}:raw", ptr_bits_64, ptr_bits, start, end, raw);
      let v = Value::from_parts(Tag::Tuple as u16, 0, raw);
      dprintln!("actual {}:ptr {}:start {}:end", v.tptr() as u128, v.start(), v.end());
      v
   }
   pub fn start(&self) -> usize {
      assert!(self.tag() == Tag::Tuple ||
              self.tag() == Tag::String, ".start must be `Tuple or `String");
      let mut raw = self.0;
      raw <<= 32; raw >>= 32;
      raw >>= 80;
      raw as usize
   }
   pub fn set_end(&mut self, end: usize) {
      dprintln!("Value::set_end({})", end);
      assert!(self.tag() == Tag::Tuple, "set_end must be `Tuple");
      assert!(end <= self.end(), "set end expected: {} < {}", end, self.end());
      let ptr_bits = (self.tptr() as usize) as u128;
      let start = self.start() as u128;
      let end = end as u128;
      let mut raw: u128 = 0;
      raw |= start; raw <<= 16;
      raw |= end;   raw <<= 64;
      raw |= ptr_bits;
      self.0 = Value::from_parts(Tag::Tuple as u16, 0, raw).0;
   }
   pub fn end(&self) -> usize {
      assert!(self.tag() == Tag::Tuple ||
              self.tag() == Tag::String, ".end must be `Tuple or `String");
      let mut raw = self.0;
      raw <<= 48; raw >>= 48;
      raw >>= 64;
      raw as usize
   }
   pub fn cptr(&self) -> *mut u32 {
      assert!(self.tag() == Tag::String, "String::ptr must be `String");
      let mut raw = self.0;
      raw <<= 64; raw >>= 64;
      raw as *mut u32
   }
   pub fn tptr(&self) -> *mut u128 {
      assert!(self.tag() == Tag::Tuple, "Tuple::ptr must be `Tuple");
      let mut raw = self.0;
      raw <<= 64; raw >>= 64;
      raw as *mut u128
   }
   pub fn i8(slot: i8, _nom: &str) -> Value {
      Value::from_parts(Tag::I8 as u16, 0, (slot as u8) as u128)
   }
   pub fn u8(slot: u8, _nom: &str) -> Value {
      dprintln!("Value::u8");
      Value::from_parts(Tag::U8 as u16, 0, (slot as u8) as u128)
   }
   pub fn i8s(slots: &[i8], _nom: &str) -> Value {
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
         0 => Value::from_parts(Tag::Unit as u16, 0, v),
         1 => Value::from_parts(Tag::I8 as u16, 0, v),
         2 => Value::from_parts(Tag::I82 as u16, 0, v),
         3 => Value::from_parts(Tag::I83 as u16, 0, v),
         4 => Value::from_parts(Tag::I84 as u16, 0, v),
         5 => Value::from_parts(Tag::I85 as u16, 0, v),
         6 => Value::from_parts(Tag::I86 as u16, 0, v),
         7 => Value::from_parts(Tag::I87 as u16, 0, v),
         8 => Value::from_parts(Tag::I88 as u16, 0, v),
         9 => Value::from_parts(Tag::I89 as u16, 0, v),
         10 => Value::from_parts(Tag::I810 as u16, 0, v),
         11 => Value::from_parts(Tag::I811 as u16, 0, v),
         12 => Value::from_parts(Tag::I812 as u16, 0, v),
         _ => unreachable!(),
      }
   }
   pub fn u8s(slots: &[u8], _nom: &str) -> Value {
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
         0 => Value::from_parts(Tag::Unit as u16, 0, v),
         1 => Value::from_parts(Tag::U8 as u16, 0, v),
         2 => Value::from_parts(Tag::U82 as u16, 0, v),
         3 => Value::from_parts(Tag::U83 as u16, 0, v),
         4 => Value::from_parts(Tag::U84 as u16, 0, v),
         5 => Value::from_parts(Tag::U85 as u16, 0, v),
         6 => Value::from_parts(Tag::U86 as u16, 0, v),
         7 => Value::from_parts(Tag::U87 as u16, 0, v),
         8 => Value::from_parts(Tag::U88 as u16, 0, v),
         9 => Value::from_parts(Tag::U89 as u16, 0, v),
         10 => Value::from_parts(Tag::U810 as u16, 0, v),
         11 => Value::from_parts(Tag::U811 as u16, 0, v),
         12 => Value::from_parts(Tag::U812 as u16, 0, v),
         _ => unreachable!(),
      }
   }
   pub fn i16(slot: i16, _nom: &str) -> Value {
      Value::from_parts(Tag::I16 as u16, 0, (slot as u16) as u128)
   }
   pub fn u16(slot: u16, _nom: &str) -> Value {
      Value::from_parts(Tag::U16 as u16, 0, (slot as u16) as u128)
   }
   pub fn i16s(slots: &[i16], _nom: &str) -> Value {
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
         0 => Value::from_parts(Tag::Unit as u16, 0, v),
         1 => Value::from_parts(Tag::I16 as u16, 0, v),
         2 => Value::from_parts(Tag::I162 as u16, 0, v),
         3 => Value::from_parts(Tag::I163 as u16, 0, v),
         4 => Value::from_parts(Tag::I164 as u16, 0, v),
         5 => Value::from_parts(Tag::I165 as u16, 0, v),
         6 => Value::from_parts(Tag::I166 as u16, 0, v),
         _ => unreachable!(),
      }
   }
   pub fn u16s(slots: &[u16], _nom: &str) -> Value {
      let mut v: u128 = 0;
      if slots.len()>=6  { v += slots[5]  as u128; } v <<= 16;
      if slots.len()>=5  { v += slots[4]  as u128; } v <<= 16;
      if slots.len()>=4  { v += slots[3]  as u128; } v <<= 16;
      if slots.len()>=3  { v += slots[2]  as u128; } v <<= 16;
      if slots.len()>=2  { v += slots[1]  as u128; } v <<= 16;
      if slots.len()>=1  { v += slots[0]  as u128; }
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, 0, v),
         1 => Value::from_parts(Tag::U16 as u16, 0, v),
         2 => Value::from_parts(Tag::U162 as u16, 0, v),
         3 => Value::from_parts(Tag::U163 as u16, 0, v),
         4 => Value::from_parts(Tag::U164 as u16, 0, v),
         5 => Value::from_parts(Tag::U165 as u16, 0, v),
         6 => Value::from_parts(Tag::U166 as u16, 0, v),
         _ => unreachable!(),
      }
   }
   pub fn i32(slot: i32, _nom: &str) -> Value {
      Value::from_parts(Tag::I32 as u16, 0, (slot as u32) as u128)
   }
   pub fn u32(slot: u32, _nom: &str) -> Value {
      Value::from_parts(Tag::U32 as u16, 0, (slot as u32) as u128)
   }
   pub fn i32s(slots: &[i32], _nom: &str) -> Value {
      let mut v: u128 = 0;
      unsafe {
         if slots.len()>=3  { v += std::mem::transmute::<i32,u32>(slots[2])  as u128; } v <<= 32;
         if slots.len()>=2  { v += std::mem::transmute::<i32,u32>(slots[1])  as u128; } v <<= 32;
         if slots.len()>=1  { v += std::mem::transmute::<i32,u32>(slots[0])  as u128; }
      }
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, 0, v),
         1 => Value::from_parts(Tag::I32 as u16, 0, v),
         2 => Value::from_parts(Tag::I322 as u16, 0, v),
         3 => Value::from_parts(Tag::I323 as u16, 0, v),
         _ => unreachable!(),
      }
   }
   pub fn u32s(slots: &[u32], _nom: &str) -> Value {
      let mut v: u128 = 0;
      if slots.len()>=3  { v += slots[2]  as u128; } v <<= 32;
      if slots.len()>=2  { v += slots[1]  as u128; } v <<= 32;
      if slots.len()>=1  { v += slots[0]  as u128; }
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, 0, v),
         1 => Value::from_parts(Tag::U32 as u16, 0, v),
         2 => Value::from_parts(Tag::U322 as u16, 0, v),
         3 => Value::from_parts(Tag::U323 as u16, 0, v),
         _ => unreachable!(),
      }
   }
   pub fn f32(slot: f32, _nom: &str) -> Value {
      let slot = unsafe { std::mem::transmute::<f32,u32>(slot) };
      Value::from_parts(Tag::F32 as u16, 0, slot as u128)
   }
   pub fn f32s(slots: &[f32], _nom: &str) -> Value {
      let mut v: u128 = 0;
      unsafe {
         if slots.len()>=3  { v += std::mem::transmute::<f32,u32>(slots[2])  as u128; } v <<= 32;
         if slots.len()>=2  { v += std::mem::transmute::<f32,u32>(slots[1])  as u128; } v <<= 32;
         if slots.len()>=1  { v += std::mem::transmute::<f32,u32>(slots[0])  as u128; }
      }
      match slots.len() {
         0 => Value::from_parts(Tag::Unit as u16, 0, v),
         1 => Value::from_parts(Tag::F32 as u16, 0, v),
         2 => Value::from_parts(Tag::F322 as u16, 0, v),
         3 => Value::from_parts(Tag::F323 as u16, 0, v),
         _ => unreachable!(),
      }
   }
   pub fn i64(slot: i64, _nom: &str) -> Value {
      dprintln!("Value::i64({})", slot);
      Value::from_parts(Tag::I64 as u16, 0, (slot as u64) as u128)
   }
   pub fn u64(slot: u64, _nom: &str) -> Value {
      dprintln!("Value::u64({})", slot);
      Value::from_parts(Tag::U64 as u16, 0, (slot as u64) as u128)
   }
   pub fn f64(slot: f64, _nom: &str) -> Value {
      let slot = unsafe { std::mem::transmute::<f64,u64>(slot) };
      Value::from_parts(Tag::F64 as u16, 0, slot as u128)
   }
   pub fn tag(&self) -> Tag {
      let t = (self.0 >> 112) as u16;
      FromPrimitive::from_i32(t.into()).expect(&format!("Invalid Tag in Value: {}", t))
   }
   pub fn tag_as_str(&self) -> String {
      dprintln!("Value::tag_as_str");
      format!("{:?}", self.tag())
   }
   pub fn name(&self) -> String {
      dprintln!("Value::name");
      "_".to_string()
   }
   pub fn slice(&self, start: usize, end: usize) -> Value {
      dprintln!("Value::slice({},{})",start,end);
      let tag = (self.0 >> 112) as u16;
      let nom = ((self.0 << 16) >> 112) as u16;
      let ptr_bits = (self.cptr() as usize) as u128;
      let start = start as u128;
      let end = end as u128;
      let mut raw: u128 = 0;
      raw |= start; raw <<= 16;
      raw |= end;   raw <<= 64;
      raw |= ptr_bits;
      Value::from_parts(tag, nom, raw)
   }
   pub fn literal(&self) -> String {
      let start = self.start();
      let end = self.end();
      let cptr = self.cptr();
      let mut val = Vec::new();
      for po in start..end {
      unsafe {
         val.push( char::from_u32_unchecked(*cptr.offset(po as isize)) );
      }}
      String::from_iter(val)
   }
   pub fn vslot(&self, slot: usize) -> Value {
      assert!(self.tag() == Tag::Tuple, "vslot must be `Tuple");
      let tag = self.tag();
      match tag {
         Tag::Tuple => {
            assert!(slot >= self.start(), ".vslot({}) out of bounds", slot);
            assert!(slot < self.end(), ".vslot({}) out of bounds", slot);
            let ptr = self.tptr();
            unsafe {
               Value( *ptr.offset(slot as isize) )
            }
         },
         _ => { panic!("Could not cast {:?} as Tuple",tag) },         
      }
   }
   pub fn push(&self, x: Value) {
      assert!(self.tag() == Tag::Tuple, "push must be `Tuple");
      dprintln!("Value::push");
      for i in self.start()..self.end() {
      if self.vslot(i).0 == 0 {
         let ptr = self.tptr();
         unsafe {
            *ptr.offset(i as isize) = x.0;
         }
         break;
      }}
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
