use crate::ast::{FunctionDefinition};
use cranelift::prelude::*;

pub struct FFI {
   pub args: Vec<types::Type>,
   pub fdef: FunctionDefinition<()>,
   pub cons: for<'f> fn(&mut FunctionBuilder<'f>,&[Value]) -> Value,
   pub rname: String,
   pub rtype: types::Type,
}

pub fn import<'f>() -> Vec<FFI> {
   let mut imported = Vec::new();
   imported.extend(crate::recipes::cranelift_impl::add::import());
   imported.extend(crate::recipes::cranelift_impl::sub::import());
   /*
   imported.extend(crate::recipes::cranelift_impl::eq::import());
   imported.extend(crate::recipes::cranelift_impl::ne::import());
   imported.extend(crate::recipes::cranelift_impl::lt::import());
   imported.extend(crate::recipes::cranelift_impl::lte::import());
   imported.extend(crate::recipes::cranelift_impl::gt::import());
   imported.extend(crate::recipes::cranelift_impl::gte::import());
   imported.extend(crate::recipes::cranelift_impl::mul::import());
   imported.extend(crate::recipes::cranelift_impl::div::import());
   imported.extend(crate::recipes::cranelift_impl::rem::import());
   */
   imported
}
