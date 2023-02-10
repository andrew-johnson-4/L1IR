use crate::ast;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

pub struct FFI {
   pub args: Vec<types::Type>,
   pub arg_types: Vec<ast::Type>,
   pub name: String,
   pub cons: for<'f> fn(&HashMap<String,FuncRef>, &mut FunctionBuilder<'f>,&[Value]) -> Value,
   pub symbol: Option<*const u8>,
   pub rname: String,
   pub rtype: types::Type,
}
unsafe impl Send for FFI {}

pub fn import<'f>() -> Vec<FFI> {
   let mut imported = Vec::new();
   imported.extend(crate::recipes::cranelift_impl::add::import());
   imported.extend(crate::recipes::cranelift_impl::sub::import());
   imported.extend(crate::recipes::cranelift_impl::eq::import());
   imported.extend(crate::recipes::cranelift_impl::ne::import());
   imported.extend(crate::recipes::cranelift_impl::lt::import());
   imported.extend(crate::recipes::cranelift_impl::lte::import());
   imported.extend(crate::recipes::cranelift_impl::gt::import());
   imported.extend(crate::recipes::cranelift_impl::gte::import());
   imported.extend(crate::recipes::cranelift_impl::mul::import());
   imported.extend(crate::recipes::cranelift_impl::div::import());
   imported.extend(crate::recipes::cranelift_impl::rem::import());

   imported.extend(crate::recipes::cranelift_impl::range1::import());
   imported.extend(crate::recipes::cranelift_impl::range2::import());
   imported.extend(crate::recipes::cranelift_impl::range3::import());

   imported.extend(crate::recipes::cranelift_impl::tuple_length::import());
   imported
}
