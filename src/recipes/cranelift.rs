use crate::ast::{FunctionDefinition};
use cranelift::prelude::*;

pub fn import<'f>() -> Vec<(Vec<types::Type>,FunctionDefinition<()>,fn(&mut FunctionBuilder<'f>,Vec<Value>) -> Value,types::Type)> {
   let mut imported = Vec::new();
   imported.push(crate::recipes::cranelift_impl::add::import());
   imported.push(crate::recipes::cranelift_impl::sub::import());
   imported.push(crate::recipes::cranelift_impl::eq::import());
   imported.push(crate::recipes::cranelift_impl::ne::import());
   imported.push(crate::recipes::cranelift_impl::lt::import());
   imported.push(crate::recipes::cranelift_impl::lte::import());
   imported.push(crate::recipes::cranelift_impl::gt::import());
   imported.push(crate::recipes::cranelift_impl::gte::import());
   imported.push(crate::recipes::cranelift_impl::mul::import());
   imported
}
