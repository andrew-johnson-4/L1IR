use crate::ast::{Type};
use crate::value;
use crate::recipes::cranelift::FFI;
use cranelift::prelude::*;
use std::collections::HashMap;
use cranelift_codegen::ir::FuncRef;

pub fn s_u64(t_lo: u64, t_hi: u64) -> (u64,u64) {
   let t = value::Value::from_lohi(t_lo,t_hi);
   let mut r_len = 0;
   for ti in t.start()..t.end() {
      let s = t.vslot(ti);
      assert!( s.tag() == value::Tag::String );
      r_len += s.end() - s.start();
   }
   let r = value::Value::string_with_capacity(r_len as u64);
   for ti in t.start()..t.end() {
      let s = t.vslot(ti);
      for si in s.start()..s.end() {
         r.pushc(s.cslot(si));
      }
   }
   r.lohi()
}

pub fn f_u64<'f>(frefs: &HashMap<String,FuncRef>, ctx: &mut FunctionBuilder<'f>, val: &[Value]) -> Value {
   let arg0 = val[0].clone();
   let arg1 = val[1].clone();
   let fref = frefs.get(".join(String[])->String").unwrap();
   let call = ctx.ins().call(*fref, &[arg0,arg1]);
   let r_lo = ctx.inst_results(call)[0];
   let r_hi = ctx.inst_results(call)[1];
   ctx.ins().iconcat(r_lo,r_hi)
}

pub fn import() -> Vec<FFI> {vec![
   FFI {
      args: vec![types::I128],
      arg_types: vec![Type::nominal("Tuple")],
      name: ".join(String[])->String".to_string(),
      cons: f_u64,
      symbol: Some(s_u64 as *const u8),
      rname: "String".to_string(),
      rtype: types::I128,
   }
]}
