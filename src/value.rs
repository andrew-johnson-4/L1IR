
pub enum Tag {
   Unit,
   I8, I82, I83, I84, I85, I86, I87, I89, I810, I811, I812,
   U8, U82, U83, U84, U85, U86, U87, U89, U810, U811, U812,
}

pub struct Value(u128);

impl Value {
   pub fn nil(nom: &str) -> Value {
      Value(0)
   }
   pub fn i8(slot: i8, nom: &str) -> Value {
      Value(0)
   }
   pub fn u8(slot: u8, nom: &str) -> Value {
      Value(0)
   }
   pub fn i8s(slots: &[i8], nom: &str) -> Value {
      Value(0)
   }
   pub fn u8s(slot: &[u8], nom: &str) -> Value {
      Value(0)
   }
   pub fn tag(&self) -> &str {
      "Tag#123"
   }
   pub fn name(&self) -> &str {
      "Type#123"
   }
   pub fn slot(&self, slot: usize) -> i128 {
      0
   }
}
